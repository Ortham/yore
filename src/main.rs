extern crate clap;
extern crate yore;

use std::fs::File;
use std::io;
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use clap::{Arg, App};

use yore::Coordinates;
use yore::find_jpegs;
use yore::get_location_suggestion;
use yore::golo::load_location_history;
use yore::PhotoError;
use yore::PhotoLocation;

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

    run_cli(photo_path, location_history_path, interpolate, read_only);
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

fn run_cli(root_path: &Path, location_history_path: &Path, interpolate: bool, read_only: bool) {
    let location_history_file = File::open(location_history_path).unwrap();
    let location_history = unsafe { load_location_history(&location_history_file).unwrap() };

    for photo_path in photo_paths(root_path) {
        let result = get_location_suggestion(&photo_path, &location_history, interpolate);

        print_location_result(&photo_path, &result);

        if let Ok(PhotoLocation::Suggested(location, _)) = result {
            if !read_only && should_write() {
                let output = exiv2_write_coordinates(&photo_path, &location).unwrap();

                if output.status.success() {
                    println!("Location saved for {}", photo_path.display());
                } else {
                    eprintln!(
                        "Error: Failed to save location for \"{}\"!",
                        photo_path.display()
                    );
                }
            }
        }
    }
}

fn print_location_result(path: &Path, location: &Result<PhotoLocation, PhotoError>) {
    println!("");
    match location {
        &Err(ref e) => {
            eprintln!("{:?}:", path);
            eprintln!("\tError loading photo: {:?}", e);
        }
        &Ok(PhotoLocation::Existing(ref location)) => {
            println!("{:?}:", path);
            println!("\tAlready has a location: {}", location);
        }
        &Ok(PhotoLocation::Suggested(ref location, ref accuracy)) => {
            println!("{:?}:", path);
            println!("\tSuggested location: {}", location);
            println!("\tSuggestion accuracy: {}", accuracy);
            println!("\tView on map: {}", location.map_url());
        }
        &Ok(PhotoLocation::None) => {
            println!("{:?}:\n\tNo suggested location found", path);
        }
    }
}

fn should_write() -> bool {
    println!("");
    println!("Save the suggested location to this image? (y/n)");

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect(
            "Couldn't read input line",
        );

        match input.trim() {
            "y" => return true,
            "n" => return false,
            _ => println!("Unrecognised input, please try again."),
        };
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
