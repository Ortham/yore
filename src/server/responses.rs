use std::path::{Path, PathBuf};

use rayon::prelude::*;
use yore::get_location_suggestion;
use yore::golo::GoogleLocationHistory;
use yore::{Photo, PhotoLocation};

use super::error::ServiceError;
use super::service::GuiServiceState;
use super::image::ImageDimensions;

#[derive(Serialize)]
pub struct RootPathResponse {
    #[serde(rename = "rootPath")]
    root_path: PathBuf,
}

impl RootPathResponse {
    pub fn new(state: &GuiServiceState) -> RootPathResponse {
        RootPathResponse { root_path: state.root_path().to_path_buf() }
    }
}

#[derive(Serialize)]
pub struct PhotosResponse {
    photos: Vec<ImageDimensions>,
}

impl PhotosResponse {
    pub fn new(state: &GuiServiceState) -> Result<PhotosResponse, ServiceError> {
        state
            .photo_paths()
            .par_iter()
            .map(|path| ImageDimensions::new(path))
            .collect::<Result<Vec<ImageDimensions>, ServiceError>>()
            .map(|photos| PhotosResponse { photos })
    }
}

#[derive(Serialize)]
pub struct FilteredPhotosResponse {
    photo_indices: Vec<usize>,
}

impl FilteredPhotosResponse {
    pub fn new(state: &GuiServiceState) -> FilteredPhotosResponse {
        let photo_indices: Vec<usize> = state
            .photo_paths()
            .par_iter()
            .enumerate()
            .filter_map(|(index, path)| {
                Photo::new(path).ok().and_then(
                    |photo| if photo.location().is_some() {
                        None
                    } else if state.location_history().contains(photo.timestamp()) {
                        Some(index)
                    } else {
                        None
                    },
                )
            })
            .collect();

        FilteredPhotosResponse { photo_indices }
    }
}

#[derive(Serialize)]
pub struct LocationsResponse {
    locations: Vec<LocationResponse>,
    start_index: usize,
    stop_index: usize,
}

impl LocationsResponse {
    pub fn new(
        state: &GuiServiceState,
        start_index: usize,
        stop_index: usize,
    ) -> Result<LocationsResponse, ServiceError> {
        state.photo_paths()[start_index..stop_index]
            .par_iter()
            .map(|path| {
                LocationResponse::new(path, &state.location_history(), state.interpolate())
            })
            .collect::<Result<Vec<LocationResponse>, ServiceError>>()
            .map(|locations| {
                LocationsResponse {
                    locations,
                    start_index,
                    stop_index,
                }
            })
    }
}

#[derive(Serialize)]
pub struct LocationResponse {
    path: PathBuf,

    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PhotoLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl LocationResponse {
    pub fn new(
        path: &Path,
        location_history: &GoogleLocationHistory,
        interpolate: bool,
    ) -> Result<LocationResponse, ServiceError> {
        let result = get_location_suggestion(&path, &location_history, interpolate)
            .map_err(|e| format!("{}", e));

        let (location, error) = match result {
            Ok(PhotoLocation::None) => (None, None),
            Ok(l) => (Some(l), None),
            Err(e) => (None, Some(e)),
        };

        Ok(LocationResponse {
            path: path.to_path_buf(),
            location,
            error,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use serde_json::to_string;
    use yore::golo::load_location_history;

    #[test]
    fn root_path_response_new_should_get_the_root_path() {
        let state = GuiServiceState::new(
            Path::new("tests/assets"),
            GoogleLocationHistory::default(),
            false,
        );
        let response = RootPathResponse::new(&state);

        assert_eq!(state.root_path(), response.root_path);
    }

    #[test]
    fn photos_response_new_should_get_data_for_all_found_photos() {
        let state = GuiServiceState::new(
            Path::new("tests/assets"),
            GoogleLocationHistory::default(),
            false,
        );
        let response = PhotosResponse::new(&state).unwrap();

        assert_eq!(
            "{\"photos\":[\
                {\"path\":\"tests/assets/photo.jpg\",\"height\":37,\"width\":55},\
                {\"path\":\"tests/assets/photo_rotated.jpg\",\"height\":50,\"width\":33},\
                {\"path\":\"tests/assets/photo_without_exif.jpg\",\"height\":37,\"width\":55},\
                {\"path\":\"tests/assets/photo_without_gps.jpg\",\"height\":37,\"width\":55},\
                {\
                    \"path\":\"tests/assets/photo_without_orientation.jpg\",\
                    \"height\":33,\
                    \"width\":50\
                },\
                {\
                    \"path\":\"tests/assets/photo_without_timestamp.jpg\",\
                    \"height\":37,\
                    \"width\":55\
                }\
            ]}",
            to_string(&response).unwrap().replace("\\\\", "/")
        );
    }

    #[test]
    fn filtered_photos_response_new_should_store_indices_of_photos_with_location_suggestions() {
        let history = unsafe {
            load_location_history(&File::open("tests/assets/location_history.json").unwrap())
                .unwrap()
        };
        let state = GuiServiceState::new(Path::new("tests/assets"), history, false);
        let response = FilteredPhotosResponse::new(&state);

        assert_eq!(1, response.photo_indices.len());
        assert_eq!(
            Path::new("tests/assets/photo_without_gps.jpg"),
            state.photo_paths()[response.photo_indices[0]],
        );
    }

    #[test]
    fn locations_response_new_should_get_locations_for_the_given_photo_index_range() {
        let state = GuiServiceState::new(
            Path::new("tests/assets"),
            GoogleLocationHistory::default(),
            false,
        );
        let response = LocationsResponse::new(&state, 1, 3).unwrap();

        assert_eq!(1, response.start_index);
        assert_eq!(3, response.stop_index);
        assert_eq!(2, response.locations.len());
        assert_eq!(state.photo_paths()[1], response.locations[0].path);
        assert_eq!(state.photo_paths()[2], response.locations[1].path);
    }

    #[test]
    fn location_response_new_should_set_an_error_message_if_passed_a_non_jpeg_file() {
        let path = Path::new("Cargo.toml");
        let history = GoogleLocationHistory::default();
        let response = LocationResponse::new(path, &history, false).unwrap();

        assert_eq!(path, response.path);
        assert!(response.location.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn location_response_new_should_set_only_a_path_for_empty_location_history() {
        let path = Path::new("tests/assets/photo_without_gps.jpg");
        let history = GoogleLocationHistory::default();
        let response = LocationResponse::new(path, &history, false).unwrap();

        assert_eq!(path, response.path);
        assert!(response.location.is_none());
        assert!(response.error.is_none());
    }

    #[test]
    fn get_location_suggestion_should_set_a_location_if_the_photo_has_gps_metadata() {
        let path = Path::new("tests/assets/photo.jpg");
        let history = GoogleLocationHistory::default();
        let response = LocationResponse::new(path, &history, false).unwrap();

        assert_eq!(path, response.path);
        assert!(response.location.is_some());
        assert!(response.error.is_none());
    }
}
