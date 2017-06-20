extern crate serde_json;

use std::io;
use std::path::Path;

use memmap::{Mmap, Protection};

pub mod json;

pub use self::json::GoogleLocationHistory;

#[derive(Debug)]
pub enum HistoryError {
    DeserializeError(serde_json::Error),
    IOError(io::Error),
}

pub unsafe fn load_location_history(path: &Path) -> Result<GoogleLocationHistory, HistoryError> {
    let mmap_view = Mmap::open_path(path, Protection::Read)
        .map_err(HistoryError::IOError)?
        .into_view();

    serde_json::from_slice(mmap_view.as_slice()).map_err(HistoryError::DeserializeError)
}
