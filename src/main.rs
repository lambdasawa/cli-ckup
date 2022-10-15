use clap::Parser;
use std::error::Error;

mod cli;
mod client;
mod config;
mod handler;
mod model;
mod print;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();

    let cfg = config::from_cli(&cli);

    match &cli.commands {
        cli::Commands::User(args) => match args.commands {
            cli::UserCommands::View {} => handler::user_view(&cfg),
        },

        cli::Commands::Workspace(args) => match args.commands {
            cli::WorkspaceCommands::View {} => handler::workspace_view(&cfg),
        },

        cli::Commands::Space(args) => match &args.commands {
            cli::SpaceCommands::View(args) => handler::space_view(&cfg, args),
        },

        cli::Commands::List(args) => match &args.commands {
            cli::ListCommands::View(args) => handler::list_view(&cfg, args),
        },

        cli::Commands::Task(args) => match &args.commands {
            cli::TaskCommands::View(args) => handler::task_view(&cfg, args),
        },
    }

    Ok(())
}
