use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use hyper;
use yore::{Coordinates, find_jpegs};
use yore::golo::HistoryError;

pub fn photo_paths(root_path: &Path) -> Vec<PathBuf> {
    if root_path.is_file() {
        vec![root_path.to_path_buf()]
    } else if root_path.is_dir() {
        find_jpegs(root_path)
    } else {
        vec![]
    }
}

#[derive(Debug)]
pub enum ApplicationError {
    HistoryError(HistoryError),
    IoError(io::Error),
    ServerError(hyper::Error),
}

impl From<HistoryError> for ApplicationError {
    fn from(error: HistoryError) -> Self {
        ApplicationError::HistoryError(error)
    }
}

impl From<io::Error> for ApplicationError {
    fn from(error: io::Error) -> Self {
        ApplicationError::IoError(error)
    }
}

impl From<hyper::Error> for ApplicationError {
    fn from(error: hyper::Error) -> Self {
        ApplicationError::ServerError(error)
    }
}

pub fn exiv2_write_coordinates(path: &Path, coordinates: &Coordinates) -> io::Result<Output> {
    let latitude_degrees = dms_string(coordinates.latitude());
    let longitude_degrees = dms_string(coordinates.longitude());

    Command::new("exiv2")
        .arg("-k")
        .arg(format!(
            "-Mset Exif.GPSInfo.GPSLatitude {}",
            latitude_degrees
        ))
        .arg(format!(
            "-Mset Exif.GPSInfo.GPSLatitudeRef {}",
            coordinates.latitude_ref()
        ))
        .arg(format!(
            "-Mset Exif.GPSInfo.GPSLongitude {}",
            longitude_degrees
        ))
        .arg(format!(
            "-Mset Exif.GPSInfo.GPSLongitudeRef {}",
            coordinates.longitude_ref()
        ))
        .arg(path)
        .stderr(Stdio::inherit())
        .output()
}

fn dms_string(coordinate: f64) -> String {
    format!("{}/10000000 0/1 0/1", (coordinate * 1e7) as u32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dms_string_should_print_coordinate_in_exif_degrees_minutes_seconds_format() {
        assert_eq!("556382576/10000000 0/1 0/1", dms_string(55.6382576));
    }
}
