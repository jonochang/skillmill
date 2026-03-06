use criterion::{Criterion, criterion_group, criterion_main};
use rand::SeedableRng;
use rand::rngs::StdRng;
use skillmill_core::compose::Composer;
use skillmill_core::plugin::DisciplinePlugin;
use skillmill_core::policy::{Band, BandSource, WorksheetPolicy};
use skillmill_core::profile::{
    StudentProfile, WorksheetCustomisation, WorksheetHeader, WorksheetLayout,
};
use skillmill_core::render::RenderPipeline;

fn render_bench(c: &mut Criterion) {
    let plugin = match skillmill_math::MathPlugin::new() {
        Ok(plugin) => plugin,
        Err(_) => return,
    };

    if which::which("typst").is_err() {
        return;
    }

    let profile = StudentProfile {
        name: "Benchmark Student".to_string(),
        discipline: plugin.id().to_string(),
        current_node: "p3-add-sub-within-10000".to_string(),
        mastery: std::collections::HashMap::new(),
        customisations: WorksheetCustomisation {
            header: WorksheetHeader {
                school: None,
                class: None,
                date: "auto".to_string(),
            },
            layout: WorksheetLayout {
                font_size: 12,
                working_space: "medium".to_string(),
            },
        },
    };

    let policy = WorksheetPolicy {
        discipline: plugin.id().to_string(),
        target_node: "p3-add-sub-within-10000".to_string(),
        composition: vec![
            Band {
                source: BandSource::TargetNode,
                weight: 0.7,
                item_types: vec!["drill".into()],
            },
            Band {
                source: BandSource::Prerequisites,
                weight: 0.2,
                item_types: vec!["drill".into()],
            },
            Band {
                source: BandSource::NonRoutine,
                weight: 0.1,
                item_types: vec!["drill".into()],
            },
        ],
        item_count: 20,
        include_answer_key: true,
        include_workings: false,
        custom_sections: vec![],
    };

    c.bench_function("render_20_item_math", |b| {
        b.iter(|| {
            let mut rng = StdRng::seed_from_u64(42);
            let spec =
                Composer::compose(&plugin, policy.clone(), profile.clone(), &mut rng).unwrap();
            let out_dir = tempfile::tempdir().unwrap();
            let pipeline = RenderPipeline::new(plugin.template_dir());
            let _ = pipeline.render(&spec, out_dir.path(), true).unwrap();
        })
    });
}

criterion_group!(benches, render_bench);
criterion_main!(benches);
