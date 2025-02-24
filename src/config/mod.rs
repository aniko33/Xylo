use std::{fs::File, io::Read, path::PathBuf};
use serde_json::json;
use toml;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigBuildCompiler {
    pub exec: String,
    pub args: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigBuild {
    pub compiler: ConfigBuildCompiler,
    pub main_filename: String,
    pub target: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigStructure {
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub build: ConfigBuild,
    pub structure: ConfigStructure
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_profile: String,
    pub profile: Vec<Profile>
}

//
// Shitty OOP
//

pub struct ConfigManager {
    pub config: Config
}

impl ConfigManager {
    pub fn parse(mut fd: File) -> Self {
        let mut value = String::new();
        fd.read_to_string(&mut value).unwrap();

        let config = toml::from_str(&value).expect("Invalid config");
        Self { config }
    }

    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn get_profile(&self, profile_name: &String) -> Option<Profile> {
        self.config.profile.iter().find(|p| p.name.eq(profile_name)).cloned()
    }

    pub fn to_string(&self) -> Result<String, toml::ser::Error>{
        toml::to_string(&self.config)
    }
}

pub struct ProfileManager {
    pub profile: Profile
}

impl ProfileManager {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }

    pub fn get_compile_command(&self) -> String {
        let compiler_args = match &self.profile.build.target {
            Some(target) => format!("-target {} {}", target, self.profile.build.compiler.args),
            None => self.profile.build.compiler.args.clone()
        };

        let compiler_command = format!(
            "{} {} {}",
            self.profile.build.compiler.exec,
            self.profile.build.main_filename,
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

}
