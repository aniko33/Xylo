use std::{fs::File, io::Read};
use toml;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub package: ConfigPackage,
    pub build: ConfigBuild
}

#[derive(Serialize, Deserialize)]
pub struct ConfigPackage {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct ConfigBuild {
    pub compiler: String,
    pub files: String,
    pub args: String,
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

    pub fn to_string(&self) -> Result<String, toml::ser::Error>{
        toml::to_string(&self.config)
    }
}
