use std::error;
use std::fmt;
use std::io;
use std::sync::PoisonError;

use actix_web::error::ResponseError;
use actix_web::{self, http, HttpResponse};
use exif;
use image;
use jpeg_decoder;
use yore::HistoryError;

#[derive(Debug)]
pub enum ServiceError {
    IoError(io::Error),
    ImageError(image::ImageError),
    ImageSizeError,
    ImageFormatError(String),
    ImageUnsupportedError(jpeg_decoder::UnsupportedFeature),
    ExifError(exif::Error),
    HistoryError(HistoryError),
    PoisonError,
    ActixError(actix_web::Error),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::IoError(e) => e.fmt(f),
            ServiceError::ImageError(e) => e.fmt(f),
            ServiceError::ImageSizeError => write!(f, "An internal error occurred while decoding the image."),
            ServiceError::ImageFormatError(e) => write!(f, "The image is not formatted properly: {}", e),
            ServiceError::ImageUnsupportedError(x) => write!(f, "The image makes use of a JPEG feature not (currently) supported by this library: {:?}", x),
            ServiceError::ExifError(e) => e.fmt(f),
            ServiceError::HistoryError(_) => write!(f, "Couldn't load location history"),
            ServiceError::PoisonError => write!(f, "Poisoned mutex"),
            ServiceError::ActixError(e) => e.fmt(f),
        }
    }
}

impl error::Error for ServiceError {
    fn description(&self) -> &str {
        match self {
            ServiceError::IoError(e) => e.description(),
            ServiceError::ImageError(e) => e.description(),
            ServiceError::ImageSizeError => "An internal error occurred while decoding the image.",
            ServiceError::ImageFormatError(_) => "The image is not formatted properly.",
            ServiceError::ImageUnsupportedError(_) => {
                "The image makes use of a JPEG feature not (currently) supported by this library."
            }
            ServiceError::ExifError(e) => e.description(),
            ServiceError::HistoryError(_) => "Couldn't load location history",
            ServiceError::PoisonError => "Poisoned mutex",
            ServiceError::ActixError(_) => "Unknown actix error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            ServiceError::IoError(e) => Some(e),
            ServiceError::ImageError(e) => Some(e),
            ServiceError::ImageSizeError => None,
            ServiceError::ImageFormatError(_) => None,
            ServiceError::ImageUnsupportedError(_) => None,
            ServiceError::ExifError(e) => Some(e),
            ServiceError::HistoryError(_) => None,
            ServiceError::PoisonError => None,
            ServiceError::ActixError(_) => None,
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        println!("Error handling request: {:?}", self);
        match self {
            ServiceError::IoError(e) if e.kind() == io::ErrorKind::NotFound => {
                HttpResponse::NotFound().finish()
            }
            ServiceError::ActixError(e) => e.as_response_error().error_response(),
            e => {
                HttpResponse::with_body(http::StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))
            }
        }
    }
}

impl From<actix_web::Error> for ServiceError {
    fn from(error: actix_web::Error) -> Self {
        ServiceError::ActixError(error)
    }
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

impl From<HistoryError> for ServiceError {
    fn from(error: HistoryError) -> Self {
        ServiceError::HistoryError(error)
    }
}

impl<T> From<PoisonError<T>> for ServiceError {
    fn from(_error: PoisonError<T>) -> Self {
        ServiceError::PoisonError
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
