// TODO: better error exceptions
mod config;
mod cli;

use clap::Parser;
use gix;
use owo_colors::OwoColorize;
use std::fs::create_dir;
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

fn main() {
    let args = cli::Cli::parse();

    let config_path = gix::path::env::home_dir().unwrap().join(".config").join("xylo");

    if !config_path.exists() {
        //
        // Create config directory
        //

        fs::create_dir(&config_path).unwrap();
    }

    let config_parser = if !config_path.join(CONFIG_FILENAME).exists() {
        //
        // Load templates
        //
        let template_config = config::Config {
            build: config::ConfigBuild {
                linker: config::ConfigBuildLinker {
                    exec: "clang".to_string(),
                    args: "-c -Iinclude -o target/main.o".to_string(),
                },
                compiler: config::ConfigBuildCompiler {
                    exec: "clang".to_string(),
                    args: "-Iinclude -o target/main".to_string(),
                },
                main_filename: "main".to_string(),
                target: None,
                commands: None
                } 
            };

        let config_parser = config::ConfigManager::new(template_config);

        let mut config_file = fs::File::create_new(config_path.join(CONFIG_FILENAME)).unwrap();
        config_file.write_all(config_parser.to_string().unwrap().as_bytes()).unwrap();

        config_parser
    } else {
        let config_file = fs::File::options()
        .read(true)
        .open(config_path.join(CONFIG_FILENAME))
        .unwrap();

        config::ConfigManager::parse(config_file)
    };

    let path = path::absolute(&args.path).unwrap();

    //
    // Checks
    //

    if path.is_file() {
        eprintln!("Error: Is not a directory");
        return;
    } else if path.exists() && args.force {
        fs::remove_dir_all(&path).unwrap()
    } else if path.exists() {
        eprintln!("Error: Already exists");
        return
    } else {
        create_dir(&path).unwrap()
    }

    //
    // Create dirs 
    //
    for directory in ["src", "target", "include"] {
        match fs::create_dir(path.join(directory)) {
            Ok(r) => r,
            Err(_e) => {
                if args.force {
                    continue;
                }
            }
        }
    }

    let config_file = path.join(CONFIG_FILENAME);

    //
    // Create files
    //
    let mut config_toml = create_new2(&config_file, args.force).unwrap();
    let mut makefile = create_new2(path.join("makefile"), args.force).unwrap();
    let mut compilation_database = create_new2(path.join("compile_commands.json"), args.force).unwrap();
    create_new2(path.join(".clang-format"), args.force).unwrap();
    let mut main_template = create_new2(path.join("src/main.c"), args.force).unwrap();
    

    //
    // Set up Git
    //
    if !args.no_git {
        if path.join(".git").exists() && args.force {
            fs::remove_dir_all(path.join(".git")).unwrap();
        }

        gix::init(&path).unwrap();

        let mut gitignore = create_new2(path.join(".gitignore"), args.force).unwrap();

        gitignore.write_all(".build/\ncompile_commands.json\n.clang-format".as_bytes()).unwrap();
    }

    config_toml.write_all(config_parser.to_string().unwrap().as_bytes()).unwrap();
    main_template.write_all(MAIN_TEMPLATE.trim().as_bytes()).unwrap();

    //
    // Set up the project build/LSP 
    //

    let compilation_database_template = config_parser.create_compilation_database(&path);
    let makefile_template = config_parser.create_makefile();

    compilation_database.write_all(compilation_database_template.to_string().trim().as_bytes()).unwrap();
    makefile.write_all(makefile_template.trim().as_bytes()).unwrap();

    println!("Project {} was successfully created", args.path.to_str().unwrap().bright_green().bold());
}
