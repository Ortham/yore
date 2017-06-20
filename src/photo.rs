//#![deny(warnings)]

extern crate rexif;

use std::path;

use chrono::offset::TimeZone;
use chrono::offset::utc::UTC;
use chrono::format::ParseError;

use coordinates;

#[derive(Debug)]
pub struct Photo {
    pub path: path::PathBuf,
    pub timestamp: i64,
    pub location: Option<coordinates::Coordinates>,
}

#[derive(Debug)]
pub enum PhotoError {
    PathEncodingError,
    ExifError(rexif::ExifError),
    TimestampFormatError(ParseError),
    TimestampMissing,
}

impl Photo {
    pub fn new(path: &path::Path) -> Result<Photo, PhotoError> {
        let unicode_path = path.to_str().ok_or(PhotoError::PathEncodingError)?;

        let exif = rexif::parse_file(unicode_path).map_err(
            PhotoError::ExifError,
        )?;

        let mut date_time: Option<i64> = None;
        let mut latitude: Option<f64> = None;
        let mut longitude: Option<f64> = None;
        let mut latitude_sign: f64 = 1.0;
        let mut longitude_sign: f64 = 1.0;
        for entry in &exif.entries {
            match entry.tag {
                rexif::ExifTag::DateTimeOriginal |
                rexif::ExifTag::DateTime => {
                    if let rexif::TagValue::Ascii(ref x) = entry.value {
                        date_time = Some(
                            UTC.datetime_from_str(x, "%Y:%m:%d %T")
                                .map_err(PhotoError::TimestampFormatError)?
                                .timestamp(),
                        );
                    }
                }
                rexif::ExifTag::GPSLatitude => {
                    if let rexif::TagValue::URational(ref x) = entry.value {
                        latitude = Some(Photo::to_decimal_coordinate(x));
                    }
                }
                rexif::ExifTag::GPSLatitudeRef => {
                    match entry.value_more_readable.as_str() {
                        "S" => latitude_sign = -1.0,
                        _ => {}
                    }
                }
                rexif::ExifTag::GPSLongitude => {
                    if let rexif::TagValue::URational(ref x) = entry.value {
                        longitude = Some(Photo::to_decimal_coordinate(x));
                    }
                }
                rexif::ExifTag::GPSLongitudeRef => {
                    match entry.value_more_readable.as_str() {
                        "W" => longitude_sign = -1.0,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let location: Option<coordinates::Coordinates>;
        match (longitude, latitude) {
            (Some(longitude), Some(latitude)) => {
                location = Some(coordinates::Coordinates::new(
                    latitude * latitude_sign,
                    longitude * longitude_sign,
                ));
            }
            _ => location = None,
        }

        match date_time {
            None => Err(PhotoError::TimestampMissing),
            Some(timestamp) => {
                Ok(Photo {
                    path: path.to_path_buf(),
                    timestamp,
                    location,
                })
            }
        }
    }

    fn to_decimal_coordinate(dms: &[rexif::URational]) -> f64 {
        dms[0].value() + dms[1].value() / 60.0 + dms[2].value() / 3600.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn should_error_if_passed_a_path_that_does_not_exist() {
            let photo = Photo::new(path::Path::new("foo"));

            assert!(photo.is_err());
        }

        #[test]
        fn should_error_if_passed_a_path_that_is_not_an_image_with_exif_metadata() {
            let photo = Photo::new(path::Path::new("tests/assets/photo_without_exif.jpg"));

            assert!(photo.is_err());
        }

        #[test]
        fn should_error_if_passed_a_non_photo_path() {
            let photo = Photo::new(path::Path::new("Cargo.toml"));

            assert!(photo.is_err());
        }

        #[test]
        fn should_error_if_passed_an_image_with_no_exif_timestamp() {
            let photo = Photo::new(path::Path::new("tests/assets/photo_without_timestamp.jpg"));

            assert!(photo.is_err());
        }

        #[test]
        fn should_return_a_photo_object_with_the_image_timestamp_from_exif_metadata() {
            let photo = Photo::new(path::Path::new("tests/assets/photo_without_gps.jpg")).unwrap();

            assert_eq!(1473158321, photo.timestamp);
            assert_eq!(None, photo.location);
        }

        #[test]
        fn should_return_a_photo_object_with_the_image_timestamp_and_gps_from_exif_metadata() {
            let photo = Photo::new(path::Path::new("tests/assets/photo.jpg")).unwrap();
            let location = photo.location.unwrap();

            assert_eq!(1473158321, photo.timestamp);
            assert_eq!(38.76544, location.latitude);
            assert_eq!(-9.094802222222222, location.longitude);
        }
    }
}
