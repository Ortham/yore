use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use exif;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    Untransformed,
    FlippedHorizontally,
    HalfRotated,
    FlippedVertically,
    QuarterRotatedAndFlippedHorizontally,
    ThreeQuarterRotated,
    QuarterRotatedAndFlippedVertically,
    QuarterRotated,
}

impl Orientation {
    pub fn read(path: &Path) -> Result<Orientation, exif::Error> {
        let file = File::open(path)?;

        let reader = match exif::Reader::new(&mut BufReader::new(&file)) {
            Ok(x) => x,
            Err(exif::Error::NotFound(_)) => return Ok(Orientation::Untransformed),
            Err(e) => return Err(e),
        };

        let orientation = reader
            .get_field(exif::Tag::Orientation, false)
            .map(|field| {
                if let exif::Value::Short(ref x) = field.value {
                    Orientation::from_exif_value(x[0])
                } else {
                    Orientation::Untransformed
                }
            })
            .unwrap_or(Orientation::Untransformed);

        Ok(orientation)
    }

    fn from_exif_value(exif_value: u16) -> Orientation {
        match exif_value {
            2 => Orientation::FlippedHorizontally,
            3 => Orientation::HalfRotated,
            4 => Orientation::FlippedVertically,
            5 => Orientation::QuarterRotatedAndFlippedHorizontally,
            6 => Orientation::ThreeQuarterRotated,
            7 => Orientation::QuarterRotatedAndFlippedVertically,
            8 => Orientation::QuarterRotated,
            _ => Orientation::Untransformed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_should_return_untransformed_for_an_image_without_exif_metadata() {
        let path = Path::new("tests/assets/photo_without_exif.jpg");
        let orientation = Orientation::read(path).unwrap();

        assert_eq!(Orientation::Untransformed, orientation);
    }

    #[test]
    fn read_should_return_untransformed_for_an_image_without_orientation_metadata() {
        let path = Path::new("tests/assets/photo_without_orientation.jpg");
        let orientation = Orientation::read(path).unwrap();

        assert_eq!(Orientation::Untransformed, orientation);
    }

    #[test]
    fn read_should_return_quarter_rotated_for_a_photo_taken_with_that_orientation() {
        let path = Path::new("tests/assets/photo_rotated.jpg");
        let orientation = Orientation::read(path).unwrap();

        assert_eq!(Orientation::QuarterRotated, orientation);
    }

    #[test]
    fn from_exif_value_should_construct_from_u16_values_correctly() {
        assert_eq!(Orientation::Untransformed, Orientation::from_exif_value(0));
        assert_eq!(Orientation::Untransformed, Orientation::from_exif_value(1));
        assert_eq!(
            Orientation::FlippedHorizontally,
            Orientation::from_exif_value(2)
        );
        assert_eq!(Orientation::HalfRotated, Orientation::from_exif_value(3));
        assert_eq!(
            Orientation::FlippedVertically,
            Orientation::from_exif_value(4)
        );
        assert_eq!(
            Orientation::QuarterRotatedAndFlippedHorizontally,
            Orientation::from_exif_value(5)
        );
        assert_eq!(
            Orientation::ThreeQuarterRotated,
            Orientation::from_exif_value(6)
        );
        assert_eq!(
            Orientation::QuarterRotatedAndFlippedVertically,
            Orientation::from_exif_value(7)
        );
        assert_eq!(Orientation::QuarterRotated, Orientation::from_exif_value(8));
        assert_eq!(Orientation::Untransformed, Orientation::from_exif_value(9));
    }
}
