mod error;
mod config;
mod cli;

use clap::Parser;
use gix;
use error::{Error, ErrorKind};
use owo_colors::OwoColorize;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::io::Write;
use std::{fs, path};

const CONFIG_FILENAME: &str = "xylo.toml";
const MAIN_TEMPLATE: &str = r#"#include <stdio.h>

int main() {
    printf("Hello Xylo!");
    return 0;
}"#;

fn create_new2<P: AsRef<std::path::Path>> (path: P, overwrite: bool) -> std::io::Result<fs::File> {
    match fs::File::create_new(path.as_ref()) {
        Ok(fd) => Ok(fd),
        Err(e) => {
            if overwrite {
                fs::File::options()
                    .write(true)
                    .read(true)
                    .open(path.as_ref())
            } else {
                return Err(e);
            }
        },
    }
}

fn find_file<P: AsRef<path::Path>> (path: P, filename: &str) -> Option<path::PathBuf> {
    let mut path = path.as_ref();

    loop {
        for entry in fs::read_dir(path).unwrap() {
            match entry {
                Ok(dir_entry) => {
                    if dir_entry.file_name().eq(filename) {
                        return Some(dir_entry.path())
                    }
                },
                Err(_) => continue
            }
        }

        path = match path.parent() {
            Some(r) => r,
            None => break 
        };
    }

    None
}

fn initialize_project(path: &path::PathBuf, with_git: bool, overwrite: bool, target: &Option<String>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = path::absolute(path).unwrap();

    //
    // Checks
    //

    if path.is_file() {
        return Err(Box::new(Error::new(ErrorKind::IsNotADirectory)));
    }
    
    if !path.exists() {
        fs::create_dir(&path)?;
    } else if path.join(CONFIG_FILENAME).exists() && !overwrite {
        return Err(Box::new(Error::new(ErrorKind::AlreadyExists)));
    }

    //
    // Create dirs 
    //
    for directory in ["src", "target", "include"] {
        match fs::create_dir(path.join(directory)) {
            Ok(r) => r,
            Err(_e) => {
                if overwrite {
                    continue;
                }
            }
        }
    }

    let config_file = path.join(CONFIG_FILENAME);

    //
    // Create files
    //
    let mut config_toml = create_new2(&config_file, overwrite)?;
    let mut makefile = create_new2(path.join("makefile"), overwrite)?;
    let mut compilation_database = create_new2(path.join("compile_commands.json"), overwrite)?;
    create_new2(path.join(".clang-format"), overwrite)?;
    let mut main_template = create_new2(path.join("src/main.c"), overwrite)?;
    

    //
    // Set up Git
    //
    if with_git {
        if path.join(".git").exists() && overwrite {
            fs::remove_dir_all(path.join(".git"))?;
        }
        gix::init(&path)?;

        let mut gitignore = create_new2(path.join(".gitignore"), overwrite)?;

        gitignore.write_all(".build/\ncompile_commands.json\n.clang-format".as_bytes())?;
    }

    //
    // Load templates
    //
    let template_config = config::Config {
        package: config::ConfigPackage {
            name: path.file_name().unwrap().to_string_lossy().to_string()
        },
        build: config::ConfigBuild {
            linker: config::ConfigBuildLinker {
                exec: "clang".to_string(),
                args: "-c -Iinclude -o target/main.o".to_string(),
            },
            compiler: config::ConfigBuildCompiler {
                exec: "clang".to_string(),
                args: "-Iinclude -o target/main".to_string(),
            },
            files: vec![ "src/main.c".to_string() ],
            target: target.clone(),
            commands: None
            } 
        };
    
    let config_parser = config::ConfigManager::new(template_config);

    config_toml.write_all(config_parser.to_string()?.as_bytes())?;
    main_template.write_all(MAIN_TEMPLATE.trim().as_bytes())?;

    //
    // Set up the project build/LSP 
    //

    let compilation_database_template = config_parser.create_compilation_database(&path);
    let makefile_template = config_parser.create_makefile();

    compilation_database.write_all(compilation_database_template.to_string().trim().as_bytes())?;
    makefile.write_all(makefile_template.trim().as_bytes())?;

    Ok(())
}

fn build_project() -> std::result::Result<(), Box<dyn std::error::Error>> {
    //
    // Find the config file
    //
    let config_path = match find_file(std::env::current_dir().unwrap(), CONFIG_FILENAME) {
        Some(r) => r,
        None => return Err(Box::new(IoError::new(IoErrorKind::NotFound, "Config file not found")))
    };

    let config = fs::File::options()
        .write(false)
        .read(true)
        .open(&config_path)?;

    let config_parser = config::ConfigManager::parse(config);

    //
    // Set up the build/LSP 
    //

    let project_path = config_path.parent().unwrap();
    let cache_path = project_path.join(".cache");

    let mut makefile = create_new2(project_path.join("makefile"), true)?;
    let mut compilation_database = create_new2(project_path.join("compile_commands.json"), true)?;

    if !cache_path.exists() {
        fs::create_dir(cache_path)?;
    }

    let compilation_database_content = config_parser.create_compilation_database(project_path);
    let makefile_content = config_parser.create_makefile();

    compilation_database.write_all(compilation_database_content.to_string().trim().as_bytes())?;
    makefile.write_all(makefile_content.trim().as_bytes())?;

    let exec_make_output = std::process::Command::new("make")
        .arg("build")
        .output()?;

    println!("{}", String::from_utf8_lossy(&exec_make_output.stdout));

    Ok(())
}

fn main() {
    let args = cli::Cli::parse();

    match &args.commands {
        cli::CommandsArgs::Init { path, no_git, force, target} => {
            match initialize_project(path, !no_git, *force, target) {
                Ok(_) => println!("Project {} was successfully created", path.to_str().unwrap().bright_green().bold()),
                Err(e) => println!("{} while creating the project: {}", "Error".bright_red().bold(), (*e).to_string()),
            }
        }
        cli::CommandsArgs::Build => {
            build_project().unwrap()
        }
    }
}
