use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::marker::Send;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::thread;

use futures::future::{Future, ok};
use futures::Stream;
use futures::sync::oneshot;
use hyper;
use hyper::header::{CacheControl, CacheDirective, ContentType};
use hyper::mime;
use hyper::server::{Request, Response, Service};
use hyper::{StatusCode, Method, Uri};
use serde::Serialize;
use serde_json;
use tinyfiledialogs::{open_file_dialog, select_folder_dialog};
use yore::golo::{GoogleLocationHistory, HistoryError, load_location_history};

use common::{exiv2_write_coordinates, photo_paths};
use super::error::ServiceError;
use super::image::thumbnail;
use super::responses::{InterpolateResponse, LocationHistoryPathResponse, LocationResponse,
                       LocationsResponse, PhotosResponse, RootPathResponse};
use super::uri::{has_filter_parameter, queried_dimensions, queried_indices, queried_path};

pub struct GuiServiceState {
    root_path: Option<PathBuf>,
    photo_paths: Vec<PathBuf>,
    location_history_path: Option<PathBuf>,
    location_history: GoogleLocationHistory,
    interpolate: bool,
}

impl GuiServiceState {
    pub fn with_interpolate(interpolate: bool) -> GuiServiceState {
        GuiServiceState {
            root_path: None,
            photo_paths: Vec::default(),
            location_history_path: None,
            location_history: GoogleLocationHistory::default(),
            interpolate,
        }
    }

    pub fn root_path(&self) -> Option<&PathBuf> {
        self.root_path.as_ref()
    }

    pub fn photo_paths(&self) -> &[PathBuf] {
        &self.photo_paths
    }

    pub fn location_history_path(&self) -> Option<&PathBuf> {
        self.location_history_path.as_ref()
    }

    pub fn location_history(&self) -> &GoogleLocationHistory {
        &self.location_history
    }

    pub fn interpolate(&self) -> bool {
        self.interpolate
    }

    pub fn search_new_root_path(&mut self, root_path: PathBuf) {
        self.photo_paths = photo_paths(&root_path);
        self.root_path = Some(root_path);
    }

    pub fn load_location_history(&mut self, path: PathBuf) -> Result<(), HistoryError> {
        let file = File::open(&path)?;
        self.location_history = unsafe { load_location_history(&file)? };
        self.location_history_path = Some(path);

        Ok(())
    }

    pub fn set_interpolate(&mut self, interpolate: bool) {
        self.interpolate = interpolate;
    }
}

pub struct GuiService(Arc<RwLock<GuiServiceState>>);

impl GuiService {
    pub fn new(state: Arc<RwLock<GuiServiceState>>) -> GuiService {
        GuiService(state)
    }
}

impl Service for GuiService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let (method, uri, _, _, body) = req.deconstruct();

        match (method, uri.path()) {
            (Method::Get, "/rootPath") => handle_get_root_path(self.0.clone()),
            (Method::Get, "/rootPath/new") => handle_get_new_root_path(self.0.clone()),
            (Method::Get, "/locationHistoryPath") => handle_get_location_history(self.0.clone()),
            (Method::Get, "/locationHistory/new") => handle_get_new_location_history(
                self.0.clone(),
            ),
            (Method::Get, "/interpolate") => handle_get_interpolate(self.0.clone()),

            (Method::Get, "/photos") => handle_get_photos(self.0.clone(), uri.clone()),
            (Method::Get, "/locations") => handle_get_locations(self.0.clone(), uri.clone()),
            (Method::Get, "/location") => handle_get_location(self.0.clone(), uri.clone()),
            (Method::Get, "/thumbnail") => handle_get_thumbnail(uri.clone()),
            (Method::Get, path) => handle_get_static_file(path),

            (Method::Put, "/interpolate") => handle_put_interpolate(self.0.clone(), body),
            (Method::Put, "/location") => handle_put_location(uri.clone(), body),
            _ => {
                Box::new(ok(
                    Response::new().with_status(StatusCode::MethodNotAllowed),
                ))
            }
        }
    }
}

#[derive(Deserialize)]
struct InterpolateRequestBody {
    interpolate: bool,
}

type GuiServiceResponse = <GuiService as Service>::Future;

fn handle_get_root_path(state: Arc<RwLock<GuiServiceState>>) -> GuiServiceResponse {
    handle_in_thread(
        move || {
            let state = state.read()?;
            serialize(RootPathResponse::new(&state))
        },
        mime::APPLICATION_JSON,
    )
}

fn handle_get_new_root_path(state: Arc<RwLock<GuiServiceState>>) -> GuiServiceResponse {
    handle_in_thread(
        move || {
            if let Some(path) = select_folder_dialog("", "") {
                state.write()?.search_new_root_path(PathBuf::from(path));
            }

            let state = state.read()?;
            serialize(RootPathResponse::new(&state))
        },
        mime::APPLICATION_JSON,
    )
}

fn handle_get_location_history(state: Arc<RwLock<GuiServiceState>>) -> GuiServiceResponse {
    handle_in_thread(
        move || {
            let state = state.read()?;
            serialize(LocationHistoryPathResponse::new(&state))
        },
        mime::APPLICATION_JSON,
    )
}

fn handle_get_new_location_history(state: Arc<RwLock<GuiServiceState>>) -> GuiServiceResponse {
    handle_in_thread(
        move || {
            if let Some(path) = open_file_dialog("", "", None) {
                state.write()?.load_location_history(PathBuf::from(&path))?;
            }
            let state = state.read()?;
            serialize(LocationHistoryPathResponse::new(&state))
        },
        mime::APPLICATION_JSON,
    )
}

fn handle_get_interpolate(state: Arc<RwLock<GuiServiceState>>) -> GuiServiceResponse {
    handle_in_thread(
        move || {
            let state = state.read()?;
            serialize(InterpolateResponse::new(&state))
        },
        mime::APPLICATION_JSON,
    )
}

fn handle_get_photos(state: Arc<RwLock<GuiServiceState>>, uri: Uri) -> GuiServiceResponse {
    handle_in_thread(
        move || if has_filter_parameter(&uri) {
            PhotosResponse::filtered(&state.read()?.deref()).and_then(serialize)
        } else {
            PhotosResponse::new(&state.read()?.deref()).and_then(serialize)
        },
        mime::APPLICATION_JSON,
    )
}

fn handle_get_locations(state: Arc<RwLock<GuiServiceState>>, uri: Uri) -> GuiServiceResponse {
    handle_in_thread(
        move || {
            queried_indices(&uri)
                .and_then(|indices| {
                    let state = state.read()?;
                    LocationsResponse::new(&state, indices.0, indices.1)
                })
                .and_then(serialize)
        },
        mime::APPLICATION_JSON,
    )
}

fn handle_get_location(state: Arc<RwLock<GuiServiceState>>, uri: Uri) -> GuiServiceResponse {
    handle_in_thread(
        move || {
            queried_path(&uri)
                .and_then(|path| {
                    let state = state.read()?;
                    LocationResponse::new(&path, &state)
                })
                .and_then(serialize)
        },
        mime::APPLICATION_JSON,
    )
}

fn handle_get_thumbnail(uri: Uri) -> GuiServiceResponse {
    handle_in_thread(
        move || {
            queried_path(&uri)
                .and_then(|path| {
                    queried_dimensions(&uri).map(|(width, height)| (path, width, height))
                })
                .and_then(|(path, width, height)| thumbnail(&path, width, height))
        },
        mime::IMAGE_JPEG,
    )
}

fn handle_get_static_file(request_path: &str) -> GuiServiceResponse {
    let resolved_path = resolve_path(request_path);
    let owned_path = resolved_path.to_owned();
    handle_in_thread(
        move || read_file_bytes(&owned_path),
        file_mime_type(resolved_path),
    )
}

fn handle_put_interpolate(
    state: Arc<RwLock<GuiServiceState>>,
    body: hyper::Body,
) -> GuiServiceResponse {
    handle_request_body(body, move |bytes| {
        serde_json::from_slice(&bytes)
            .map_err(ServiceError::JsonError)
            .and_then(|body: InterpolateRequestBody| {
                Ok(state.write()?.set_interpolate(body.interpolate))
            })
    })
}

fn handle_put_location(uri: Uri, body: hyper::Body) -> GuiServiceResponse {
    handle_request_body(body, move |bytes| {
        serde_json::from_slice(&bytes)
            .map_err(ServiceError::JsonError)
            .and_then(|coordinates| {
                queried_path(&uri).map(|path| (path, coordinates))
            })
            .and_then(|(path, coordinates)| {
                exiv2_write_coordinates(&path, &coordinates).map_err(ServiceError::IoError)
            })
    })
}

fn handle_request_body<T, F>(body: hyper::Body, body_handler: F) -> GuiServiceResponse
where
    F: FnOnce(Vec<u8>) -> Result<T, ServiceError> + Send + 'static,
{
    let future = body.fold(Vec::new(), |mut vec, chunk| {
        vec.extend(&chunk[..]);
        ok::<Vec<u8>, hyper::Error>(vec)
    }).and_then(move |bytes| {
            let result = body_handler(bytes).map(|_| Vec::<u8>::new());

            ok(to_response(result, mime::TEXT_PLAIN_UTF_8))
        });

    Box::new(future)
}

fn handle_in_thread<T, F>(handle_request: F, response_mime_type: mime::Mime) -> GuiServiceResponse
where
    T: Into<hyper::Body>,
    F: FnOnce() -> Result<T, ServiceError> + Send + 'static,
{
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        let result = handle_request();

        tx.send(to_response(result, response_mime_type)).expect(
            "Error sending GET /thumbnail response from worker thread",
        );
    });

    Box::new(rx.map_err(|e| {
        hyper::Error::from(io::Error::new(io::ErrorKind::Other, e))
    }))
}

fn serialize<T: Serialize>(response_data: T) -> Result<String, ServiceError> {
    serde_json::to_string(&response_data).map_err(ServiceError::JsonError)
}

fn to_response<T: Into<hyper::Body>>(
    result: Result<T, ServiceError>,
    mime_type: mime::Mime,
) -> Response {
    let response = match result {
        Ok(body) => {
            Response::new().with_body(body).with_header(
                ContentType(mime_type),
            )
        }
        Err(ServiceError::IoError(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            Response::new().with_status(StatusCode::NotFound)
        }
        Err(ServiceError::MissingQueryParameter(e)) => {
            Response::new()
                .with_status(StatusCode::BadRequest)
                .with_body(format!("Missing query parameter: {:?}", e))
        }
        Err(e) => {
            Response::new()
                .with_status(StatusCode::InternalServerError)
                .with_body(format!("{:?}", e))
        }
    };

    response.with_header(CacheControl(vec![CacheDirective::NoCache]))
}

fn file_mime_type(path: &Path) -> mime::Mime {
    match path.extension().and_then(OsStr::to_str) {
        Some("css") => mime::TEXT_CSS,
        Some("html") => mime::TEXT_HTML_UTF_8,
        Some("js") => mime::TEXT_JAVASCRIPT,
        _ => mime::TEXT_PLAIN,
    }
}

fn resolve_path(path: &str) -> &Path {
    if path == "/" {
        Path::new("index.html")
    } else {
        Path::new(&path[1..])
    }
}

fn read_file_bytes(path: &Path) -> Result<&'static [u8], ServiceError> {
    match path.to_str() {
        Some("style.css") => Ok(include_bytes!("../../../dist/style.css")),
        Some("index.html") => Ok(include_bytes!("../../../dist/index.html")),
        Some("app.bundle.js") => Ok(include_bytes!("../../../dist/app.bundle.js")),
        _ => {
            Err(ServiceError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                "unrecognised resource",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_should_serialize_the_given_data_structure() {
        let state = GuiServiceState {
            root_path: Some(PathBuf::from("tests/assets")),
            photo_paths: Vec::default(),
            location_history_path: Some(PathBuf::from("tests/assets/location_history.json")),
            location_history: GoogleLocationHistory::default(),
            interpolate: false,
        };
        let string = serialize(&RootPathResponse::new(&state)).unwrap();

        assert_eq!("{\"rootPath\":\"tests/assets\"}", string);
    }

    #[test]
    fn to_response_should_map_ok_to_a_200_response_with_the_given_mime_type() {
        let result = Ok("test");
        let response = to_response(result, mime::TEXT_XML);

        assert_eq!(StatusCode::Ok, response.status());
        let raw = response.headers().get_raw("content-type").unwrap();
        assert_eq!(raw, "text/xml");
    }

    #[test]
    fn to_response_should_map_a_not_found_io_error_to_a_404_response() {
        let result: Result<String, ServiceError> = Err(ServiceError::IoError(
            io::Error::new(io::ErrorKind::NotFound, ""),
        ));
        let response = to_response(result, mime::TEXT_XML);

        assert_eq!(StatusCode::NotFound, response.status());
    }

    #[test]
    fn to_response_should_map_a_missing_query_parameter_error_to_a_400_response() {
        let result: Result<String, ServiceError> = Err(ServiceError::MissingQueryParameter("test"));
        let response = to_response(result, mime::TEXT_XML);

        assert_eq!(StatusCode::BadRequest, response.status());
    }

    #[test]
    fn to_response_should_map_general_errors_to_a_500_response() {
        let result: Result<String, ServiceError> = Err(ServiceError::ImageSizeError);
        let response = to_response(result, mime::TEXT_XML);

        assert_eq!(StatusCode::InternalServerError, response.status());
    }

    #[test]
    fn file_mime_type_should_return_text_css_for_a_path_ending_in_dot_css() {
        assert_eq!(mime::TEXT_CSS, file_mime_type(Path::new("test.css")));
    }

    #[test]
    fn file_mime_type_should_return_text_html_for_a_path_ending_in_dot_html() {
        assert_eq!(
            mime::TEXT_HTML_UTF_8,
            file_mime_type(Path::new("test.html"))
        );
    }

    #[test]
    fn file_mime_type_should_return_text_javascript_for_a_path_ending_in_dot_js() {
        assert_eq!(mime::TEXT_JAVASCRIPT, file_mime_type(Path::new("test.js")));
    }

    #[test]
    fn file_mime_type_should_return_text_plain_for_a_path_not_ending_in_dot_css_html_or_js() {
        assert_eq!(mime::TEXT_PLAIN, file_mime_type(Path::new("test.jpg")));
    }

    #[test]
    fn resolve_path_should_return_index_html_if_passed_a_single_forwardslash() {
        assert_eq!(Path::new("index.html"), resolve_path("/"));
    }

    #[test]
    fn resolve_path_should_trim_the_first_character_from_the_passed_path() {
        assert_eq!(Path::new("test"), resolve_path("/test"));
    }

    #[test]
    fn read_file_bytes_should_error_for_an_unrecognised_path() {
        assert!(read_file_bytes(Path::new("README.md")).is_err());
    }

    #[test]
    fn read_file_bytes_should_ok_for_a_recognised_path() {
        assert!(read_file_bytes(Path::new("index.html")).is_ok());
        assert!(read_file_bytes(Path::new("style.css")).is_ok());
        assert!(read_file_bytes(Path::new("app.bundle.js")).is_ok());
    }
}
