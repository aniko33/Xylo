mod config;
mod cli;

use clap::Parser;
use config::ProfileManager;
use git2;
use dirs;
use owo_colors::OwoColorize;
use std::io::Write;
use std::{fs, path};

const CONFIG_FILENAME: &str = "xylo.toml";

macro_rules! strPathBuf {
    ($str: literal) => {
        path::PathBuf::from($str)
    };
}

fn main() {
    let args = cli::Cli::parse();

    let config_path = dirs::home_dir()
        .expect("Unable to get `home_dir`")
        .join(".config")
        .join("xylo");

    if !config_path.exists() {
        //
        // Create config directory
        //
        fs::create_dir(&config_path).unwrap();
    }

    //
    // Get config parser
    //

    let config_parser = if !config_path.join(CONFIG_FILENAME).exists() {
        //
        // Create and load a template
        //
        let template_config = config::Config {
            default_profile: "template".to_string(),
            profile: vec![
                config::Profile {
                    name: "template".to_string(),
                    build: config::ConfigBuild {
                        compiler: config::ConfigBuildCompiler {
                            exec: "clang".to_string(),
                            args: "-Iinclude -o target/main".to_string(),
                        },
                        main_filename: "main".to_string(),
                        target: None,
                        },
                    structure: config::ConfigStructure {
                        files: vec![
                            strPathBuf!("src/main.c"),
                        ],
                        directories: vec![
                            strPathBuf!("src/"),
                            strPathBuf!("target/"),
                            strPathBuf!("include/"),
                        ]
                    }
                }
            ]
        };

        let config_parser = config::ConfigManager::new(template_config);

        let mut config_file = fs::File::create_new(config_path.join(CONFIG_FILENAME)).unwrap();
        config_file.write_all(config_parser.to_string().unwrap().as_bytes()).unwrap();

        config_parser
    } else {
        //
        // Load config
        //

        let config_file = fs::File::options()
        .read(true)
        .open(config_path.join(CONFIG_FILENAME))
        .expect("Unable to open filename");

        config::ConfigManager::parse(config_file)
    };

    //
    // Get profile
    //

    let profile = match args.profile {
        Some(ref p) => {
            config_parser.get_profile(p)
        },
        None => {
            config_parser.get_profile(&config_parser.config.default_profile)
        }
    };

    if profile.is_none() {
        errorln!("'{}' profile don't exists", args.profile.unwrap_or(config_parser.config.default_profile));
    }

    let profile_parser = ProfileManager::new(profile.unwrap());

    let path = path::absolute(&args.path).unwrap();

    //
    // Checks
    //

    if path.is_file() {
        errorln!("'{}' is not a directory", path.to_string_lossy());
    } else if path.exists() && args.force {
        fs::remove_dir_all(&path).unwrap()
    } else if path.exists() {
        errorln!("'{}' already exists", path.to_string_lossy());
    } else {
        fs::create_dir(&path).unwrap()
    }

    //
    // Create dirs 
    //
    for directory in &profile_parser.profile.structure.directories {
        match fs::create_dir_all(path.join(directory)) {
            Ok(_) => (),
            Err(e) => errorln!("{}", e)
        }
    }

    //
    // Create files
    //
    let mut _makefile = fs::File::create_new(path.join("makefile")).unwrap();
    let mut compilation_database = fs::File::create_new(path.join("compile_commands.json")).unwrap();

    for file in &profile_parser.profile.structure.files {
        match fs::File::create_new(path.join(file)) {
            Ok(_) => (),
            Err(e) => errorln!("{}", e)
        }
    }

    //
    // Set up Git
    //
    if !args.no_git {
        if path.join(".git").exists() && args.force {
            fs::remove_dir_all(path.join(".git")).unwrap();
        }

        git2::Repository::init(&path).unwrap();
        let mut gitignore = fs::File::create_new(path.join(".gitignore")).unwrap();

        gitignore.write_all(".build/\ncompile_commands.json\n.clang-format".as_bytes()).unwrap();
    }

    //
    // Set up the project LSP
    //

    let compilation_database_template = profile_parser.create_compilation_database(&path);
    compilation_database.write_all(compilation_database_template.to_string().trim().as_bytes()).unwrap();

    println!("Project {} was successfully created", args.path.to_str().unwrap().bright_green().bold());
}
