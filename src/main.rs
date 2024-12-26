mod error;
mod config;

use gix;
use clap::builder::styling;
use clap::{Parser, Subcommand};
use error::{Error, ErrorKind};
use owo_colors::OwoColorize;
use std::io::Write;
use std::{fs, path};
use std::path::PathBuf;

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Cyan.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(styles = STYLES)]
struct Cli {
    #[command(subcommand)]
    commands: CommandsArgs,
}

#[derive(Subcommand, Debug)]
enum CommandsArgs {
    Init {
        path: PathBuf,

        #[arg(long)]
        no_git: bool,
        #[arg(short, long)]
        force: bool,
    },
    Build 
}

fn create_new2<P: AsRef<std::path::Path>> (path: P, overwrite: bool) -> std::io::Result<fs::File> {
    match fs::File::create_new(path.as_ref()) {
        Ok(fd) => Ok(fd),
        Err(e) => {
            if overwrite {
                fs::File::create(path.as_ref())
            } else {
                return Err(e);
            }
        },
    }
}

fn initialize_project(path: &PathBuf, with_git: bool, overwrite: bool) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = path::absolute(path).unwrap();

    if path.is_file() {
        return Err(Box::new(Error::new(ErrorKind::IsNotADirectory)));
    }
    
    if !path.exists() {
        fs::create_dir(&path)?;
    } else if path.join("xylo.toml").exists() && !overwrite {
        return Err(Box::new(Error::new(ErrorKind::AlreadyExists)));
    }

    for d in ["src", "target", "include"] {
        match fs::create_dir(path.join(d)) {
            Ok(r) => r,
            Err(_e) => {
                if overwrite {
                    continue;
                }
            }
        }
    }

    let config_file = path.join("xylo.toml");

    let mut config_toml = create_new2(&config_file, overwrite)?;
    let mut makefile = create_new2(path.join("makefile"), overwrite)?;
    let mut clang_format = create_new2(path.join(".clang-format"), overwrite)?;
    
    if with_git {
        if path.join(".git").exists() && overwrite {
            fs::remove_dir_all(path.join(".git"))?;
        }
        gix::init(&path)?;

        let mut gitignore = create_new2(path.join(".gitignore"), overwrite)?;

        gitignore.write_all(".build/\ncompile_commands.json\n.clang-format".as_bytes())?;
    }

    let new_config = config::ConfigManager::new(
        config::Config {
            package: config::ConfigPackage {
                name: path.file_name().unwrap().to_string_lossy().to_string()
            },
            build: config::ConfigBuild { 
                compiler: "clang".to_string(),
                files: "src/main.c".to_string(),
                args: "-Iinclude -o target/main.exe".to_string()
            } 
        },
    ).to_string()?;

    config_toml.write_all(new_config.as_bytes())?;

    Ok(())
}

fn main() {
    let args = Cli::parse();

    match &args.commands {
        CommandsArgs::Init { path, no_git, force} => {
            match initialize_project(path, !no_git, *force) {
                Ok(_) => println!("Project {} was successfully created", path.to_str().unwrap().bright_green().bold()),
                Err(e) => println!("{} while creating the project: {}", "Error".bright_red().bold(), (*e).to_string()),
            }
                    }
        CommandsArgs::Build => {
        }
    }
}
