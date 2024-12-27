use std::{fs::File, io::Read};
use serde_json::json;
use toml;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigPackage {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct ConfigCommands {
    pre_build: Option<String>,
    post_build: Option<String>,
    clean: String
}

#[derive(Serialize, Deserialize)]
pub struct ConfigBuildCompiler {
    pub exec: String,
    pub args: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigBuildLinker {
    pub exec: String,
    pub args: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigBuild {
    pub compiler: ConfigBuildCompiler,
    pub linker: ConfigBuildLinker,
    pub main_filename: String,
    pub target: Option<String>,
    pub commands: Option<ConfigCommands>
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub build: ConfigBuild
}

pub struct ConfigManager {
    pub config: Config
}

impl ConfigManager {
    pub fn parse(mut fd: File) -> Self {
        let mut value = String::new();
        fd.read_to_string(&mut value).unwrap();
        
        let config = toml::from_str(&value).unwrap();
        Self { config }
    }

    pub fn new(config: Config) -> Self {
        let config = config;
        Self { config }
    }

    pub fn get_compile_command(&self) -> (String, String) {
        let linker_args = match &self.config.build.target {
            Some(target) => format!("-target {} {}", target, self.config.build.linker.args),
            None => self.config.build.linker.args.clone()
        };

        let compiler_args = match &self.config.build.target {
            Some(target) => format!("-target {} {}", target, self.config.build.compiler.args),
            None => self.config.build.compiler.args.clone()
        };

        let linker_command = format!(
            "{} {} {}",
            self.config.build.linker.exec,
            self.config.build.main_filename,
            linker_args
        );

        let compiler_command = format!(
            "{} {} {}",
            self.config.build.linker.exec,
            self.config.build.main_filename,
            compiler_args
        );

        (linker_command, compiler_command)
    }

    pub fn create_compilation_database<P: AsRef<std::path::Path>> (&self, project_path: P) -> serde_json::Value {
        json!(
            [
                { 
                    "directory": project_path.as_ref(),
                    "command": self.get_compile_command().1,
                    "file": "src/main.c",
                }
            ]
        )
    }

    pub fn create_makefile(&self) -> String {
        let (linker_command, compiler_command) = self.get_compile_command();
        let mut makefile_content = String::new();

        makefile_content.push_str(&format!("target/{}.o: src/{}.c\n", &self.config.build.main_filename, &self.config.build.main_filename));
        makefile_content.push_str(&format!("\t{}\n", linker_command));

        makefile_content.push_str(&format!("build: target/{}.o\n", &self.config.build.main_filename));
        if let Some(config_commands) = &self.config.build.commands {
            if let Some(pre_build) = &config_commands.pre_build {
                makefile_content.push_str(&format!("\t{}\n", pre_build));
            }
        } 

        makefile_content.push_str(&format!("\t{}\n", compiler_command));

        if let Some(config_commands) = &self.config.build.commands {
            if let Some(post_build) = &config_commands.post_build {
                makefile_content.push_str(&format!("\t{}\n", post_build));
            }
        } 

        makefile_content
    }

    pub fn to_string(&self) -> Result<String, toml::ser::Error>{
        toml::to_string(&self.config)
    }
}
