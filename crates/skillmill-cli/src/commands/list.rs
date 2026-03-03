use clap::{Parser, Subcommand};

use crate::commands::registry::load_registry;

#[derive(Parser, Debug)]
pub struct ListArgs {
    #[command(subcommand)]
    pub command: ListCommands,
}

#[derive(Subcommand, Debug)]
pub enum ListCommands {
    Disciplines,
    Nodes {
        #[arg(long)]
        discipline: String,
        #[arg(long)]
        level: Option<String>,
    },
}

pub fn run(args: ListArgs) -> anyhow::Result<()> {
    let registry = load_registry()?;
    match args.command {
        ListCommands::Disciplines => {
            for id in registry.ids() {
                let plugin = registry.get(id).expect("plugin exists");
                println!("{} — {}", id, plugin.name());
            }
        }
        ListCommands::Nodes { discipline, level } => {
            let plugin = registry
                .get(&discipline)
                .ok_or_else(|| anyhow::anyhow!("unknown discipline: {}", discipline))?;
            let mut nodes: Vec<_> = plugin.curriculum().nodes.values().collect();
            if let Some(level) = level {
                nodes = nodes
                    .into_iter()
                    .filter(|n| n.level.0.eq_ignore_ascii_case(&level))
                    .collect();
            }
            nodes.sort_by(|a, b| a.id.0.cmp(&b.id.0));
            for node in nodes {
                println!("{} — {}", node.id.0, node.label);
            }
        }
    }
    Ok(())
}
