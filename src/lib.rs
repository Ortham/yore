extern crate chrono;
extern crate exif;
extern crate memmap;
extern crate serde;
extern crate serde_json;
extern crate walkdir;

#[macro_use]
extern crate serde_derive;

mod coordinates;
mod golo;
mod photo;
mod suggestion_accuracy;

use std::path::Path;
use std::path::PathBuf;

use walkdir::WalkDir;

pub use coordinates::Coordinates;
pub use golo::{load_location_history, GoogleLocationHistory, HistoryError, Location};
pub use photo::Photo;
pub use photo::PhotoError;
pub use suggestion_accuracy::SuggestionAccuracy;

#[derive(Debug, PartialEq, Serialize)]
pub enum PhotoLocation {
    Existing(Coordinates),
    Suggested(Coordinates, SuggestionAccuracy),
    None,
}

fn is_jpeg_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    match path.extension() {
        Some(x) => x == "jpg" || x == "jpeg" || x == "JPG" || x == "JPEG",
        _ => false,
    }
}

pub fn find_jpegs(root_directory: &Path) -> Vec<PathBuf> {
    WalkDir::new(root_directory)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| is_jpeg_file(p.as_path()))
        .collect()
}

pub fn get_location_suggestion(
    path: &Path,
    location_history: &GoogleLocationHistory,
    interpolate: bool,
) -> Result<PhotoLocation, PhotoError> {
    let photo = Photo::new(path)?;

    if let Some(coordinates) = photo.gps_coordinates() {
        return Ok(PhotoLocation::Existing(coordinates.clone()));
    }

    let suggested_location = if interpolate {
        location_history.interpolate_location(photo.timestamp())
    } else {
        location_history
            .get_most_likely_location(photo.timestamp())
            .cloned()
    };

    match suggested_location {
        None => Ok(PhotoLocation::None),
        Some(suggested_location) => {
            let accuracy = SuggestionAccuracy::new(
                suggested_location.accuracy(),
                suggested_location.timestamp() - photo.timestamp(),
            );
            Ok(PhotoLocation::Suggested(
                suggested_location.coordinates(),
                accuracy,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use super::*;

    use std::fs::{copy, create_dir_all, File};

    use self::tempfile::tempdir;

    #[test]
    fn is_jpeg_file_should_return_false_for_a_file_that_does_not_exist() {
        assert!(!is_jpeg_file(Path::new("nonexistent")));
    }

    #[test]
    fn is_jpeg_file_should_return_false_for_a_file_that_does_not_have_a_jpg_or_jpeg_extension() {
        assert!(!is_jpeg_file(Path::new("Cargo.toml")));
    }

    #[test]
    fn is_jpeg_file_should_return_false_for_a_directory() {
        assert!(!is_jpeg_file(Path::new("Cargo.toml")));
    }

    #[test]
    fn is_jpeg_file_should_return_true_for_a_file_with_jpg_file_extension() {
        let tmp_dir = tempdir().unwrap();

        assert!(is_jpeg_file(Path::new("tests/assets/photo.jpg")));

        let jpg_file = tmp_dir.path().join("photo.JPG");
        copy("tests/assets/photo.jpg", &jpg_file).unwrap();

        assert!(is_jpeg_file(jpg_file.as_path()));
    }

    #[test]
    fn is_jpeg_file_should_return_true_for_a_file_with_jpeg_file_extension() {
        let tmp_dir = tempdir().unwrap();

        let jpeg_file = tmp_dir.path().join("photo.jpeg");
        copy("tests/assets/photo.jpg", &jpeg_file).unwrap();

        assert!(is_jpeg_file(jpeg_file.as_path()));

        let jpeg_file = tmp_dir.path().join("photo.JPEG");
        copy("tests/assets/photo.jpg", &jpeg_file).unwrap();

        assert!(is_jpeg_file(jpeg_file.as_path()));
    }

    #[test]
    fn find_jpegs_should_return_all_jpeg_files_in_directory_recusively() {
        let tmp_dir = tempdir().unwrap();
        let tmp_subdir = tmp_dir.path().join("subdir");

        create_dir_all(&tmp_subdir).unwrap();

        copy("Cargo.toml", tmp_dir.path().join("Cargo.toml")).unwrap();
        let jpeg_file = tmp_dir.path().join("photo.jpeg");
        copy("tests/assets/photo.jpg", &jpeg_file).unwrap();
        let jpg_file = tmp_subdir.join("photo.JPG");
        copy("tests/assets/photo.jpg", &jpg_file).unwrap();

        let jpegs = find_jpegs(tmp_dir.path());

        assert_eq!(vec![jpeg_file, jpg_file], jpegs);
    }

    #[test]
    fn get_location_suggestion_should_error_if_passed_a_non_jpeg_file() {
        let history = GoogleLocationHistory::default();
        let location = get_location_suggestion(Path::new("Cargo.toml"), &history, false);

        assert!(location.is_err());
    }

    #[test]
    fn get_location_suggestion_should_error_if_passed_a_jpeg_with_no_exif_metadata() {
        let history = GoogleLocationHistory::default();
        let path = Path::new("tests/assets/photo_without_exif.jpg");
        let location = get_location_suggestion(path, &history, false);

        assert!(location.is_err());
    }

    #[test]
    fn get_location_suggestion_should_error_if_passed_a_jpeg_with_no_timestamp_metadata() {
        let history = GoogleLocationHistory::default();
        let path = Path::new("tests/assets/photo_without_timestamp.jpg");
        let location = get_location_suggestion(path, &history, false);

        assert!(location.is_err());
    }

    #[test]
    fn get_location_suggestion_should_return_none_if_the_location_history_is_empty() {
        let history = GoogleLocationHistory::default();
        let path = Path::new("tests/assets/photo_without_gps.jpg");
        let location = get_location_suggestion(path, &history, false);

        assert_eq!(PhotoLocation::None, location.unwrap());
    }

    #[test]
    fn get_location_suggestion_should_return_existing_if_the_photo_has_gps_metadata() {
        let history = GoogleLocationHistory::default();
        let path = Path::new("tests/assets/photo.jpg");
        let location = get_location_suggestion(path, &history, false);

        assert_eq!(
            PhotoLocation::Existing(Coordinates::new(38.76544, -9.094802222222222)),
            location.unwrap()
        );
    }

    #[test]
    fn get_location_suggestion_should_return_suggested_if_a_suggestion_is_possible() {
        let history = unsafe {
            golo::load_location_history(&File::open("tests/assets/location_history.json").unwrap())
                .unwrap()
        };
        let path = Path::new("tests/assets/photo_without_gps.jpg");
        let location = get_location_suggestion(path, &history, false);

        assert_eq!(
            PhotoLocation::Suggested(
                Coordinates::new(52.0796733, 1.1965831),
                SuggestionAccuracy::new(18, -470321),
            ),
            location.unwrap()
        );
    }
}
