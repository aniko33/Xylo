use std::path::PathBuf;
use clap::builder::styling;
use clap::{Parser, Subcommand};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Cyan.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(styles = STYLES)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: CommandsArgs,
}

#[derive(Subcommand, Debug)]
pub enum CommandsArgs {
    Init {
        path: PathBuf,

        #[arg(long)]
        no_git: bool,
        #[arg(short, long)]
        force: bool,
        #[arg(short, long)]
        target: Option<String>
    },
    Build 
}
