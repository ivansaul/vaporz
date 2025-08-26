use color_eyre::Result;
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, WriteLogger};
use std::fs::File;

pub fn init() -> Result<()> {
    let log_file = File::create("app.log")?;

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        log_file,
    )])?;

    Ok(())
}
