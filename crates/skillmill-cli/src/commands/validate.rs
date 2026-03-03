use clap::Parser;
use rand::SeedableRng;
use rand::rngs::StdRng;
use skillmill_core::schema::{DifficultyAxes, SchemaId};

use crate::commands::registry::load_registry;

#[derive(Parser, Debug)]
pub struct ValidateArgs {
    #[arg(long)]
    pub discipline: String,
    #[arg(long)]
    pub schema: Option<String>,
    #[arg(long, default_value_t = 1000)]
    pub count: u32,
    #[arg(long)]
    pub seed: Option<u64>,
}

pub fn run(args: ValidateArgs) -> anyhow::Result<()> {
    let registry = load_registry()?;
    let plugin = registry
        .get(&args.discipline)
        .ok_or_else(|| anyhow::anyhow!("unknown discipline: {}", args.discipline))?;

    let mut rng: StdRng = match args.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_rng(&mut rand::rng()),
    };

    let mut failures = 0u32;
    let schemas: Vec<String> = if let Some(schema) = args.schema {
        vec![schema]
    } else {
        plugin
            .curriculum()
            .nodes
            .values()
            .flat_map(|n| n.schemas.clone())
            .collect()
    };

    for schema_id in schemas {
        for _ in 0..args.count {
            let item = plugin.execute_schema(&SchemaId(schema_id.clone()), &mut rng, &DifficultyAxes::default())?;
            let result = plugin.validate_answer(&item);
            if !result.ok {
                failures += 1;
                println!("✗ {}: {}", schema_id, result.message.unwrap_or_else(|| "unknown".to_string()));
                break;
            }
        }
    }

    if failures > 0 {
        return Err(anyhow::anyhow!("validation failed: {} schemas", failures));
    }

    println!("✓ Validation passed");
    Ok(())
}
