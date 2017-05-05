extern crate clap;
extern crate yore;

use std::path::{Path, PathBuf};

use clap::{Arg, App};

use yore::find_jpegs;
use yore::get_location_suggestion;
use yore::golo::load_location_history;
use yore::PhotoError;
use yore::PhotoLocation;

fn main() {
    let matches = App::new("yore")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Yore uses an exported Google Location History JSON file to suggest locations for
            images")
        .author("Oliver Hamlet")
        .arg(Arg::with_name("location_history")
                 .long("locations")
                 .short("l")
                 .value_name("FILE")
                 .takes_value(true)
                 .required(true)
                 .help("The path to a Google Location History JSON file"))
        .arg(Arg::with_name("INPUT")
                 .required(true)
                 .index(1)
                 .help("The image or a directory of images to suggest a location for"))
        .get_matches();

    let photo_path = Path::new(matches.value_of("INPUT").unwrap());
    let photo_paths = photo_paths(photo_path);

    let location_history_path = Path::new(matches.value_of("location_history").unwrap());
    let location_history = unsafe { load_location_history(location_history_path).unwrap() };

    for photo_path in photo_paths {
        let location = get_location_suggestion(photo_path.as_path(), &location_history);
        print_location_suggestion(photo_path.as_path(), location);
    }
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

fn print_location_suggestion(path: &Path, location: Result<PhotoLocation, PhotoError>) {
    match location {
        Err(e) => {
            println!("{:?}:", path);
            println!("\tError loading photo: {:?}", e);
        }
        Ok(PhotoLocation::Existing(location)) => {
            println!("{:?}:", path);
            println!("\tAlready has a location: {}", location);
        }
        Ok(PhotoLocation::Suggested(location, accuracy)) => {
            println!("{:?}:", path);
            println!("\tSuggested location: {}", location);
            println!("\tSuggestion accuracy: {}", accuracy);
            println!("\tView on map: {}", location.map_url());
        }
        Ok(PhotoLocation::None) => {
            println!("{:?}:\n\tNo suggested location found", path);
        }
    }
}
