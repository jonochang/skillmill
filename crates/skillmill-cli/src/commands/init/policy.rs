use clap::Parser;
use skillmill_core::policy::{Band, BandSource, CustomSection, WorksheetPolicy};
use skillmill_core::profile::StudentProfile;

use crate::commands::registry::load_registry;
use crate::ui::prompt;

#[derive(Parser, Debug)]
pub struct InitPolicyArgs {
    #[arg(long)]
    pub out: Option<std::path::PathBuf>,
    #[arg(long)]
    pub profile: Option<std::path::PathBuf>,
}

pub fn run(args: InitPolicyArgs) -> anyhow::Result<()> {
    let registry = load_registry()?;
    let policy = loop {
        let (discipline, target_node) = if let Some(profile_path) = args.profile.as_ref() {
            let profile = StudentProfile::load_from_file(profile_path)?;
            (profile.discipline, profile.current_node)
        } else {
            let disciplines: Vec<String> = registry.ids().map(|s| s.to_string()).collect();
            let discipline_idx = prompt::select("Discipline", &disciplines)?;
            let discipline = disciplines[discipline_idx].clone();

            let plugin = registry.get(&discipline).expect("discipline exists");
            let mut nodes: Vec<_> = plugin
                .curriculum()
                .nodes
                .values()
                .map(|n| format!("{} — {}", n.id.0, n.label))
                .collect();
            nodes.sort();
            let node_idx = prompt::select("Target node", &nodes)?;
            let target_node = nodes[node_idx]
                .split(' ')
                .next()
                .unwrap_or("unknown")
                .to_string();
            (discipline, target_node)
        };

        let item_count = prompt::input_default("Total questions", "20")?;
        let item_count: u32 = item_count.parse().unwrap_or(20);
        let include_answer_key = prompt::confirm("Include answer key?", true)?;
        let include_workings = prompt::confirm("Include step-by-step workings?", false)?;

        let plugin = registry.get(&discipline).expect("discipline exists");
        let mut composition = plugin.default_composition();
        if prompt::confirm("Customise composition mix?", false)? {
            let target = prompt::input_default("Target weight (0-1)", "0.70")?;
            let review = prompt::input_default("Prerequisite weight (0-1)", "0.20")?;
            let non_routine = prompt::input_default("Non-routine weight (0-1)", "0.10")?;
            composition = vec![
                Band {
                    source: BandSource::TargetNode,
                    weight: target.parse().unwrap_or(0.70),
                    item_types: vec!["drill".into()],
                },
                Band {
                    source: BandSource::Prerequisites,
                    weight: review.parse().unwrap_or(0.20),
                    item_types: vec!["drill".into()],
                },
                Band {
                    source: BandSource::NonRoutine,
                    weight: non_routine.parse().unwrap_or(0.10),
                    item_types: vec!["drill".into()],
                },
            ];
        }

        let mut custom_sections = Vec::new();
        if prompt::confirm("Add custom sections?", false)? {
            loop {
                let options = vec![
                    "worked_example".to_string(),
                    "free_text".to_string(),
                    "page_break".to_string(),
                    "done".to_string(),
                ];
                let idx = prompt::select("Section type", &options)?;
                let choice = &options[idx];
                if choice == "done" {
                    break;
                }
                let position =
                    prompt::input_default("Position (e.g., before_item 5)", "before_item 1")?;
                let content = if choice == "page_break" {
                    "".to_string()
                } else {
                    prompt::input("Content")?
                };
                custom_sections.push(CustomSection {
                    position,
                    r#type: choice.to_string(),
                    content,
                });
                if !prompt::confirm("Add another section?", false)? {
                    break;
                }
            }
        }

        let policy = WorksheetPolicy {
            discipline,
            target_node,
            composition,
            item_count,
            include_answer_key,
            include_workings,
            custom_sections,
        };

        println!("\nSummary");
        println!("  Discipline: {}", policy.discipline);
        println!("  Target node: {}", policy.target_node);
        println!("  Questions: {}", policy.item_count);
        println!(
            "  Answer key: {}  Workings: {}",
            if policy.include_answer_key {
                "yes"
            } else {
                "no"
            },
            if policy.include_workings { "yes" } else { "no" }
        );
        println!("  Custom sections: {}", policy.custom_sections.len());

        if prompt::confirm("Confirm policy?", true)? {
            break policy;
        }
    };

    let out_path = args
        .out
        .unwrap_or_else(|| std::path::PathBuf::from("policies/policy.yaml"));
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    write_policy_with_header(&policy, &out_path)?;
    println!("✓ Created {}", out_path.display());
    println!("  Next: skillmill generate --policy {}", out_path.display());
    Ok(())
}

fn write_policy_with_header(
    policy: &WorksheetPolicy,
    path: &std::path::Path,
) -> anyhow::Result<()> {
    let mut yaml = serde_yaml::to_string(policy)?;
    yaml = yaml.replace("item_count:", "item_count:  # total questions");
    yaml = yaml.replace(
        "include_answer_key:",
        "include_answer_key:  # generate answer key PDF",
    );
    yaml = yaml.replace(
        "include_workings:",
        "include_workings:  # include step-by-step workings",
    );
    let mut output = String::new();
    output.push_str("# Generated by: skillmill init policy\n");
    output.push_str(&yaml);
    std::fs::write(path, output)?;
    Ok(())
}
