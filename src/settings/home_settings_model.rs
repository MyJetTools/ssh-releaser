use serde::*;

use crate::file_name::FileName;

use super::{CloudFlareConfig, SshConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeSettingsModel {
    pub working_dir: String,
    pub vars: std::collections::HashMap<String, String>,
    pub ssh: Vec<SshConfig>,
    pub cloud_flare: Option<Vec<CloudFlareConfig>>,
    pub features: Option<Vec<String>>,
}

impl HomeSettingsModel {
    /*
       pub async fn load_release_settings(&self, settings: &EnvSettingsModel) -> ReleaseSettingsModel {
           let script_env: Option<&ScriptModel> = None;
           let release_settings = settings.get_file_name(script_env, "release.yaml");

           let content = tokio::fs::read(release_settings.as_str()).await.unwrap();

           println!(
               "Loading release settings from: {}",
               release_settings.as_str()
           );

           let mut release_settings: ReleaseSettingsModel =
               serde_yaml::from_slice(content.as_slice()).unwrap();

           if let Some(var_files) = release_settings.var_files.clone() {
               for var_file in var_files {
                   let file_name = settings.get_file_name(script_env, var_file.as_str());

                   let content = file_name.load_content().await;

                   let external_vars: ExternalVariablesModel =
                       match serde_yaml::from_slice(content.as_slice()) {
                           Ok(result) => result,
                           Err(err) => {
                               panic!("can not load yaml: {}. Err: {}", file_name.as_str(), err)
                           }
                       };

                   for (key, value) in external_vars.vars {
                       if release_settings.vars.contains_key(key.as_str()) {
                           panic!("Variable {} already defined", key);
                       }

                       release_settings.vars.insert(key, value);
                   }
               }
           }

           for (key, value) in self.vars.clone() {
               if release_settings.vars.contains_key(key.as_str()) {
                   panic!("Variable {} already defined", key);
               }

               release_settings.vars.insert(key, value);
           }

           release_settings
       }
    */
    pub fn post_process(&mut self) {
        if self.working_dir.starts_with("~") {
            self.working_dir = self
                .working_dir
                .replace("~", std::env::var("HOME").unwrap().as_str());
        }

        /*
        if self.global_vars.starts_with("~") {
            self.global_vars = self
                .global_vars
                .replace("~", std::env::var("HOME").unwrap().as_str());
        }
         */
    }

    pub fn get_release_yaml_file_name(&self) -> FileName {
        let mut file_name = self.working_dir.to_string();

        if !file_name.ends_with(std::path::MAIN_SEPARATOR) {
            file_name.push(std::path::MAIN_SEPARATOR);
        }

        file_name.push_str("release.yaml");
        FileName::new(file_name)
    }
}
