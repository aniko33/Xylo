use std::{fs::File, io::Read, path::PathBuf};
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
pub struct ConfigBuild {
    pub compiler: ConfigBuildCompiler,
    pub main_filename: String,
    pub target: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigStructure {
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub build: ConfigBuild,
    pub structure: ConfigStructure
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

    pub fn get_compile_command(&self) -> String {

        let compiler_args = match &self.config.build.target {
            Some(target) => format!("-target {} {}", target, self.config.build.compiler.args),
            None => self.config.build.compiler.args.clone()
        };

        let compiler_command = format!(
            "{} {} {}",
            self.config.build.compiler.exec,
            self.config.build.main_filename,
            compiler_args
        );

        compiler_command
    }

    pub fn create_compilation_database<P: AsRef<std::path::Path>> (&self, project_path: P) -> serde_json::Value {
        json!(
            [
                { 
                    "directory": project_path.as_ref(),
                    "command": self.get_compile_command(),
                    "file": "src/main.c",
                }
            ]
        )
    }

    pub fn to_string(&self) -> Result<String, toml::ser::Error>{
        toml::to_string(&self.config)
    }
}
