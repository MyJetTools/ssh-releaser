use rust_extensions::StrOrString;
use tokio::sync::Mutex;

pub struct LogProcess {
    pub process_name: String,
    pub logs: Vec<String>,
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
            processes: Mutex::new(Vec::new()),
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
        to_write.logs.push(log);
    }

    pub async fn write_error(&self, log: impl Into<StrOrString<'static>>) {
        let mut process = self.processes.lock().await;
        let log = log.into().to_string();

        let to_write = process.last_mut().unwrap();

        println!("Err: {}", log);

        to_write.logs.push(format!("Err: {}", log));
    }
}
