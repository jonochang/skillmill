use clap::{Parser, Subcommand};

mod commands;
mod ui;

#[derive(Parser)]
#[command(name = "skillmill")]
#[command(about = "SkillMill CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[command(subcommand)]
        command: InitCommands,
    },
    Generate(commands::generate::GenerateArgs),
    Preview(commands::preview::PreviewArgs),
    List(commands::list::ListArgs),
    Validate(commands::validate::ValidateArgs),
}

#[derive(Subcommand)]
enum InitCommands {
    Profile(commands::init::profile::InitProfileArgs),
    Policy(commands::init::policy::InitPolicyArgs),
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Init { command } => match command {
            InitCommands::Profile(args) => commands::init::profile::run(args),
            InitCommands::Policy(args) => commands::init::policy::run(args),
        },
        Commands::Generate(args) => commands::generate::run(args),
        Commands::Preview(args) => commands::preview::run(args),
        Commands::List(args) => commands::list::run(args),
        Commands::Validate(args) => commands::validate::run(args),
    }
}
