use crate::consts::HERMOD_LOG_FILE;

pub fn setup_logger(stdout: bool, verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => base_config.level(log::LevelFilter::Info),
        1 => base_config.level(log::LevelFilter::Debug),
        _ => base_config.level(log::LevelFilter::Trace),
    };

    base_config = base_config.chain(file_logger()?);
    if stdout {
        base_config = base_config.chain(stdout_logger()?);
    }

    base_config.apply()?;

    Ok(())
}

fn file_logger() -> Result<fern::Dispatch, fern::InitError> {
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
        .chain(fern::log_file(HERMOD_LOG_FILE)?);
    Ok(cfg)
}

fn stdout_logger() -> Result<fern::Dispatch, fern::InitError> {
    let cfg = fern::Dispatch::new()
        .format(|out, message, record| {
            if record.level() > log::LevelFilter::Info {
                out.finish(format_args!(
                    "---\nDebug: {}: {}\n",
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
