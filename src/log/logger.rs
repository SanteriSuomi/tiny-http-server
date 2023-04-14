use std::{
    env,
    fs::File,
    io::Write,
    sync::{Mutex, Once},
    time::SystemTime,
};

static INIT_LOGGER: Once = Once::new();
pub static mut LOGGER: Option<Mutex<Logger>> = None;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        unsafe {
            if let Some(logger) = &crate::log::logger::LOGGER {
                logger.lock().unwrap().log(&format_args!($($arg)*).to_string());
           }
        }
    };
}

// Singleton logger.
pub struct Logger {
    pub log_file: File,
    pub start_time: SystemTime,
}

impl Logger {
    pub fn init(log_file_path: &str) {
        INIT_LOGGER.call_once(|| {
            let mut path = env::current_dir().unwrap();
            path.push(log_file_path);
            unsafe {
                LOGGER = Some(Mutex::new(Logger {
                    log_file: File::create(path).unwrap(),
                    start_time: SystemTime::now(),
                }))
            };
        });
    }

    // Get the time since the logger was initialized.
    pub fn get_time_since_start(&self) -> String {
        let since_start = SystemTime::now()
            .duration_since(self.start_time)
            .unwrap_or_default();
        since_start.as_secs().to_string()
    }

    // Log a message to the log file.
    pub fn log(&mut self, message: &str) {
        if let Err(e) = write!(
            self.log_file,
            "{}",
            format!("{}: {} \n", self.get_time_since_start(), message)
        ) {
            println!("LOG ERROR: {e}");
        }
    }
}
