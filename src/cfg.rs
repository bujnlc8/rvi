use serde::{Deserialize, Serialize};

use anyhow::Result;
use std::io::Write;

use std::fs::File;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// The directory of project
    project_dir: String,
    /// The content of rust-project.json
    rust_project_json: String,
    /// The content of Cargo.toml, if not specified, will gennerate a simple one.
    cargo_toml: Option<String>,
    /// the editor to open the file, the default is vim.
    editor: Option<String>,
    /// ym_ycm_extra_conf path, if you use [Youcompleteme](https://github.com/ycm-core/YouCompleteMe)
    ycm_extra_conf: Option<String>,
}

impl Config {
    pub fn loads() -> Result<Self> {
        let config_dir = dirs::home_dir()
            .expect("get home dir failed")
            .join(".config");
        if !config_dir.exists() {
            std::fs::create_dir(&config_dir)?;
        }
        let config_path = config_dir.join("rvi.toml");
        match std::fs::read_to_string(config_path) {
            Ok(s) => {
                let o: Config = toml::from_str(s.as_str())?;
                Ok(o)
            }
            Err(e) => {
                eprint!("Get config file error, ");
                Err(anyhow::Error::from(e))
            }
        }
    }

    fn get_rust_project_json(&self) -> String {
        self.rust_project_json.clone()
    }

    fn get_cargo_toml(&self) -> String {
        match self.cargo_toml {
            Some(ref e) => e.clone(),
            None => r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"
description = "Just a test"

[lib]
path='./{#file_name}'

[dependencies]"#
                .to_string(),
        }
    }

    pub fn get_editor(&self) -> String {
        match self.editor {
            Some(ref e) => e.clone(),
            None => "vim".to_string(),
        }
    }

    fn get_project_dir(&self, file_name: &str) -> String {
        let h = dirs::home_dir().expect("get home dir failed");
        let p = self.project_dir.replace('~', h.to_str().unwrap());
        if !std::path::PathBuf::from(&p).exists() {
            if let Err(e) = std::fs::create_dir(&p) {
                eprintln!("Create project dir err, {}", e);
                std::process::exit(1);
            }
        }
        if file_name.contains(std::path::MAIN_SEPARATOR) {
            let dir = std::path::PathBuf::from(&p)
                .join(std::path::PathBuf::from(file_name).parent().unwrap());
            if !dir.exists() {
                if let Err(e) = std::fs::create_dir(&dir) {
                    eprintln!("Create dir err, {}", e);
                    std::process::exit(1);
                }
            }
            return dir.to_string_lossy().to_string();
        }
        p
    }

    pub fn get_full_file_path(&self, file_name: &str) -> String {
        let project_dir = self.get_project_dir(file_name);
        if file_name.contains(std::path::MAIN_SEPARATOR) {
            return std::path::PathBuf::from(project_dir)
                .join(std::path::PathBuf::from(file_name).file_name().unwrap())
                .to_string_lossy()
                .to_string();
        }
        std::path::PathBuf::from(project_dir)
            .join(file_name)
            .to_string_lossy()
            .to_string()
    }

    #[cfg(target_family = "unix")]
    fn link_ycm_extra_conf(&self, file_name: &str) -> Result<()> {
        let p =
            std::path::PathBuf::from(self.get_project_dir(file_name)).join(".ycm_extra_conf.py");
        if p.exists() {
            return Ok(());
        }
        std::process::Command::new("cp")
            .args(vec![
                self.ycm_extra_conf
                    .as_ref()
                    .unwrap()
                    .replace(
                        '~',
                        dirs::home_dir()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                            .as_str(),
                    )
                    .as_str(),
                p.to_string_lossy().to_string().as_str(),
            ])
            .status()?;
        Ok(())
    }

    #[cfg(not(target_family = "unix"))]
    fn copy_ycm_extra_conf(&self, file_name: &str) -> Result<()> {
        let p =
            std::path::PathBuf::from(self.get_project_dir(file_name)).join(".ycm_extra_conf.py");
        if p.exists() {
            return Ok(());
        }
        let conf = std::fs::read_to_string(
            self.ycm_extra_conf.as_ref().unwrap().replace(
                '~',
                dirs::home_dir()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .as_str(),
            ),
        )?;
        let mut f = File::create(p)?;
        f.write_all(conf.as_bytes())?;
        Ok(())
    }

    pub fn write_config(&self, file_name: &str) -> Result<()> {
        let pure_file_name = std::path::PathBuf::from(file_name)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let cargo_toml = self
            .get_cargo_toml()
            .replace("{#file_name}", pure_file_name.as_str());
        // write cargo.toml
        let cargo_path =
            std::path::PathBuf::from(self.get_project_dir(file_name)).join("Cargo.toml");
        let mut cargo_file = File::create(cargo_path)?;
        cargo_file.write_all(cargo_toml.as_bytes())?;
        // write rust-project.json
        let rust_project_json = self
            .get_rust_project_json()
            .replace("{#file_name}", pure_file_name.as_str());
        let rust_project_json_path =
            std::path::PathBuf::from(self.get_project_dir(file_name)).join("rust-project.json");
        let mut json_file = File::create(rust_project_json_path)?;
        json_file.write_all(rust_project_json.as_bytes())?;
        // write ycm_extra_conf.py
        if self.ycm_extra_conf.is_some() && !self.ycm_extra_conf.as_ref().unwrap().trim().is_empty()
        {
            #[cfg(target_family = "unix")]
            self.link_ycm_extra_conf(file_name)?;
            #[cfg(not(target_family = "unix"))]
            self.copy_ycm_extra_conf(file_name)?;
        }
        Ok(())
    }
}
