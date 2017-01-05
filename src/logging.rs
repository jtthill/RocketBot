use std::fs;
use fern;
use time;
use log;

pub fn setup() -> Result<(), fern::InitError> {
    match fs::metadata("log") {
        Ok(meta) => {
            if !meta.is_dir() {
                panic!("Cannot create file directory \"log\" due to existing file. Please rename or delete the file.");
            }
        },
        Err(e) => {
            fs::DirBuilder::new().recursive(false).create("log").unwrap();
        }
    }

    // NOTE: If file exists, data is APPENDED, not OVERWRITTEN
    let logger_cfg = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, location: &log::LogLocation| {
            format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
        }),
        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("log/output.log")],
        level: log::LogLevelFilter::Trace,
    };
    fern::init_global_logger(logger_cfg, log::LogLevelFilter::Trace)
}