use crate::consts::*;

use std::path::PathBuf;

/// Helper Struct for initializing logging
///
/// The LogBuilder implements the builder pattern for creating a logger.
#[derive(Default, Debug, Clone, Copy)]
pub struct LogBuilder {
    verbosity: u8,
    stdout: bool,
    file: bool,
}

impl LogBuilder {
    /// Create a new LogBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new LogBuilder with verbosity
    pub fn with_verbosity(verbosity: u8) -> Self {
        Self {
            verbosity,
            stdout: false,
            file: false,
        }
    }

    /// Set verbosity level
    pub fn set_verbosity(mut self, verbosity: u8) -> Self {
        self.verbosity = verbosity;
        self
    }

    /// Set log to stdout to true or false
    pub fn set_stdout(mut self, stdout: bool) -> Self {
        self.stdout = stdout;
        self
    }

    /// Set log to file to true or false
    pub fn set_file(mut self, file: bool) -> Self {
        self.file = file;
        self
    }

    /// Get verbosity level
    pub fn get_verbosity(&self) -> u8 {
        self.verbosity
    }

    /// Get log to stdout
    pub fn get_stdout(&self) -> bool {
        self.stdout
    }

    /// Get log to file
    pub fn get_file(&self) -> bool {
        self.file
    }

    /// Initialize logger with values from the LogBuilder
    pub fn init_logger(self) -> Result<(), fern::InitError> {
        setup_logger(self)
    }
}

fn setup_logger(log_builder: LogBuilder) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = match log_builder.get_verbosity() {
        0 => base_config.level(log::LevelFilter::Info),
        1 => base_config.level(log::LevelFilter::Debug),
        _ => base_config.level(log::LevelFilter::Trace),
    };

    if log_builder.get_file() {
        base_config = base_config.chain(file_logger()?);
    }

    if log_builder.get_stdout() {
        base_config = base_config.chain(stdout_logger()?);
    }

    base_config.apply()?;

    Ok(())
}

fn file_logger() -> Result<fern::Dispatch, fern::InitError> {
    let log_path: PathBuf = [
        dirs::home_dir().expect("Failed to get home_directory"),
        HERMOD_BASE_DIR.into(),
        HERMOD_LOG_FILE.into(),
    ]
    .iter()
    .collect();

    let cfg = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(log_path)?);
    Ok(cfg)
}

fn stdout_logger() -> Result<fern::Dispatch, fern::InitError> {
    let cfg = fern::Dispatch::new()
        .format(|out, message, record| {
            if record.level() > log::LevelFilter::Info {
                out.finish(format_args!(
                    "---\nDebug: {}: {}\n---",
                    chrono::Local::now().format("%H%M%s"),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            }
        })
        .chain(std::io::stdout());
    Ok(cfg)
}
