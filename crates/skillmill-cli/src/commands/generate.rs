use clap::Parser;
use rand::SeedableRng;
use rand::rngs::StdRng;
use skillmill_core::compose::Composer;
use skillmill_core::render::RenderPipeline;
use skillmill_core::{StudentProfile, WorksheetPolicy};

use crate::commands::registry::load_registry;

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    #[arg(long)]
    pub policy: std::path::PathBuf,
    #[arg(long)]
    pub profile: Option<std::path::PathBuf>,
    #[arg(long, default_value = "./out")]
    pub out: std::path::PathBuf,
    #[arg(long)]
    pub no_answer_key: bool,
    #[arg(long)]
    pub seed: Option<u64>,
}

pub fn run(args: GenerateArgs) -> anyhow::Result<()> {
    let registry = load_registry()?;
    let policy = WorksheetPolicy::load_from_file(&args.policy)?;
    let profile = if let Some(profile_path) = &args.profile {
        StudentProfile::load_from_file(profile_path)?
    } else {
        StudentProfile {
            name: "Student".to_string(),
            discipline: policy.discipline.clone(),
            current_node: policy.target_node.clone(),
            mastery: std::collections::HashMap::new(),
            customisations: skillmill_core::WorksheetCustomisation {
                header: skillmill_core::profile::WorksheetHeader {
                    school: None,
                    class: None,
                    date: "auto".to_string(),
                },
                layout: skillmill_core::profile::WorksheetLayout {
                    font_size: 12,
                    working_space: "medium".to_string(),
                },
            },
        }
    };

    let plugin = registry
        .get(&policy.discipline)
        .ok_or_else(|| anyhow::anyhow!("unknown discipline: {}", policy.discipline))?;

    let mut rng: StdRng = match args.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_rng(&mut rand::rng()),
    };

    let spec = Composer::compose(plugin, policy.clone(), profile, &mut rng)?;
    let pipeline = RenderPipeline::new(plugin.template_dir());
    let result =
        pipeline.render(&spec, &args.out, !args.no_answer_key && policy.include_answer_key)?;

    let slug = slugify(&spec.profile.name);
    let base = format!("{}-{}", slug, policy.target_node);
    let student_target = args.out.join(format!("{}-student.pdf", base));
    let answer_target = args.out.join(format!("{}-answer-key.pdf", base));

    std::fs::rename(&result.student_pdf, &student_target)?;
    let answer_path = if let Some(answer) = result.answer_key_pdf {
        std::fs::rename(&answer, &answer_target)?;
        Some(answer_target)
    } else {
        None
    };

    println!("✓ Generated");
    println!("  {}", student_target.display());
    if let Some(answer) = answer_path {
        println!("  {}", answer.display());
    }

    Ok(())
}

fn slugify(input: &str) -> String {
    let mut slug = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if ch.is_whitespace() || ch == '-' {
            if !slug.ends_with('-') {
                slug.push('-');
            }
        }
    }
    slug.trim_matches('-').to_string()
}
