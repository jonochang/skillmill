use clap::Parser;
use rand::SeedableRng;
use rand::rngs::StdRng;
use skillmill_core::schema::{DifficultyAxes, SchemaId};

use crate::commands::registry::load_registry;

#[derive(Parser, Debug)]
pub struct PreviewArgs {
    #[arg(long)]
    pub discipline: String,
    #[arg(long)]
    pub node: String,
    #[arg(long, default_value_t = 5)]
    pub count: u32,
    #[arg(long)]
    pub seed: Option<u64>,
}

pub fn run(args: PreviewArgs) -> anyhow::Result<()> {
    let registry = load_registry()?;
    let plugin = registry
        .get(&args.discipline)
        .ok_or_else(|| anyhow::anyhow!("unknown discipline: {}", args.discipline))?;

    let node = plugin
        .curriculum()
        .nodes
        .get(&skillmill_core::NodeId(args.node.clone()))
        .ok_or_else(|| anyhow::anyhow!("unknown node: {}", args.node))?;

    let mut rng: StdRng = match args.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_rng(&mut rand::rng()),
    };

    for idx in 0..args.count {
        let schema_id = node
            .schemas
            .get((idx as usize) % node.schemas.len())
            .ok_or_else(|| anyhow::anyhow!("node has no schemas"))?;
        let item = plugin.execute_schema(&SchemaId(schema_id.clone()), &mut rng, &DifficultyAxes::default())?;
        println!("{}.", idx + 1);
        println!("Q: {}", item.question.0);
        println!("A: {}", item.answer.0);
        if let Some(working) = item.working {
            println!("W: {}", working.0);
        }
        println!();
    }

    Ok(())
}
