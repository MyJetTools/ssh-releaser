use std::sync::atomic::AtomicBool;

use rust_extensions::StrOrString;
use tokio::sync::Mutex;

pub enum LogEvent {
    Info(String),
    Warning(String),
    Error(String),
    FinishedOk,
    FinishedErr(String),
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
    pub finished: AtomicBool,
}

impl ExecuteLogsContainer {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            processes: Mutex::new(vec![LogProcess::new("Preparation".to_string())]),
            finished: AtomicBool::new(false),
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

    pub async fn write_warning(&self, log: impl Into<StrOrString<'static>>) {
        let mut process = self.processes.lock().await;
        let log = log.into().to_string();

        let to_write = process.last_mut().unwrap();

        println!("Warning: {}", log);

        to_write.logs.push(LogEvent::Warning(log));
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
        println!("Finished Ok");
        self.finished
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn write_finished_err(&self, log: impl Into<StrOrString<'static>>) {
        let mut process = self.processes.lock().await;

        let log = log.into().to_string();

        let to_write = process.last_mut().unwrap();

        println!("Finished with Error: {}", log);

        to_write.logs.push(LogEvent::FinishedErr(log));
        self.finished
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_finished(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Relaxed)
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
                    LogEvent::Warning(log) => {
                        result.push_str(&format!("<li style='color: orange;'>{}</li>", log));
                    }
                    LogEvent::Error(log) => {
                        result.push_str(&format!("<li style='color: red;'>{}</li>", log));
                    }
                    LogEvent::FinishedOk => {
                        result
                            .push_str(r#"<span class="badge text-bg-success">Finished OK</span>"#);
                    }

                    LogEvent::FinishedErr(log) => {
                        result.push_str(&format!(
                            r#"<span class="badge text-bg-danger">{}</span>"#,
                            log
                        ));
                    }
                }
            }

            result.push_str("</ul>");
        }

        result
    }
}
