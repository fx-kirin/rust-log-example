extern crate log;
extern crate log4rs;

use log::{debug, error, info, trace, warn};
use log::{SetLoggerError};

fn main() -> Result<(), SetLoggerError> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.

    error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only");

    Ok(())
}
