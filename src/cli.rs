use clap::{command, Args, Parser, Subcommand};

use crate::config;

#[derive(Debug, Parser)]
#[command(
  name = "ckup",
  author = "Tsubasa Irisawa <lambdasawa@gmail.com>",
  version = "0.1.0",
  about = "CLI for clickup.com",
  long_about = None,
  propagate_version = true
)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "API_KEY",
        help = "https://app.clickup.com/settings/apps",
        env = "CLICKUP_API_KEY",
        global = true,
        required = false
    )]
    pub api_key: String,

    #[arg(
        short,
        long,
        value_enum,
        default_value_t = config::FormatConfig::Table,
        env = "CLICKUP_FORMAT",
        global = true,
        required = false,
    )]
    pub format: config::FormatConfig,

    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    User(UserArgs),
    Workspace(WorkspaceArgs),
    Space(SpaceArgs),
    List(ListArgs),
    Task(TaskArgs),
}

#[derive(Debug, Args)]
pub struct UserArgs {
    #[command(subcommand)]
    pub commands: UserCommands,
}

#[derive(Debug, Subcommand)]
pub enum UserCommands {
    View {},
}

#[derive(Debug, Args)]
pub struct WorkspaceArgs {
    #[command(subcommand)]
    pub commands: WorkspaceCommands,
}

#[derive(Debug, Subcommand)]
pub enum WorkspaceCommands {
    View {},
}

#[derive(Debug, Args)]
pub struct SpaceArgs {
    #[command(subcommand)]
    pub commands: SpaceCommands,
}

#[derive(Debug, Subcommand)]
pub enum SpaceCommands {
    View(SpaceViewArgs),
}

#[derive(Debug, Args)]
pub struct SpaceViewArgs {
    #[arg(short = 'i', long)]
    pub workspace_id: u32,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[command(subcommand)]
    pub commands: ListCommands,
}

#[derive(Debug, Subcommand)]
pub enum ListCommands {
    View(ListViewArgs),
}

#[derive(Debug, Args)]
pub struct ListViewArgs {
    #[arg(short = 'i', long)]
    pub space_id: u32,
}

#[derive(Debug, Args)]
pub struct TaskArgs {
    #[command(subcommand)]
    pub commands: TaskCommands,
}

#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    View(TaskViewArgs),
}

#[derive(Debug, Args)]
pub struct TaskViewArgs {
    #[arg(short = 'i', long)]
    pub list_id: u32,
}
