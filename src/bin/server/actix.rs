use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use actix_web::http::Method;
use actix_web::{App, Body, FromRequest, HttpRequest, HttpResponse, Json, Path as PathExtractor,
                Query, Result};
use futures::future::result;
use futures::Future;
use tinyfiledialogs::{open_file_dialog, select_folder_dialog};
use yore::Coordinates;

use super::error::ServiceError;
use super::image::{oriented_image, thumbnail};
use super::responses::{read_file_bytes, InterpolateResponse, LocationHistoryPathResponse,
                       LocationResponse, LocationsResponse, PhotosResponse, RootPathResponse};
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
type JsonResult<T> = Result<Json<T>, ServiceError>;
type HttpResult = Result<HttpResponse, ServiceError>;

pub struct GuiApplication(SharedGuiState);

impl GuiApplication {
    pub fn new(state: SharedGuiState) -> App<SharedGuiState> {
        App::with_state(state)
            .route("/rootPath", Method::GET, get_root_path)
            .route("/rootPath/new", Method::GET, get_new_root_path)
            .route("/locationHistoryPath", Method::GET, get_location_history)
            .route(
                "/locationHistory/new",
                Method::GET,
                get_new_location_history,
            )
            .route("/interpolate", Method::GET, get_interpolate)
            .route("/locations", Method::GET, get_locations)
            .route("/location", Method::GET, get_location)
            .route("/photos", Method::GET, get_photos)
            .route("/photo", Method::GET, get_photo)
            .route("/thumbnail", Method::GET, get_thumbnail)
            .resource("/{file}", |r| r.method(Method::GET).with(get_static_file))
            .route("/", Method::GET, get_index)
            .route("/interpolate", Method::PUT, put_interpolate)
            .route("/location", Method::PUT, put_location)
    }
}

fn get_root_path(req: Request) -> JsonResult<RootPathResponse> {
    let state = req.state().read()?;
    Ok(Json(RootPathResponse::new(&state)))
}

fn get_new_root_path(req: Request) -> JsonResult<RootPathResponse> {
    if let Some(path) = select_folder_dialog("", "") {
        req.state()
            .write()?
            .search_new_root_path(PathBuf::from(path));
    }

    let state = req.state().read()?;
    Ok(Json(RootPathResponse::new(&state)))
}

fn get_location_history(req: Request) -> JsonResult<LocationHistoryPathResponse> {
    let state = req.state().read()?;
    Ok(Json(LocationHistoryPathResponse::new(&state)))
}

fn get_new_location_history(req: Request) -> JsonResult<LocationHistoryPathResponse> {
    if let Some(path) = open_file_dialog("", "", None) {
        req.state()
            .write()?
            .load_location_history(PathBuf::from(&path))?;
    }
    let state = req.state().read()?;
    Ok(Json(LocationHistoryPathResponse::new(&state)))
}

fn get_interpolate(req: Request) -> JsonResult<InterpolateResponse> {
    let state = req.state().read()?;
    Ok(Json(InterpolateResponse::new(&state)))
}

fn get_locations(req: Request) -> JsonResult<LocationsResponse> {
    let indices = Query::<Indices>::extract(&req)?;

    let state = req.state().read()?;
    LocationsResponse::new(&state, indices.start, indices.end).map(Json)
}

fn get_location(req: Request) -> JsonResult<LocationResponse> {
    let query_params = Query::<QueriedPath>::extract(&req)?;

    let state = req.state().read()?;
    LocationResponse::new(&query_params.path, &state).map(Json)
}

fn get_photos(req: Request) -> JsonResult<PhotosResponse> {
    let query_params = Query::<GetPhotosQueryParams>::extract(&req)?;

    let state = req.state().read()?;

    if let Some(true) = query_params.filter {
        PhotosResponse::filtered(&state).map(Json)
    } else {
        PhotosResponse::new(&state).map(Json)
    }
}

fn get_photo(req: Request) -> HttpResult {
    let query_params = Query::<QueriedPath>::extract(&req)?;

    let body = oriented_image(&query_params.path).map(Body::from)?;

    Ok(HttpResponse::Ok().content_type(IMAGE_JPEG).body(body))
}

fn get_thumbnail(req: Request) -> HttpResult {
    let query_params = Query::<ThumbnailQueryParams>::extract(&req)?;
    let state = req.state().read()?;

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

fn get_static_file(file: PathExtractor<PathBuf>) -> HttpResult {
    let body = read_file_bytes(&file).map(Body::from)?;
    let mime = file_mime_type(&file);

    Ok(HttpResponse::Ok().content_type(mime).body(body))
}

fn get_index(_req: Request) -> HttpResult {
    let file = Path::new("index.html");
    let body = read_file_bytes(&file).map(Body::from)?;

    Ok(HttpResponse::Ok().content_type(TEXT_HTML_UTF_8).body(body))
}

fn put_interpolate(req: Request) -> Box<Future<Item = &'static str, Error = ServiceError>> {
    let request_body = Json::<InterpolateRequestBody>::extract(&req);

    Box::new(request_body.map_err(ServiceError::from).and_then(move |b| {
        let set_result = match req.state().write() {
            Ok(ref mut s) => {
                s.set_interpolate(b.interpolate);
                Ok("")
            }
            Err(e) => Err(ServiceError::from(e)),
        };
        result(set_result)
    }))
}

fn put_location(req: Request) -> Box<Future<Item = &'static str, Error = ServiceError>> {
    let coordinates = Json::<Coordinates>::extract(&req);

    Box::new(coordinates.map_err(ServiceError::from).and_then(move |c| {
        result(
            Query::<QueriedPath>::extract(&req)
                .map_err(ServiceError::from)
                .and_then(|q| {
                    exiv2_write_coordinates(&q.path, &c)
                        .map(|_| "")
                        .map_err(ServiceError::from)
                }),
        )
    }))
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
    use super::*;

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
