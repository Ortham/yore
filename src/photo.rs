//#![deny(warnings)]
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::format::ParseError;
use chrono::offset::TimeZone;
use chrono::offset::Utc;

use exif;
use exif::Tag;

use coordinates::Coordinates;

#[derive(Debug)]
pub struct Photo {
    path: PathBuf,
    timestamp: i64,
    location: Option<Coordinates>,
}

#[derive(Debug)]
pub enum PhotoError {
    ExifError(exif::Error),
    IoError(io::Error),
    TimestampFormatError(ParseError),
    TimestampMissing,
}

impl From<exif::Error> for PhotoError {
    fn from(error: exif::Error) -> Self {
        PhotoError::ExifError(error)
    }
}

impl From<io::Error> for PhotoError {
    fn from(error: io::Error) -> Self {
        PhotoError::IoError(error)
    }
}

impl From<ParseError> for PhotoError {
    fn from(error: ParseError) -> Self {
        PhotoError::TimestampFormatError(error)
    }
}

impl fmt::Display for PhotoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PhotoError::ExifError(ref x) => x.fmt(f),
            PhotoError::IoError(ref x) => x.fmt(f),
            PhotoError::TimestampFormatError(ref x) => x.fmt(f),
            PhotoError::TimestampMissing => write!(f, "The image has no timestamp metadata"),
        }
    }
}

impl error::Error for PhotoError {
    fn description(&self) -> &str {
        match *self {
            PhotoError::ExifError(ref x) => x.description(),
            PhotoError::IoError(ref x) => x.description(),
            PhotoError::TimestampFormatError(ref x) => x.description(),
            PhotoError::TimestampMissing => "The image has no timestamp metadata",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PhotoError::ExifError(ref x) => Some(x),
            PhotoError::IoError(ref x) => Some(x),
            PhotoError::TimestampFormatError(ref x) => Some(x),
            _ => None,
        }
    }
}

impl Photo {
    pub fn new(path: &Path) -> Result<Photo, PhotoError> {
        let file = fs::File::open(path)?;

        let reader = exif::Reader::new(&mut io::BufReader::new(&file))?;

        let mut date_time: Option<i64> = None;
        let mut latitude: Option<f64> = None;
        let mut longitude: Option<f64> = None;
        let mut latitude_sign: f64 = 1.0;
        let mut longitude_sign: f64 = 1.0;
        for field in reader.fields() {
            match field.tag {
                Tag::DateTimeOriginal | Tag::DateTime => {
                    if let exif::Value::Ascii(_) = field.value {
                        let string_value = format!("{}", field.value.display_as(field.tag));
                        date_time = Some(
                            Utc.datetime_from_str(string_value.as_str(), "%F %T")?
                                .timestamp(),
                        );
                    }
                }
                Tag::GPSLatitude => {
                    if let exif::Value::Rational(ref x) = field.value {
                        latitude = Some(Photo::to_decimal_coordinate(x));
                    }
                }
                Tag::GPSLatitudeRef => {
                    let string_value = format!("{}", field.value.display_as(field.tag));
                    match string_value.as_str() {
                        "S" => latitude_sign = -1.0,
                        _ => {}
                    }
                }
                Tag::GPSLongitude => {
                    if let exif::Value::Rational(ref x) = field.value {
                        longitude = Some(Photo::to_decimal_coordinate(x));
                    }
                }
                Tag::GPSLongitudeRef => {
                    let string_value = format!("{}", field.value.display_as(field.tag));
                    match string_value.as_str() {
                        "W" => longitude_sign = -1.0,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let location: Option<Coordinates>;
        match (longitude, latitude) {
            (Some(longitude), Some(latitude)) => {
                location = Some(Coordinates::new(
                    latitude * latitude_sign,
                    longitude * longitude_sign,
                ));
            }
            _ => location = None,
        }

        match date_time {
            None => Err(PhotoError::TimestampMissing),
            Some(timestamp) => Ok(Photo {
                path: path.to_path_buf(),
                timestamp,
                location,
            }),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn location(&self) -> Option<&Coordinates> {
        self.location.as_ref()
    }

    fn to_decimal_coordinate(dms: &[exif::Rational]) -> f64 {
        dms[0].to_f64() + dms[1].to_f64() / 60.0 + dms[2].to_f64() / 3600.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn should_error_if_passed_a_path_that_does_not_exist() {
            let photo = Photo::new(Path::new("foo"));

            assert!(photo.is_err());
        }

        #[test]
        fn should_error_if_passed_a_path_that_is_not_an_image_with_exif_metadata() {
            let photo = Photo::new(Path::new("tests/assets/photo_without_exif.jpg"));

            assert!(photo.is_err());
        }

        #[test]
        fn should_error_if_passed_a_non_photo_path() {
            let photo = Photo::new(Path::new("Cargo.toml"));

            assert!(photo.is_err());
        }

        #[test]
        fn should_error_if_passed_an_image_with_no_exif_timestamp() {
            let photo = Photo::new(Path::new("tests/assets/photo_without_timestamp.jpg"));

            assert!(photo.is_err());
        }

        #[test]
        fn should_return_a_photo_object_with_the_image_timestamp_from_exif_metadata() {
            let photo = Photo::new(Path::new("tests/assets/photo_without_gps.jpg")).unwrap();

            assert_eq!(1473158321, photo.timestamp);
            assert_eq!(None, photo.location);
        }

        #[test]
        fn should_return_a_photo_object_with_the_image_timestamp_and_gps_from_exif_metadata() {
            let photo = Photo::new(Path::new("tests/assets/photo.jpg")).unwrap();
            let location = photo.location.unwrap();

            assert_eq!(1473158321, photo.timestamp);
            assert_eq!(38.76544, location.latitude());
            assert_eq!(-9.094802222222222, location.longitude());
        }
    }
}
