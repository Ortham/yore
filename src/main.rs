extern crate clap;
extern crate yore;

mod cli;

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use clap::{Arg, App};
use yore::{Coordinates, find_jpegs};
use yore::golo::HistoryError;

use cli::run_cli;

fn main() {
    let matches = App::new("yore")
        .version(env!("CARGO_PKG_VERSION"))
        .about(
            "Yore uses an exported Google Location History JSON file to suggest locations for
            images",
        )
        .author("Oliver Hamlet")
        .arg(
            Arg::with_name("location_history")
                .long("locations")
                .short("l")
                .value_name("FILE")
                .takes_value(true)
                .required(true)
                .help("The path to a Google Location History JSON file"),
        )
        .arg(
            Arg::with_name("interpolate")
                .long("interpolate")
                .short("i")
                .help(
                    "Interpolate between locations if an exact match is not found",
                ),
        )
        .arg(
            Arg::with_name("read-only")
                .long("read-only")
                .short("r")
                .help("Don't offer to save suggested locations"),
        )
        .arg(Arg::with_name("INPUT").required(true).index(1).help(
            "The image or a directory of images to suggest a location for",
        ))
        .get_matches();

    let photo_path = Path::new(matches.value_of("INPUT").unwrap());
    let location_history_path = Path::new(matches.value_of("location_history").unwrap());
    let interpolate = matches.is_present("interpolate");
    let read_only = matches.is_present("read-only");

    run_cli(photo_path, location_history_path, interpolate, read_only).unwrap();
}

fn photo_paths(root_path: &Path) -> Vec<PathBuf> {
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

fn exiv2_write_coordinates(path: &Path, coordinates: &Coordinates) -> io::Result<Output> {
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
