//#![deny(warnings)]

extern crate rexif;

use std::io;
use std::path;

use coordinate;

pub struct Photo {
    pub path: path::PathBuf,
    pub timestamp: String,
    pub location: coordinate::Coordinate,
}

impl Photo {
    pub fn new(path: &path::Path) -> Result<Photo, io::Error> {
        let unicode_path = try!(path.to_str()
            .ok_or(io::Error::new(io::ErrorKind::Other,
                                  "Could not convert the input path to UTF-8")));

        let exif = try!(rexif::parse_file(unicode_path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.extra)));

        let mut date_time: String = String::new();
        let mut latitude: f64 = 0.0;
        let mut longitude: f64 = 0.0;
        let mut latitude_sign: f64 = 1.0;
        let mut longitude_sign: f64 = 1.0;
        for entry in &exif.entries {
            match entry.tag {
                rexif::ExifTag::DateTimeOriginal | rexif::ExifTag::DateTime =>  {
                    if let rexif::TagValue::Ascii(ref x) = entry.value {
                        date_time = x.clone();
                    }
                },
                rexif::ExifTag::GPSLatitude => {
                    if let rexif::TagValue::URational(ref x) = entry.value {
                        latitude = Photo::to_decimal_coordinate(x);
                    }
                },
                rexif::ExifTag::GPSLatitudeRef => {
                    match entry.value_more_readable.as_str() {
                        "S" => latitude_sign = -1.0,
                        _ => {}
                    }
                },
                rexif::ExifTag::GPSLongitude => {
                    if let rexif::TagValue::URational(ref x) = entry.value {
                        longitude = Photo::to_decimal_coordinate(x);
                    }
                },
                rexif::ExifTag::GPSLongitudeRef => {
                    match entry.value_more_readable.as_str() {
                        "W" => longitude_sign = -1.0,
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        latitude *= latitude_sign;
        longitude *= longitude_sign;

        if date_time.is_empty() {
            Err(io::Error::new(io::ErrorKind::Other, "No date taken metadata found"))
        } else {
            Ok(Photo {
                path: path.to_path_buf(),
                timestamp: date_time,
                location: coordinate::Coordinate::new(latitude, longitude),
            })
        }
    }

    fn to_decimal_coordinate(dms: &Vec<rexif::URational>) -> f64 {
        dms[0].value() + dms[1].value() / 60.0 + dms[2].value() / 3600.0
    }
}

#[cfg(test)]
mod tests {
    #[path = "./fixture.rs"]
    mod fixture;

    use super::*;

    use self::fixture::*;

    mod new {
        use super::*;

        #[test]
        #[should_panic]
        fn should_error_if_passed_a_path_that_does_not_exist() {
            Photo::new(path::Path::new("foo")).unwrap();
        }

        #[test]
        #[should_panic]
        fn should_error_if_passed_a_path_that_is_not_an_image_with_exif_metadata() {
            let temp_dir = super::setup().unwrap();

            Photo::new(temp_dir.path().join(SUBDIR_2).join(INACCURATE_FILENAME).as_path()).unwrap();
        }

        #[test]
        #[should_panic]
        fn should_error_if_passed_a_path_that_is_not_an_image_with_an_exif_timestamp() {
            let temp_dir = super::setup().unwrap();

            Photo::new(temp_dir.path().join(SUBDIR_1).join(JPEG_FILENAME).as_path()).unwrap();
        }

        #[test]
        fn should_return_a_photo_object_with_the_image_timestamp_from_exif_metadata() {
            let temp_dir = super::setup().unwrap();

            let photo = Photo::new(temp_dir.path().join(SUBDIR_2).join(JPG_FILENAME).as_path()).unwrap();

            assert_eq!("2003:12:14 12:01:44", photo.timestamp);
        }
    }
}
