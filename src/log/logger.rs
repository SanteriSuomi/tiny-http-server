use std::{
    env,
    fs::File,
    sync::{Mutex, Once},
    time,
};

static INIT_LOGGER: Once = Once::new();
pub static mut LOGGER: Option<Mutex<Logger>> = None;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        unsafe {
            if let Some(logger) = &crate::log::logger::LOGGER {
                let mut logger = logger.lock().unwrap();
                let time_since_launch =  logger.get_time_since_start();
                let log_string = format!("{}: {} \n", time_since_launch, format_args!($($arg)*));
                write!(logger.log_file, "{}", log_string).unwrap();
           }
        }
    };
}

pub struct Logger {
    pub log_file: File,
    pub start_time: time::SystemTime,
}

impl Logger {
    pub fn init(log_file_path: &str) {
        INIT_LOGGER.call_once(|| {
            let mut path = env::current_dir().unwrap();
            path.push(log_file_path);
            unsafe {
                LOGGER = Some(Mutex::new(Logger {
                    log_file: File::create(path).unwrap(),
                    start_time: time::SystemTime::now(),
                }))
            };
        });
    }

    pub fn get_time_since_start(&self) -> String {
        let since_start = time::SystemTime::now()
            .duration_since(self.start_time)
            .expect("Error: Time went backwards");
        since_start.as_secs().to_string()
    }
}
