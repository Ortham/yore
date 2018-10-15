use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use actix_web::http::StatusCode;
use actix_web::{
    App, Body, HttpRequest, HttpResponse, Json, Path as PathExtractor, Query, Result, State,
};
use tinyfiledialogs::{open_file_dialog, select_folder_dialog};
use yore::Coordinates;

use super::error::ServiceError;
use super::image::{oriented_image, thumbnail};
use super::responses::{
    read_file_bytes, InterpolateResponse, LocationHistoryPathResponse, LocationResponse,
    LocationsResponse, PhotosResponse, RootPathResponse,
};
use super::state::GuiState;
use common::exiv2_write_coordinates;

const IMAGE_JPEG: &str = "image/jpeg";
const TEXT_CSS: &str = "text/css";
const TEXT_HTML_UTF_8: &str = "text/html; charset=utf-8";
const TEXT_JAVASCRIPT: &str = "text/javascript";
const TEXT_PLAIN: &str = "text/plain";

#[derive(Deserialize)]
struct GetPhotosQueryParams {
    filter: Option<bool>,
}

#[derive(Deserialize)]
struct Indices {
    start: usize,
    end: usize,
}

#[derive(Deserialize)]
struct QueriedPath {
    path: PathBuf,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ThumbnailQueryParams {
    path: PathBuf,
    max_width: u32,
    max_height: u32,
}

#[derive(Deserialize)]
struct InterpolateRequestBody {
    interpolate: bool,
}

type SharedGuiState = Arc<RwLock<GuiState>>;
type Request = HttpRequest<SharedGuiState>;
type RequestState = State<SharedGuiState>;
type JsonResult<T> = Result<Json<T>, ServiceError>;
type HttpResult = Result<HttpResponse, ServiceError>;

pub fn build_server_app(state: SharedGuiState) -> App<SharedGuiState> {
    App::with_state(state)
        .resource("/rootPath", |r| r.get().with(get_root_path))
        .resource("/rootPath/new", |r| r.get().with(get_new_root_path))
        .resource("/locationHistoryPath", |r| {
            r.get().with(get_location_history_path)
        }).resource("/locationHistory/new", |r| {
            r.get().with(get_new_location_history)
        }).resource("/interpolate", |r| {
            r.get().with(get_interpolate);
            r.put().with(put_interpolate);
        }).resource("/locations", |r| r.get().with(get_locations))
        .resource("/location", |r| {
            r.get().with(get_location);
            r.put().with(put_location);
        }).resource("/photos", |r| r.get().with(get_photos))
        .resource("/photo", |r| r.get().with(get_photo))
        .resource("/thumbnail", |r| r.get().with(get_thumbnail))
        .resource("/{file}", |r| r.get().with(get_static_file))
        .resource("/", |r| r.get().f(get_index))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_root_path(state: RequestState) -> JsonResult<RootPathResponse> {
    let state = state.read()?;
    Ok(Json(RootPathResponse::new(&state)))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_new_root_path(state: RequestState) -> JsonResult<RootPathResponse> {
    if let Some(path) = select_folder_dialog("", "") {
        state.write()?.search_new_root_path(PathBuf::from(path));
    }

    let state = state.read()?;
    Ok(Json(RootPathResponse::new(&state)))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_location_history_path(state: RequestState) -> JsonResult<LocationHistoryPathResponse> {
    let state = state.read()?;
    Ok(Json(LocationHistoryPathResponse::new(&state)))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_new_location_history(state: RequestState) -> JsonResult<LocationHistoryPathResponse> {
    if let Some(path) = open_file_dialog("", "", None) {
        state.write()?.load_location_history(PathBuf::from(&path))?;
    }
    let state = state.read()?;
    Ok(Json(LocationHistoryPathResponse::new(&state)))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_interpolate(state: RequestState) -> JsonResult<InterpolateResponse> {
    let state = state.read()?;
    Ok(Json(InterpolateResponse::new(&state)))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_locations(
    (indices, state): (Query<Indices>, RequestState),
) -> JsonResult<LocationsResponse> {
    let state = state.read()?;
    LocationsResponse::new(&state, indices.start, indices.end).map(Json)
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_location(
    (query_params, state): (Query<QueriedPath>, RequestState),
) -> JsonResult<LocationResponse> {
    let state = state.read()?;
    LocationResponse::new(&query_params.path, &state).map(Json)
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_photos(
    (query_params, state): (Query<GetPhotosQueryParams>, RequestState),
) -> JsonResult<PhotosResponse> {
    let state = state.read()?;

    if let Some(true) = query_params.filter {
        PhotosResponse::filtered(&state).map(Json)
    } else {
        PhotosResponse::new(&state).map(Json)
    }
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_photo(query_params: Query<QueriedPath>) -> HttpResult {
    let body = oriented_image(&query_params.path).map(Body::from)?;

    Ok(HttpResponse::Ok().content_type(IMAGE_JPEG).body(body))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_thumbnail((query_params, state): (Query<ThumbnailQueryParams>, RequestState)) -> HttpResult {
    let state = state.read()?;

    let cached_path = state.cached_image_path(
        &query_params.path,
        query_params.max_width,
        query_params.max_height,
    );

    use std::fs::{create_dir_all, read, write};

    let image = if cached_path.exists() {
        read(cached_path)?
    } else {
        let image = thumbnail(
            &query_params.path,
            query_params.max_width,
            query_params.max_height,
        )?;

        if let Some(path) = cached_path.parent() {
            create_dir_all(path)?;
        }

        write(cached_path, &image)?;

        image
    };

    Ok(HttpResponse::Ok()
        .content_type(IMAGE_JPEG)
        .body(Body::from(image)))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_static_file(file: PathExtractor<PathBuf>) -> HttpResult {
    let body = read_file_bytes(&file).map(Body::from)?;
    let mime = file_mime_type(&file);

    Ok(HttpResponse::Ok().content_type(mime).body(body))
}

#[allow(unknown_lints, needless_pass_by_value)]
fn get_index(_req: &Request) -> HttpResult {
    let file = Path::new("index.html");
    let body = read_file_bytes(&file).map(Body::from)?;

    Ok(HttpResponse::Ok().content_type(TEXT_HTML_UTF_8).body(body))
}

fn put_interpolate((body, state): (Json<InterpolateRequestBody>, RequestState)) -> HttpResult {
    state.write()?.set_interpolate(body.interpolate);

    Ok(HttpResponse::new(StatusCode::OK))
}

fn put_location((body, query_params): (Json<Coordinates>, Query<QueriedPath>)) -> HttpResult {
    exiv2_write_coordinates(&query_params.path, &body)?;

    Ok(HttpResponse::new(StatusCode::OK))
}

fn file_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(OsStr::to_str) {
        Some("css") => TEXT_CSS,
        Some("html") => TEXT_HTML_UTF_8,
        Some("js") => TEXT_JAVASCRIPT,
        _ => TEXT_PLAIN,
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use super::*;

    use std::fs::read;
    use std::process::Command;

    use self::tempfile::tempdir;
    use actix_web::client::ClientResponse;
    use actix_web::test::{self, TestServer, TestServerBuilder};
    use actix_web::{
        http::{Method, StatusCode},
        HttpMessage,
    };
    use futures::Future;

    fn test_state(cache_path: &Path) -> SharedGuiState {
        let mut state = GuiState::new(cache_path);
        state.search_new_root_path(PathBuf::from("tests/assets"));
        state
            .load_location_history(PathBuf::from("tests/assets/location_history.json"))
            .unwrap();

        Arc::new(RwLock::new(state))
    }

    fn test_server(cache_path: &Path) -> TestServer {
        test_server_and_state(cache_path).0
    }

    fn test_server_and_state(cache_path: &Path) -> (TestServer, SharedGuiState) {
        let state = test_state(cache_path);

        let server_state = state.clone();
        let server = TestServerBuilder::new(move || server_state.clone()).start(|app| {
            app.resource("/rootPath", |r| r.get().with(get_root_path))
                .resource("/rootPath/new", |r| r.get().with(get_new_root_path))
                .resource("/locationHistoryPath", |r| {
                    r.get().with(get_location_history_path)
                }).resource("/locationHistory/new", |r| {
                    r.get().with(get_new_location_history)
                }).resource("/interpolate", |r| {
                    r.get().with(get_interpolate);
                    r.put().with(put_interpolate);
                }).resource("/locations", |r| r.get().with(get_locations))
                .resource("/location", |r| {
                    r.get().with(get_location);
                    r.put().with(put_location);
                }).resource("/photos", |r| r.get().with(get_photos))
                .resource("/photo", |r| r.get().with(get_photo))
                .resource("/thumbnail", |r| r.get().with(get_thumbnail))
                .resource("/{file}", |r| r.get().with(get_static_file))
                .resource("/", |r| r.get().f(get_index));
        });

        (server, state)
    }

    fn body(response: ClientResponse) -> impl AsRef<[u8]> {
        response
            .body()
            .limit(u16::max_value() as usize)
            .wait()
            .unwrap()
    }

    fn json(response: ClientResponse) -> String {
        let json = response
            .body()
            .limit(u16::max_value() as usize)
            .wait()
            .unwrap();
        String::from_utf8_lossy(json.as_ref())
            .into_owned()
            .replace("\\\\", "/")
    }

    #[test]
    fn get_root_path_should_respond_with_the_current_photos_root_path() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv.client(Method::GET, "/rootPath").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(json(response), "{\"rootPath\":\"tests/assets\"}");
    }

    #[test]
    fn get_location_history_path_should_respond_with_the_current_location_history_path() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv
            .client(Method::GET, "/locationHistoryPath")
            .finish()
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let expected_json = "{\"locationHistoryPath\":\"tests/assets/location_history.json\"}";

        assert_eq!(json(response), expected_json);
    }

    #[test]
    fn get_interpolate_should_respond_with_the_current_interpolation_state() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv.client(Method::GET, "/interpolate").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(json(response), "{\"interpolate\":false}");
    }

    #[test]
    fn get_locations_should_respond_with_the_locations_of_the_queried_range_of_photos() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv
            .client(Method::GET, "/locations?start=0&end=1")
            .finish()
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let expected_json = "{\"locations\":[{\"path\":\"tests/assets/photo.jpg\",\"location\":{\"Existing\":{\"latitude\":38.76544,\"longitude\":-9.094802222222222}}}],\"start_index\":0,\"stop_index\":1}";

        assert_eq!(json(response), expected_json);
    }

    #[test]
    fn get_location_should_respond_with_the_location_for_the_queried_path() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv
            .client(Method::GET, "/location?path=tests/assets/photo.jpg")
            .finish()
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let expected_json = "{\"path\":\"tests/assets/photo.jpg\",\"location\":{\"Existing\":{\"latitude\":38.76544,\"longitude\":-9.094802222222222}}}";

        assert_eq!(json(response), expected_json);
    }

    #[test]
    fn get_photos_should_respond_with_all_photos_if_filter_is_not_set() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv.client(Method::GET, "/photos").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let expected_json = "{\"photos\":[{\"path\":\"tests/assets/photo.jpg\",\"height\":37,\"width\":55},{\"path\":\"tests/assets/photo_rotated.jpg\",\"height\":50,\"width\":33},{\"path\":\"tests/assets/photo_without_exif.jpg\",\"height\":37,\"width\":55},{\"path\":\"tests/assets/photo_without_gps.jpg\",\"height\":37,\"width\":55},{\"path\":\"tests/assets/photo_without_orientation.jpg\",\"height\":33,\"width\":50},{\"path\":\"tests/assets/photo_without_timestamp.jpg\",\"height\":37,\"width\":55}]}";

        assert_eq!(json(response), expected_json);
    }

    #[test]
    fn get_photos_should_respond_with_all_photos_if_filter_is_false() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv
            .client(Method::GET, "/photos?filter=false")
            .finish()
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let expected_json = "{\"photos\":[{\"path\":\"tests/assets/photo.jpg\",\"height\":37,\"width\":55},{\"path\":\"tests/assets/photo_rotated.jpg\",\"height\":50,\"width\":33},{\"path\":\"tests/assets/photo_without_exif.jpg\",\"height\":37,\"width\":55},{\"path\":\"tests/assets/photo_without_gps.jpg\",\"height\":37,\"width\":55},{\"path\":\"tests/assets/photo_without_orientation.jpg\",\"height\":33,\"width\":50},{\"path\":\"tests/assets/photo_without_timestamp.jpg\",\"height\":37,\"width\":55}]}";

        assert_eq!(json(response), expected_json);
    }

    #[test]
    fn get_photos_should_respond_with_only_photos_with_suggested_locations_if_filter_is_true() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv
            .client(Method::GET, "/photos?filter=true")
            .finish()
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );

        let expected_json = "{\"photos\":[{\"path\":\"tests/assets/photo_without_gps.jpg\",\"height\":37,\"width\":55}]}";

        assert_eq!(json(response), expected_json);
    }

    #[test]
    fn get_photo_should_respond_with_image_in_body_and_image_jpeg_mime_type() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let request = srv
            .client(Method::GET, "/photo?path=tests/assets/photo.jpg")
            .finish()
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "image/jpeg"
        );

        let image = oriented_image(Path::new("tests/assets/photo.jpg")).unwrap();

        assert_eq!(body(response).as_ref(), image.as_slice());
    }

    #[test]
    fn get_thumbnail_should_respond_with_a_binary_body_and_image_jpeg_mime_type() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let path = "/thumbnail?path=tests/assets/photo_rotated.jpg&maxWidth=300&maxHeight=300";
        let request = srv.client(Method::GET, path).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "image/jpeg"
        );
        assert!(!body(response).as_ref().is_empty());
    }

    #[test]
    fn get_thumbnail_should_not_respond_with_original_image() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let path = "/thumbnail?path=tests/assets/photo_rotated.jpg&maxWidth=300&maxHeight=300";
        let request = srv.client(Method::GET, path).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "image/jpeg"
        );

        let image = oriented_image(Path::new("tests/assets/photo_rotated.jpg")).unwrap();

        assert_ne!(body(response).as_ref(), image.as_slice());
    }

    #[test]
    fn get_thumbnail_should_cache_generated_thumbnails() {
        let tmp_dir = tempdir().unwrap();
        let (mut srv, state) = test_server_and_state(tmp_dir.path());

        let cached_path = state.read().unwrap().cached_image_path(
            Path::new("tests/assets/photo_rotated.jpg"),
            300,
            300,
        );
        assert!(!cached_path.exists());

        let path = "/thumbnail?path=tests/assets/photo_rotated.jpg&maxWidth=300&maxHeight=300";
        let request = srv.client(Method::GET, path).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert!(cached_path.exists());
        let file_content = read(cached_path).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body(response).as_ref(), file_content.as_slice());

        let request = srv.client(Method::GET, path).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body(response).as_ref(), file_content.as_slice());
    }

    #[test]
    fn get_static_file_should_respond_with_a_non_empty_body() {
        let mut srv = TestServer::new(|app| {
            app.resource("/{file}", |r| r.method(Method::GET).with(get_static_file));
        });

        let request = srv.client(Method::GET, "/index.html").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.body().limit(1024).wait().unwrap();

        assert!(!body.is_empty());
    }

    #[test]
    fn get_index_should_respond_with_a_binary_body() {
        let tmp_dir = tempdir().unwrap();
        let state = test_state(tmp_dir.path());
        let response = test::TestRequest::with_state(state.clone())
            .run(&get_index)
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.body().is_binary());
    }

    #[test]
    fn put_interpolate_should_set_interpolate_state_to_the_given_value() {
        let tmp_dir = tempdir().unwrap();
        let (mut srv, state) = test_server_and_state(tmp_dir.path());

        let request = srv
            .client(Method::PUT, "/interpolate")
            .header("Content-Type", "application/json")
            .body("{\"interpolate\":true}")
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(state.read().unwrap().interpolate());
    }

    #[test]
    fn put_location_should_fail_if_exiv2_is_not_available_and_succeed_otherwise() {
        let tmp_dir = tempdir().unwrap();
        let mut srv = test_server(tmp_dir.path());

        let path = "/location?path=tests/assets/photo_without_gps.jpg";
        let request = srv
            .client(Method::PUT, path)
            .header("Content-Type", "application/json")
            .body("{\"latitude\":0,\"longitude\":0}")
            .unwrap();
        let response = srv.execute(request.send()).unwrap();

        if Command::new("exiv2").status().is_err() {
            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        } else {
            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[test]
    fn file_mime_type_should_return_text_css_for_a_path_ending_in_dot_css() {
        assert_eq!(TEXT_CSS, file_mime_type(Path::new("test.css")));
    }

    #[test]
    fn file_mime_type_should_return_text_html_for_a_path_ending_in_dot_html() {
        assert_eq!(TEXT_HTML_UTF_8, file_mime_type(Path::new("test.html")));
    }

    #[test]
    fn file_mime_type_should_return_text_javascript_for_a_path_ending_in_dot_js() {
        assert_eq!(TEXT_JAVASCRIPT, file_mime_type(Path::new("test.js")));
    }

    #[test]
    fn file_mime_type_should_return_text_plain_for_a_path_not_ending_in_dot_css_html_or_js() {
        assert_eq!(TEXT_PLAIN, file_mime_type(Path::new("test.jpg")));
    }
}
