
extern crate chrono;
extern crate memmap;
extern crate serde;
extern crate serde_json;
extern crate walkdir;

#[macro_use]
extern crate serde_derive;

pub mod json;
pub mod photo;
mod coordinates;

use std::path::Path;
use std::path::PathBuf;

use memmap::{Mmap, Protection};
use walkdir::WalkDir;

pub use coordinates::Coordinates;
pub use photo::Photo;
pub use photo::PhotoError;

#[derive(Debug)]
pub enum HistoryError {
    DeserializeError(serde_json::Error),
    IOError(std::io::Error),
}

#[derive(Debug, PartialEq)]
pub enum PhotoLocation {
    Existing(Coordinates),
    Suggested(Coordinates, u16),
    None,
}

pub unsafe fn load_location_history(path: &Path) ->
    Result<json::GoogleLocationHistory, HistoryError> {
    let mmap_view = Mmap::open_path(path, Protection::Read)
            .map_err(HistoryError::IOError)?
            .into_view();

    serde_json::from_slice(mmap_view.as_slice())
        .map_err(HistoryError::DeserializeError)
}

pub fn is_jpeg_file(path: &Path) -> bool {
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
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| is_jpeg_file(p.as_path()))
        .collect()
}

pub fn get_location_suggestion(path: &Path, location_history: &json::GoogleLocationHistory)
    -> Result<PhotoLocation, PhotoError> {
    let photo = Photo::new(path)?;

    if let Some(location) = photo.location {
        return Ok(PhotoLocation::Existing(location));
    }

    let suggested_location = location_history.get_most_likely_location(photo.timestamp);

    match suggested_location {
        None => Ok(PhotoLocation::None),
        Some(suggested_location) => Ok(PhotoLocation::Suggested(suggested_location.coordinates(), suggested_location.accuracy)),
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;

    use std::fs::copy;
    use std::fs::create_dir_all;

    use self::tempdir::TempDir;

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
        let tmp_dir = TempDir::new("yore_test_").unwrap();

        assert!(is_jpeg_file(Path::new("tests/assets/photo.jpg")));

        let jpg_file = tmp_dir.path().join("photo.JPG");
        copy("tests/assets/photo.jpg", &jpg_file).unwrap();

        assert!(is_jpeg_file(jpg_file.as_path()));
    }

    #[test]
    fn is_jpeg_file_should_return_true_for_a_file_with_jpeg_file_extension() {
        let tmp_dir = TempDir::new("yore_test_").unwrap();

        let jpeg_file = tmp_dir.path().join("photo.jpeg");
        copy("tests/assets/photo.jpg", &jpeg_file).unwrap();

        assert!(is_jpeg_file(jpeg_file.as_path()));

        let jpeg_file = tmp_dir.path().join("photo.JPEG");
        copy("tests/assets/photo.jpg", &jpeg_file).unwrap();

        assert!(is_jpeg_file(jpeg_file.as_path()));
    }

    #[test]
    fn find_jpegs_should_return_all_jpeg_files_in_directory_recusively() {
        let tmp_dir = TempDir::new("yore_test_").unwrap();
        let tmp_subdir = tmp_dir.path().join("subdir");

        create_dir_all(&tmp_subdir).unwrap();

        copy("Cargo.toml", tmp_dir.path().join("Cargo.toml")).unwrap();
        let jpeg_file = tmp_dir.path().join("photo.jpeg");
        copy("tests/assets/photo.jpg", &jpeg_file).unwrap();
        let jpg_file = tmp_subdir.join("photo.JPG");
        copy("tests/assets/photo.jpg", &jpg_file).unwrap();

        let jpegs = find_jpegs(tmp_dir.path());

        assert_eq!(vec![
            jpeg_file,
            jpg_file,
        ], jpegs);
    }

    #[test]
    fn get_location_suggestion_should_error_if_passed_a_non_jpeg_file() {
        let history = json::GoogleLocationHistory::new();
        let location = get_location_suggestion(Path::new("Cargo.toml"), &history);

        assert!(location.is_err());
    }

    #[test]
    fn get_location_suggestion_should_error_if_passed_a_jpeg_with_no_exif_metadata() {
        let history = json::GoogleLocationHistory::new();
        let path = Path::new("tests/assets/photo_without_exif.jpg");
        let location = get_location_suggestion(path, &history);

        assert!(location.is_err());
    }

    #[test]
    fn get_location_suggestion_should_error_if_passed_a_jpeg_with_no_timestamp_metadata() {
        let history = json::GoogleLocationHistory::new();
        let path = Path::new("tests/assets/photo_without_timestamp.jpg");
        let location = get_location_suggestion(path, &history);

        assert!(location.is_err());
    }

    #[test]
    fn get_location_suggestion_should_return_none_if_the_location_history_is_empty() {
        let history = json::GoogleLocationHistory::new();
        let path = Path::new("tests/assets/photo_without_gps.jpg");
        let location = get_location_suggestion(path, &history);

        assert_eq!(PhotoLocation::None, location.unwrap());
    }

    #[test]
    fn get_location_suggestion_should_return_existing_if_the_photo_has_gps_metadata() {
        let history = json::GoogleLocationHistory::new();
        let path = Path::new("tests/assets/photo.jpg");
        let location = get_location_suggestion(path, &history);

        assert_eq!(PhotoLocation::Existing(Coordinates {
            latitude: 38.76544,
            longitude: -9.094802222222222,
        }), location.unwrap());
    }

    #[test]
    fn get_location_suggestion_should_return_suggested_if_a_suggestion_is_possible() {
        let history = unsafe {
            load_location_history(Path::new("tests/assets/location_history.json")).unwrap()
        };
        let path = Path::new("tests/assets/photo_without_gps.jpg");
        let location = get_location_suggestion(path, &history);

        assert_eq!(PhotoLocation::Suggested(Coordinates {
            latitude: 52.0567467,
            longitude: 1.1485831,
        }, 18), location.unwrap());
    }
}
