
use std::io;
use std::num::ParseIntError;

use exif;
use image;
use jpeg_decoder;
use serde_json;
use url;

#[derive(Debug)]
pub enum ServiceError {
    IoError(io::Error),
    UrlParseError(url::ParseError),
    ImageError(image::ImageError),
    ImageSizeError,
    ImageFormatError(String),
    ImageUnsupportedError(jpeg_decoder::UnsupportedFeature),
    MissingQueryParameter(&'static str),
    QueryParameterParseError(ParseIntError),
    JsonError(serde_json::Error),
    ExifError(exif::Error),
}

impl From<exif::Error> for ServiceError {
    fn from(error: exif::Error) -> Self {
        ServiceError::ExifError(error)
    }
}

impl From<io::Error> for ServiceError {
    fn from(error: io::Error) -> Self {
        ServiceError::IoError(error)
    }
}

impl From<url::ParseError> for ServiceError {
    fn from(error: url::ParseError) -> Self {
        ServiceError::UrlParseError(error)
    }
}

impl From<image::ImageError> for ServiceError {
    fn from(error: image::ImageError) -> Self {
        ServiceError::ImageError(error)
    }
}

impl From<jpeg_decoder::Error> for ServiceError {
    fn from(error: jpeg_decoder::Error) -> Self {
        use jpeg_decoder::Error;
        match error {
            Error::Format(x) => ServiceError::ImageFormatError(x),
            Error::Unsupported(x) => ServiceError::ImageUnsupportedError(x),
            Error::Io(x) => ServiceError::IoError(x),
            Error::Internal(_) => ServiceError::ImageSizeError,
        }
    }
}

impl From<ParseIntError> for ServiceError {
    fn from(error: ParseIntError) -> Self {
        ServiceError::QueryParameterParseError(error)
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(error: serde_json::Error) -> Self {
        ServiceError::JsonError(error)
    }
}
