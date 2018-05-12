use std::fs::File;
use std::io::stdin;
use std::path::Path;

use yore::golo::{load_location_history, GoogleLocationHistory};
use yore::{get_location_suggestion, PhotoError, PhotoLocation};

use common::{photo_paths, ApplicationError, exiv2_write_coordinates};

pub fn run_cli(
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
