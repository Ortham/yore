use std::path::{Path, PathBuf};

use rayon::prelude::*;
use yore::get_location_suggestion;
use yore::{Photo, PhotoLocation};

use super::error::ServiceError;
use super::service::GuiServiceState;
use super::image::ImageDimensions;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RootPathResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    root_path: Option<PathBuf>,
}

impl RootPathResponse {
    pub fn new(state: &GuiServiceState) -> RootPathResponse {
        RootPathResponse { root_path: state.root_path().cloned() }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationHistoryPathResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    location_history_path: Option<PathBuf>,
}

impl LocationHistoryPathResponse {
    pub fn new(state: &GuiServiceState) -> LocationHistoryPathResponse {
        LocationHistoryPathResponse {
            location_history_path: state.location_history_path().cloned(),
        }
    }
}

#[derive(Serialize)]
pub struct InterpolateResponse {
    interpolate: bool,
}

impl InterpolateResponse {
    pub fn new(state: &GuiServiceState) -> InterpolateResponse {
        InterpolateResponse { interpolate: state.interpolate() }
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

    pub fn filtered(state: &GuiServiceState) -> Result<PhotosResponse, ServiceError> {
        state
            .photo_paths()
            .par_iter()
            .filter_map(|path| {
                Photo::new(path).ok().and_then(
                    |photo| if photo.location().is_some() {
                        None
                    } else if state.location_history().contains(photo.timestamp()) {
                        Some(ImageDimensions::new(path))
                    } else {
                        None
                    },
                )
            })
            .collect::<Result<Vec<ImageDimensions>, ServiceError>>()
            .map(|photos| PhotosResponse { photos })
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
            .map(|path| LocationResponse::new(path, state))
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
    pub fn new(path: &Path, state: &GuiServiceState) -> Result<LocationResponse, ServiceError> {
        let result = get_location_suggestion(path, &state.location_history(), state.interpolate())
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

    use serde_json::to_string;

    fn state_with_root_path(root_path: &Path) -> GuiServiceState {
        let mut state = GuiServiceState::with_interpolate(false);
        state.search_new_root_path(root_path.to_path_buf());
        state
    }

    fn state_with_paths(root_path: &Path, location_history_path: &Path) -> GuiServiceState {
        let mut state = state_with_root_path(root_path);
        state
            .load_location_history(location_history_path.to_path_buf())
            .unwrap();
        state
    }

    #[test]
    fn root_path_response_new_should_get_the_root_path() {
        let state = state_with_root_path(Path::new("tests/assets"));
        let response = RootPathResponse::new(&state);

        assert_eq!(state.root_path(), response.root_path.as_ref());
    }

    #[test]
    fn location_history_path_response_new_should_get_the_location_history_path() {
        let state = state_with_paths(
            Path::new("tests/assets"),
            Path::new("tests/assets/location_history.json"),
        );
        let response = LocationHistoryPathResponse::new(&state);

        assert_eq!(
            state.location_history_path().unwrap(),
            &response.location_history_path.unwrap(),
        );
    }

    #[test]
    fn interpolate_response_new_should_get_the_root_path() {
        let state = GuiServiceState::with_interpolate(false);
        let response = InterpolateResponse::new(&state);

        assert_eq!(state.interpolate(), response.interpolate);
    }

    #[test]
    fn photos_response_new_should_get_data_for_all_found_photos() {
        let state = state_with_root_path(Path::new("tests/assets"));
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
        let state = state_with_paths(
            Path::new("tests/assets"),
            Path::new("tests/assets/location_history.json"),
        );
        let response = PhotosResponse::filtered(&state).unwrap();

        assert_eq!(
            "{\"photos\":[\
                {\"path\":\"tests/assets/photo_without_gps.jpg\",\"height\":37,\"width\":55}\
            ]}",
            to_string(&response).unwrap().replace("\\\\", "/")
        );
    }

    #[test]
    fn locations_response_new_should_get_locations_for_the_given_photo_index_range() {
        let state = state_with_root_path(Path::new("tests/assets"));
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
        let state = state_with_root_path(Path::new("tests/assets"));
        let response = LocationResponse::new(path, &state).unwrap();

        assert_eq!(path, response.path);
        assert!(response.location.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn location_response_new_should_set_only_a_path_for_empty_location_history() {
        let path = Path::new("tests/assets/photo_without_gps.jpg");
        let state = state_with_root_path(Path::new("tests/assets"));
        let response = LocationResponse::new(path, &state).unwrap();

        assert_eq!(path, response.path);
        assert!(response.location.is_none());
        assert!(response.error.is_none());
    }

    #[test]
    fn get_location_suggestion_should_set_a_location_if_the_photo_has_gps_metadata() {
        let path = Path::new("tests/assets/photo.jpg");
        let state = state_with_root_path(Path::new("tests/assets"));
        let response = LocationResponse::new(path, &state).unwrap();

        assert_eq!(path, response.path);
        assert!(response.location.is_some());
        assert!(response.error.is_none());
    }
}
