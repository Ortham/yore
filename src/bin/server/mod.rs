use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use hyper::server::Http;

use common::ApplicationError;

mod error;
mod image;
mod orientation;
mod responses;
mod service;
mod uri;

use self::service::{GuiService, GuiServiceState};

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

    pub fn spawn(self) -> Result<SocketAddr, ApplicationError> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let shared_state = Arc::new(RwLock::new(self.state));

            let server = Http::new()
                .bind(&self.address, move || {
                    Ok(GuiService::new(shared_state.clone()))
                })
                .expect(&format!("Failed to bind HTTP server to {}", self.address));

            let address = server
                .local_addr()
                .expect("Failed to get server's listen address");

            tx.send(address)
                .expect("Failed to send the server's listen address to the main thread");

            server.run().expect("Failed to run the server");
        });

        let address = rx.recv()
            .expect("Failed to receive the server's listen address from its main thread");

        println!("Listening on http://{}", address);

        Ok(address)
    }
}
