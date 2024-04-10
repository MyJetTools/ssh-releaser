use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCommand {
    #[serde(rename = "type")]
    pub r#type: String,
    pub ssh: String,
    pub commands: Option<Vec<RemoteCommandItem>>,
    pub file: Option<UploadFileModel>,
    pub post_data: Option<PostDataModel>,
    pub get_data: Option<GetDataModel>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCommandItem {
    pub name: String,
    pub exec: Option<String>,
    pub exec_from_file: Option<String>,
    pub ignore_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileModel {
    pub local_path: String,
    pub remote_path: String,
    pub mode: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDataModel {
    pub url: String,
    pub body_path: Option<String>,
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    raw_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataModel {
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
}

impl PostDataModel {
    pub fn raw_content(&self) -> bool {
        self.raw_content.unwrap_or(false)
    }
}

pub enum RemoteCommandType {
    ExecuteCommands {
        ssh: String,
        commands: Vec<RemoteCommandItem>,
    },

    UploadFile {
        ssh: String,
        file: UploadFileModel,
    },

    PostRequest {
        ssh: String,
        data: PostDataModel,
    },

    GetRequest {
        ssh: String,
        data: GetDataModel,
    },
}

impl RemoteCommand {
    pub fn get_remote_command_type(&self) -> RemoteCommandType {
        match self.r#type.as_str() {
            "execute" => {
                if self.commands.is_none() {
                    panic!("Type 'execute' requires commands to be described");
                }

                return RemoteCommandType::ExecuteCommands {
                    ssh: self.ssh.clone(),
                    commands: self.commands.as_ref().unwrap().clone(),
                };
            }

            "upload" => {
                if self.file.is_none() {
                    panic!("Type 'upload' requires file property");
                }

                return RemoteCommandType::UploadFile {
                    ssh: self.ssh.clone(),
                    file: self.file.as_ref().unwrap().clone(),
                };
            }

            "http_post" => {
                if self.post_data.is_none() {
                    panic!("Type 'http_post' requires post_data property");
                }

                return RemoteCommandType::PostRequest {
                    ssh: self.ssh.clone(),
                    data: self.post_data.as_ref().unwrap().clone(),
                };
            }
            "http_get" => {
                if self.get_data.is_none() {
                    panic!("Type 'http_get' requires get_data property");
                }

                return RemoteCommandType::GetRequest {
                    ssh: self.ssh.clone(),
                    data: self.get_data.as_ref().unwrap().clone(),
                };
            }
            _ => panic!("Unknown remote command type {}", self.r#type),
        }
    }
}
