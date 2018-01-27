use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::path::Path;
use std::sync::{Arc, RwLock};

use hyper::server::Http;

mod error;
mod image;
mod orientation;
mod responses;
mod service;
mod uri;

use self::service::{GuiService, GuiServiceState};
use super::ApplicationError;

pub struct Server {
    address: SocketAddr,
    state: GuiServiceState,
}

impl Server {
    pub fn new(port: u16, interpolate: bool) -> Server {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let state = GuiServiceState::with_interpolate(interpolate);

        Server { address, state }
    }

    pub fn search_photos_path(&mut self, path: &Path) {
        self.state.search_new_root_path(path.to_path_buf());
    }

    pub fn load_location_history(&mut self, path: &Path) -> Result<(), ApplicationError> {
        self.state
            .load_location_history(path.to_path_buf())
            .map_err(ApplicationError::from)
    }

    pub fn run(self) -> Result<(), ApplicationError> {
        let shared_state = Arc::new(RwLock::new(self.state));

        let server = Http::new().bind(&self.address, move || {
            Ok(GuiService::new(shared_state.clone()))
        })?;

        println!("Listening on http://{}", server.local_addr()?);
        server.run().map_err(ApplicationError::from)
    }
}
