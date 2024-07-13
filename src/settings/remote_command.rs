use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::file_path::{FilePath, FilePathRef};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCommand {
    #[serde(rename = "type")]
    pub r#type: String,
    pub ssh: Option<String>,
    pub commands: Option<Vec<RemoteCommandItem>>,
    pub file: Option<UploadFileModel>,
    pub post_data: Option<PostDataModel>,
    pub get_data: Option<GetDataModel>,
    pub template_file_name: Option<String>,
    pub name: Option<String>,
    pub params: Option<HashMap<String, String>>,
    pub domain: Option<String>,
    pub ip: Option<String>,
    pub is_proxy: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCommandItem {
    pub name: String,
    pub exec: Option<String>,
    pub exec_from_file: Option<String>,
    pub ignore_error: bool,
    pub params: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileModel {
    pub local_file: String,
    pub remote_file: String,
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
        params: Option<HashMap<String, String>>,
    },

    PostRequest {
        ssh: String,
        data: PostDataModel,
    },

    GetRequest {
        ssh: String,
        data: GetDataModel,
    },

    FromTemplate {
        from_file: String,
        script_file_path: FilePath,
        params: Option<HashMap<String, String>>,
    },

    WriteCloudFlareDomainARecord {
        model: WriteCloudFlareDomainARecordModel,
    },
}

pub struct WriteCloudFlareDomainARecordModel {
    pub domain: String,
    pub ip: String,
    pub is_proxy: bool,
}

impl RemoteCommand {
    pub fn get_remote_command_type(
        &self,
        script_file_path: Option<FilePathRef<'_>>,
    ) -> RemoteCommandType {
        match self.r#type.as_str() {
            "execute" => {
                if self.commands.is_none() {
                    panic!("Type 'execute' requires commands to be described");
                }

                if self.ssh.is_none() {
                    panic!("Type 'execute' requires ssh property");
                }

                return RemoteCommandType::ExecuteCommands {
                    ssh: self.ssh.as_ref().unwrap().clone(),
                    commands: self.commands.as_ref().unwrap().clone(),
                };
            }

            "upload" => {
                if self.file.is_none() {
                    panic!("Type 'upload' requires file property");
                }

                if self.ssh.is_none() {
                    panic!("Type 'execute' requires ssh property");
                }

                return RemoteCommandType::UploadFile {
                    ssh: self.ssh.as_ref().unwrap().clone(),
                    file: self.file.as_ref().unwrap().clone(),
                    params: self.params.clone(),
                };
            }

            "http_post" => {
                if self.post_data.is_none() {
                    panic!("Type 'http_post' requires post_data property");
                }

                if self.ssh.is_none() {
                    panic!("Type 'execute' requires ssh property");
                }

                return RemoteCommandType::PostRequest {
                    ssh: self.ssh.as_ref().unwrap().clone(),
                    data: self.post_data.as_ref().unwrap().clone(),
                };
            }
            "http_get" => {
                if self.get_data.is_none() {
                    panic!("Type 'http_get' requires get_data property");
                }

                if self.ssh.is_none() {
                    panic!("Type 'execute' requires ssh property");
                }

                return RemoteCommandType::GetRequest {
                    ssh: self.ssh.as_ref().unwrap().clone(),
                    data: self.get_data.as_ref().unwrap().clone(),
                };
            }
            "from_template" => {
                if self.template_file_name.is_none() {
                    panic!("Type 'from_template' requires template_file_name property");
                }

                let params = self.params.clone();

                let template_file_name = self.template_file_name.as_ref().unwrap().clone();

                if script_file_path.is_none() {
                    panic!("BUG. Script file path somehow is none for 'from_template' command");
                }

                return RemoteCommandType::FromTemplate {
                    from_file: template_file_name,
                    params,
                    script_file_path: script_file_path.as_ref().unwrap().to_owned(),
                };
            }
            "write_cloud_flare_domain" => {
                if self.domain.is_none() {
                    panic!("Type 'write_cloud_flare_domain' requires 'domain' property");
                }

                if self.ip.is_none() {
                    panic!("Type 'write_cloud_flare_domain' requires 'ip' property");
                }

                if self.is_proxy.is_none() {
                    panic!("Type 'write_cloud_flare_domain' requires 'is_proxy' property");
                }

                let model = WriteCloudFlareDomainARecordModel {
                    domain: self.domain.as_ref().unwrap().clone(),
                    ip: self.ip.as_ref().unwrap().clone(),
                    is_proxy: self.is_proxy.unwrap(),
                };

                return RemoteCommandType::WriteCloudFlareDomainARecord { model };
            }
            _ => panic!("Unknown remote command type {}", self.r#type),
        }
    }
}
