extern crate clap;
extern crate exif;
extern crate futures;
extern crate hyper;
extern crate image;
extern crate jpeg_decoder;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate tinyfiledialogs;
extern crate url;
extern crate yore;

#[macro_use]
extern crate serde_derive;

mod common;
mod server;

use std::fs::File;
use std::io::stdin;
use std::path::Path;

use clap::{App, Arg};
use yore::{get_location_suggestion, load_location_history, GoogleLocationHistory, PhotoError,
           PhotoLocation};

use common::{photo_paths, ApplicationError, exiv2_write_coordinates};
use server::Server;

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
                .required_unless("gui")
                .help("The path to a Google Location History JSON file"),
        )
        .arg(
            Arg::with_name("interpolate")
                .long("interpolate")
                .short("i")
                .help("Interpolate between locations if an exact match is not found"),
        )
        .arg(
            Arg::with_name("read-only")
                .long("read-only")
                .short("r")
                .help("Don't offer to save suggested locations"),
        )
        .arg(
            Arg::with_name("gui")
                .long("gui")
                .short("g")
                .help("Start a server for the browser-based GUI"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .default_value("8080")
                .help("The port that the GUI server should listen on"),
        )
        .arg(
            Arg::with_name("INPUT")
                .required_unless("gui")
                .index(1)
                .help("The image or a directory of images to suggest a location for"),
        )
        .get_matches();

    let photo_path = matches.value_of("INPUT").map(Path::new);
    let location_history_path = matches.value_of("location_history").map(Path::new);
    let interpolate = matches.is_present("interpolate");
    let read_only = matches.is_present("read-only");
    let use_gui = matches.is_present("gui");
    let gui_port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    if use_gui {
        let mut server = Server::new(gui_port, interpolate);

        if let Some(path) = photo_path {
            server.search_photos_path(path);
        }

        if let Some(path) = location_history_path {
            server.load_location_history(path).unwrap();
        }

        server.run().unwrap();
    } else {
        run_cli(
            photo_path.unwrap(),
            location_history_path.unwrap(),
            interpolate,
            read_only,
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
