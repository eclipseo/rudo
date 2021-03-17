use systemd::journal;
use log::LevelFilter;
use std::error::Error;

pub fn log_journald() -> Result<(), Box<dyn Error>> {
    // Initialize Logs
    journal::JournalLog::init()?;
    log::set_max_level(LevelFilter::Info);
    info!("Starting logs");
    Ok(())
}
