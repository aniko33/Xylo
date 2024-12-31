use std::path::PathBuf;
use clap::builder::styling;
use clap::Parser;

#[macro_export]
macro_rules! errorln {
    () => {
        println!("\n");
    };
    ($($arg:tt)*) => {{
        print!("{}: ", "ERROR".bright_red().bold());
        println!($($arg)*);
        std::process::exit(1);
    }}
}

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Cyan.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(styles = STYLES)]
pub struct Cli {
    pub path: PathBuf,

    #[arg(long)]
    pub no_git: bool,
    // #[arg(short, long)]
    // pub remote_repo: Option<String>,
    #[arg(short, long)]
    pub force: bool,
    #[arg(short, long)]
    pub target: Option<String>
}
