use rust_extensions::StrOrString;
use tokio::sync::Mutex;

pub enum LogEvent {
    Info(String),
    Error(String),
    FinishedOk,
}

pub struct LogProcess {
    pub process_name: String,
    pub logs: Vec<LogEvent>,
}

impl LogProcess {
    pub fn new(process_name: String) -> Self {
        Self {
            process_name,
            logs: Vec::new(),
        }
    }
}

pub struct ExecuteLogsContainer {
    pub id: String,
    pub processes: Mutex<Vec<LogProcess>>,
}

impl ExecuteLogsContainer {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            processes: Mutex::new(vec![LogProcess::new("Preparation".to_string())]),
        }
    }

    pub async fn start_process(&self, process_name: impl Into<StrOrString<'static>>) {
        let mut processes = self.processes.lock().await;
        processes.push(LogProcess::new(process_name.into().to_string()));
    }

    pub async fn write_log(&self, log: impl Into<StrOrString<'static>>) {
        let log = log.into().to_string();
        let mut process = self.processes.lock().await;

        let to_write = process.last_mut().unwrap();
        println!("{}", log);
        to_write.logs.push(LogEvent::Info(log));
    }

    pub async fn write_error(&self, log: impl Into<StrOrString<'static>>) {
        let mut process = self.processes.lock().await;
        let log = log.into().to_string();

        let to_write = process.last_mut().unwrap();

        println!("Err: {}", log);

        to_write.logs.push(LogEvent::Error(log));
    }

    pub async fn write_finished_ok(&self) {
        let mut process = self.processes.lock().await;

        let to_write = process.last_mut().unwrap();

        to_write.logs.push(LogEvent::FinishedOk);
    }

    pub async fn get_as_html(&self) -> String {
        let process = self.processes.lock().await;

        let mut result = String::new();

        for process in process.as_slice() {
            result.push_str(&format!("<h3>{}</h3>", process.process_name));

            result.push_str("<ul>");

            for log in process.logs.iter() {
                match log {
                    LogEvent::Info(log) => {
                        result.push_str(&format!("<li>{}</li>", log));
                    }
                    LogEvent::Error(log) => {
                        result.push_str(&format!("<li style='color: red;'>{}</li>", log));
                    }
                    LogEvent::FinishedOk => {
                        result.push_str("<li style='color: green;'>Finished Ok</li>");
                    }
                }
            }

            result.push_str("</ul>");
        }

        result
    }
}
