use clap::ValueEnum;

use crate::cli;

#[derive(Debug)]
pub struct Config {
    pub api_key: String,

    pub format: FormatConfig,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FormatConfig {
    Debug,
    JSON,
    Table,
}

pub fn from_cli(cli: &cli::Cli) -> Config {
    Config {
        api_key: cli.api_key.clone(),
        format: cli.format,
    }
}
