extern crate serde_json;

use std::io;
use std::fs::File;

use memmap::Mmap;

pub mod json;

pub use self::json::GoogleLocationHistory;

#[derive(Debug)]
pub enum HistoryError {
    DeserializeError(serde_json::Error),
    IOError(io::Error),
}

pub unsafe fn load_location_history(file: &File) -> Result<GoogleLocationHistory, HistoryError> {
    let mmap = Mmap::map(file).map_err(HistoryError::IOError)?;

    serde_json::from_slice(&mmap).map_err(HistoryError::DeserializeError)
}
