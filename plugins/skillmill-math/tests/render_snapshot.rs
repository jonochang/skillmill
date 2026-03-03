use skillmill_core::compose::{WorksheetSection, WorksheetSpec};
use skillmill_core::render::RenderPipeline;
use skillmill_core::schema::{GeneratedItem, RenderedAnswer, RenderedQuestion, SchemaId};
use skillmill_core::policy::{Band, BandSource, CustomSection, WorksheetPolicy};
use skillmill_core::profile::{StudentProfile, WorksheetCustomisation, WorksheetHeader, WorksheetLayout};

#[test]
fn render_snapshot_json() {
    let profile = StudentProfile {
        name: "Snapshot Student".to_string(),
        discipline: "math-singapore".to_string(),
        current_node: "p2-add-sub-within-100".to_string(),
        mastery: std::collections::HashMap::new(),
        customisations: WorksheetCustomisation {
            header: WorksheetHeader {
                school: Some("Test School".to_string()),
                class: Some("2A".to_string()),
                date: "2026-03-03".to_string(),
            },
            layout: WorksheetLayout {
                font_size: 12,
                working_space: "medium".to_string(),
            },
        },
    };

    let policy = WorksheetPolicy {
        discipline: "math-singapore".to_string(),
        target_node: "p2-add-sub-within-100".to_string(),
        composition: vec![
            Band { source: BandSource::TargetNode, weight: 1.0, item_types: vec!["drill".into()] },
        ],
        item_count: 2,
        include_answer_key: true,
        include_workings: false,
        custom_sections: vec![CustomSection {
            position: "before_item 2".to_string(),
            r#type: "free_text".to_string(),
            content: "Nice work!".to_string(),
        }],
    };

    let item1 = GeneratedItem {
        node_id: "p2-add-sub-within-100".to_string(),
        schema_id: SchemaId("p2-add-sub-within-100-horizontal".to_string()),
        question: RenderedQuestion("12 + 8 = ___".to_string()),
        answer: RenderedAnswer("20".to_string()),
        working: None,
        visuals: vec![],
    };
    let item2 = GeneratedItem {
        node_id: "p2-add-sub-within-100".to_string(),
        schema_id: SchemaId("p2-add-sub-within-100-vertical".to_string()),
        question: RenderedQuestion("45\n- 7\n= ___".to_string()),
        answer: RenderedAnswer("38".to_string()),
        working: None,
        visuals: vec![],
    };

    let sections = vec![
        WorksheetSection::Item { number: 1, item: item1.clone() },
        WorksheetSection::Custom {
            position: "before_item 2".to_string(),
            kind: "free_text".to_string(),
            content: "Nice work!".to_string(),
        },
        WorksheetSection::Item { number: 2, item: item2.clone() },
    ];

    let spec = WorksheetSpec {
        profile,
        policy: policy.clone(),
        items: vec![item1, item2],
        sections,
    };

    let json = serde_json::to_string_pretty(&spec).expect("json");
    insta::assert_snapshot!("worksheet_json", json);

    if which::which("typst").is_ok() {
        let plugin = skillmill_math::MathPlugin::new().expect("plugin init");
        let out_dir = tempfile::tempdir().expect("tempdir");
        let pipeline = RenderPipeline::new(plugin.template_dir());
        let result = pipeline.render(&spec, out_dir.path(), true).expect("render");
        let student_size = std::fs::metadata(&result.student_pdf).unwrap().len();
        assert!(student_size > 0, "student pdf empty");
    }
}
