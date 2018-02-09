use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use image;
use jpeg_decoder;

use super::error::ServiceError;
use super::orientation::Orientation;

#[derive(Serialize)]
pub struct ImageDimensions {
    path: PathBuf,
    height: u16,
    width: u16,
}

impl ImageDimensions {
    pub fn new(path: &Path) -> Result<ImageDimensions, ServiceError> {
        let orientation = Orientation::read(&path)?;

        let file = File::open(path)?;
        let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(&file));

        decoder.read_info()?;

        let image_info = decoder.info().expect("image had no dimensions");
        let (width, height) = viewing_dimensions(image_info, orientation);

        Ok(ImageDimensions {
            path: path.to_path_buf(),
            height,
            width,
        })
    }
}

pub fn thumbnail(path: &Path, max_width: u32, max_height: u32) -> Result<Vec<u8>, ServiceError> {
    let orientation = Orientation::read(&path)?;

    let mut content: Vec<u8> = Vec::new();

    let image = image::open(&path)?;
    let mut thumbnail = image.resize(max_width, max_height, image::FilterType::Lanczos3);
    thumbnail = fix_image_orientation(thumbnail, orientation);

    thumbnail.save(&mut content, image::ImageFormat::JPEG)?;

    Ok(content)
}

pub fn oriented_image(path: &Path) -> Result<Vec<u8>, ServiceError> {
    let image = image::open(&path)?;
    let orientation = Orientation::read(&path)?;
    let image = fix_image_orientation(image, orientation);

    let mut content: Vec<u8> = Vec::new();
    image.save(&mut content, image::ImageFormat::JPEG)?;

    Ok(content)
}

fn viewing_dimensions(image_info: jpeg_decoder::ImageInfo, orientation: Orientation) -> (u16, u16) {
    match orientation {
        Orientation::Untransformed |
        Orientation::FlippedHorizontally |
        Orientation::HalfRotated |
        Orientation::FlippedVertically => (image_info.width, image_info.height),
        _ => (image_info.height, image_info.width),
    }
}

fn fix_image_orientation(
    image: image::DynamicImage,
    orientation: Orientation,
) -> image::DynamicImage {
    match orientation {
        Orientation::Untransformed => image,
        Orientation::FlippedHorizontally => image.fliph(),
        Orientation::HalfRotated => image.rotate180(),
        Orientation::FlippedVertically => image.flipv(),
        Orientation::QuarterRotatedAndFlippedHorizontally => image.rotate90().fliph(),
        Orientation::ThreeQuarterRotated => image.rotate90(),
        Orientation::QuarterRotatedAndFlippedVertically => image.fliph().rotate90(),
        Orientation::QuarterRotated => image.rotate270(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use image::{GenericImage, Pixel};

    fn test_image_and_pixel() -> (image::DynamicImage, image::Rgba<u8>) {
        let mut img = image::DynamicImage::ImageLuma8(image::ImageBuffer::new(5, 10));
        let pixel = image::Rgba::from_channels(1, 1, 1, 255);
        img.put_pixel(1, 0, pixel.clone());

        (img, pixel)
    }

    fn image_info() -> jpeg_decoder::ImageInfo {
        jpeg_decoder::ImageInfo {
            width: 1,
            height: 2,
            pixel_format: jpeg_decoder::PixelFormat::L8,
        }
    }

    #[test]
    fn image_dimensions_new_should_read_image_dimensions() {
        let path = Path::new("tests/assets/photo.jpg");
        let image = ImageDimensions::new(path).unwrap();

        assert_eq!(path, image.path);
        assert_eq!(55, image.width);
        assert_eq!(37, image.height);
    }

    #[test]
    fn image_dimensions_new_should_rotate_image_dimensions_to_viewing_orientation() {
        let path = Path::new("tests/assets/photo_rotated.jpg");
        let image = ImageDimensions::new(path).unwrap();

        assert_eq!(path, image.path);
        assert_eq!(33, image.width);
        assert_eq!(50, image.height);
    }

    #[test]
    fn thumbnail_should_scale_image_to_given_width_or_height() {
        let path = Path::new("tests/assets/photo.jpg");
        let thumbnail = thumbnail(path, 500, 500).unwrap();
        let image = image::load_from_memory(&thumbnail).unwrap();

        assert_eq!(500, image.width());
        assert_eq!(336, image.height());
    }

    #[test]
    fn thumbnail_should_rotate_image_according_to_its_orientation() {
        let path = Path::new("tests/assets/photo_rotated.jpg");
        let thumbnail = thumbnail(path, 500, 500).unwrap();
        let image = image::load_from_memory(&thumbnail).unwrap();

        assert_eq!(330, image.width());
        assert_eq!(500, image.height());
    }

    #[test]
    fn oriented_image_should_rotate_image_according_to_its_orientation() {
        let path = Path::new("tests/assets/photo_rotated.jpg");
        let oriented_image = oriented_image(path).unwrap();
        let image = image::load_from_memory(&oriented_image).unwrap();

        assert_eq!(33, image.width());
        assert_eq!(50, image.height());
    }

    #[test]
    fn viewing_dimensions_should_be_unchanged_if_orientation_has_same_axes() {
        let image_info = image_info();

        let (width, height) = viewing_dimensions(image_info.clone(), Orientation::Untransformed);
        assert_eq!(image_info.height, height);
        assert_eq!(image_info.width, width);

        let (width, height) =
            viewing_dimensions(image_info.clone(), Orientation::FlippedHorizontally);
        assert_eq!(image_info.height, height);
        assert_eq!(image_info.width, width);

        let (width, height) = viewing_dimensions(image_info.clone(), Orientation::HalfRotated);
        assert_eq!(image_info.height, height);
        assert_eq!(image_info.width, width);

        let (width, height) =
            viewing_dimensions(image_info.clone(), Orientation::FlippedVertically);
        assert_eq!(image_info.height, height);
        assert_eq!(image_info.width, width);
    }

    #[test]
    fn viewing_dimensions_should_flip_width_and_height_if_orientation_has_flipped_axes() {
        let image_info = image_info();

        let (width, height) = viewing_dimensions(
            image_info.clone(),
            Orientation::QuarterRotatedAndFlippedHorizontally,
        );
        assert_eq!(image_info.width, height);
        assert_eq!(image_info.height, width);

        let (width, height) =
            viewing_dimensions(image_info.clone(), Orientation::ThreeQuarterRotated);
        assert_eq!(image_info.width, height);
        assert_eq!(image_info.height, width);

        let (width, height) = viewing_dimensions(
            image_info.clone(),
            Orientation::QuarterRotatedAndFlippedVertically,
        );
        assert_eq!(image_info.width, height);
        assert_eq!(image_info.height, width);

        let (width, height) = viewing_dimensions(image_info.clone(), Orientation::QuarterRotated);
        assert_eq!(image_info.width, height);
        assert_eq!(image_info.height, width);
    }

    #[test]
    fn fix_image_orientation_should_not_modify_an_untransformed_image() {
        let (img, pixel) = test_image_and_pixel();
        let fixed_image = fix_image_orientation(img, Orientation::Untransformed);

        assert_eq!(pixel, fixed_image.get_pixel(1, 0));
    }

    #[test]
    fn fix_image_orientation_should_unflip_a_horizontally_flipped_image() {
        let (img, pixel) = test_image_and_pixel();
        let fixed_image = fix_image_orientation(img, Orientation::FlippedHorizontally);

        assert_eq!(pixel, fixed_image.get_pixel(3, 0));
    }

    #[test]
    fn fix_image_orientation_should_half_rotate_a_half_rotated_image() {
        let (img, pixel) = test_image_and_pixel();
        let fixed_image = fix_image_orientation(img, Orientation::HalfRotated);

        assert_eq!(pixel, fixed_image.get_pixel(3, 9));
    }

    #[test]
    fn fix_image_orientation_should_unflip_a_vertically_flipped_image() {
        let (img, pixel) = test_image_and_pixel();
        let fixed_image = fix_image_orientation(img, Orientation::FlippedVertically);

        assert_eq!(pixel, fixed_image.get_pixel(1, 9));
    }

    #[test]
    fn fix_image_orientation_should_rotate_and_unflip_a_quarter_rotated_and_h_flipped_image() {
        let (img, pixel) = test_image_and_pixel();
        let fixed_image =
            fix_image_orientation(img, Orientation::QuarterRotatedAndFlippedHorizontally);

        assert_eq!(pixel, fixed_image.get_pixel(0, 1));
    }

    #[test]
    fn fix_image_orientation_should_rotate_a_three_quarter_rotated_image() {
        let (img, pixel) = test_image_and_pixel();
        let fixed_image = fix_image_orientation(img, Orientation::ThreeQuarterRotated);

        assert_eq!(pixel, fixed_image.get_pixel(9, 1));
    }

    #[test]
    fn fix_image_orientation_should_rotate_and_unflip_a_quarter_rotated_and_v_flipped_image() {
        let (img, pixel) = test_image_and_pixel();
        let fixed_image =
            fix_image_orientation(img, Orientation::QuarterRotatedAndFlippedVertically);

        assert_eq!(pixel, fixed_image.get_pixel(9, 3));
    }

    #[test]
    fn fix_image_orientation_should_rotate_a_quarter_rotated_image() {
        let (img, pixel) = test_image_and_pixel();
        let fixed_image = fix_image_orientation(img, Orientation::QuarterRotated);

        assert_eq!(pixel, fixed_image.get_pixel(0, 3));
    }
}
