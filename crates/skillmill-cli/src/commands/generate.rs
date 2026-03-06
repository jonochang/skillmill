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
    let result = pipeline.render(
        &spec,
        &args.out,
        !args.no_answer_key && policy.include_answer_key,
    )?;

    let slug = slugify(&spec.profile.name);
    let base = format!("{}-{}", slug, policy.target_node);
    let student_target = args.out.join(format!("{}-student.pdf", base));
    let answer_target = args.out.join(format!("{}-answer-key.pdf", base));

    std::fs::rename(&result.student_pdf, &student_target)?;
    let student_png_targets = rename_png_pages(&args.out, &base, "student", result.student_png)?;

    let answer_paths = if let (Some(answer_pdf), Some(answer_pngs)) =
        (result.answer_key_pdf, result.answer_key_png)
    {
        std::fs::rename(&answer_pdf, &answer_target)?;
        let answer_png_targets = rename_png_pages(&args.out, &base, "answer-key", answer_pngs)?;
        Some((answer_target, answer_png_targets))
    } else {
        None
    };

    println!("✓ Generated");
    println!("  {}", student_target.display());
    for path in &student_png_targets {
        println!("  {}", path.display());
    }
    if let Some((answer_pdf, answer_pngs)) = answer_paths {
        println!("  {}", answer_pdf.display());
        for path in answer_pngs {
            println!("  {}", path.display());
        }
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

fn rename_png_pages(
    out_dir: &std::path::Path,
    base: &str,
    suffix: &str,
    sources: Vec<std::path::PathBuf>,
) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let multi = sources.len() > 1;
    let mut targets = Vec::with_capacity(sources.len());

    for (idx, source) in sources.into_iter().enumerate() {
        let target = if multi {
            out_dir.join(format!("{}-{}-p{:02}.png", base, suffix, idx + 1))
        } else {
            out_dir.join(format!("{}-{}.png", base, suffix))
        };
        std::fs::rename(source, &target)?;
        targets.push(target);
    }

    Ok(targets)
}
