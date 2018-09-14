extern crate actix_web;
extern crate directories;
extern crate exif;
extern crate futures;
extern crate image;
extern crate jpeg_decoder;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate tinyfiledialogs;
extern crate yore;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

mod common;

use std::fs::File;
use std::io::stdin;
use std::path::{Path, PathBuf};

use structopt::StructOpt;
use yore::{
    get_location_suggestion, load_location_history, GoogleLocationHistory, PhotoError,
    PhotoLocation,
};

use common::{exiv2_write_coordinates, photo_paths, server::Server, ApplicationError};

#[derive(StructOpt)]
#[structopt(
    name = "yore",
    about = "Yore uses an exported Google Location History JSON file to suggest locations for
            images"
)]
struct Options {
    #[structopt(
        short = "l",
        long = "locations",
        parse(from_os_str),
        required_unless = "use_gui",
        help = "The path to a Google Location History JSON file"
    )]
    location_history_path: Option<PathBuf>,

    #[structopt(
        short = "i",
        long = "interpolate",
        help = "Interpolate between locations if an exact match is not found"
    )]
    interpolate: bool,

    #[structopt(
        short = "r",
        long = "read-only",
        help = "Don't offer to save suggested locations"
    )]
    read_only: bool,

    #[structopt(
        short = "g",
        long = "gui",
        help = "Start a server for the browser-based GUI"
    )]
    use_gui: bool,

    #[structopt(
        short = "p",
        long = "port",
        default_value = "8080",
        help = "The port that the GUI server should listen on"
    )]
    port: u16,

    #[structopt(
        parse(from_os_str),
        required_unless = "use_gui",
        help = "The image or a directory of images to suggest a location for"
    )]
    photo_path: Option<PathBuf>,
}

fn main() {
    let options = Options::from_args();

    if options.use_gui {
        let mut server = Server::new(options.port, options.interpolate);

        if let Some(path) = options.photo_path {
            server.search_photos_path(&path);
        }

        if let Some(path) = options.location_history_path {
            server.load_location_history(&path).unwrap();
        }

        server.run().unwrap();
    } else {
        run_cli(
            &options.photo_path.unwrap(),
            &options.location_history_path.unwrap(),
            options.interpolate,
            options.read_only,
        ).unwrap();
    }
}

fn run_cli(
    root_path: &Path,
    location_history_path: &Path,
    interpolate: bool,
    read_only: bool,
) -> Result<(), ApplicationError> {
    let location_history_file = File::open(location_history_path)?;
    let location_history = unsafe { load_location_history(&location_history_file)? };

    for photo_path in photo_paths(root_path) {
        process_photo(&photo_path, &location_history, interpolate, read_only)?;
    }

    Ok(())
}

fn process_photo(
    photo_path: &Path,
    location_history: &GoogleLocationHistory,
    interpolate: bool,
    read_only: bool,
) -> Result<(), ApplicationError> {
    let result = get_location_suggestion(&photo_path, &location_history, interpolate);

    print_location_result(&photo_path, &result);

    if let Ok(PhotoLocation::Suggested(location, _)) = result {
        if !read_only && should_write() {
            let output = exiv2_write_coordinates(&photo_path, &location)?;

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

    Ok(())
}

fn print_location_result(path: &Path, location: &Result<PhotoLocation, PhotoError>) {
    println!();
    match location {
        Err(ref e) => {
            eprintln!("{:?}:", path);
            eprintln!("\tError loading photo: {:?}", e);
        }
        Ok(PhotoLocation::Existing(ref location)) => {
            println!("{:?}:", path);
            println!("\tAlready has a location: {}", location);
        }
        Ok(PhotoLocation::Suggested(ref location, ref accuracy)) => {
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

fn should_write() -> bool {
    println!();
    println!("Save the suggested location to this image? (y/n)");

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Couldn't read input line");

        match input.trim() {
            "y" => return true,
            "n" => return false,
            _ => println!("Unrecognised input, please try again."),
        };
    }
}
