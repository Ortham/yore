use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use actix_web::server;

use common::ApplicationError;

mod actix;
mod error;
mod image;
mod orientation;
mod responses;
mod state;

use self::actix::GuiApplication;
use self::state::GuiState;

pub struct Server {
    address: SocketAddr,
    state: GuiState,
}

impl Server {
    pub fn new(port: u16, interpolate: bool) -> Server {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let state = GuiState::with_interpolate(interpolate);

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
        self.state.clear_cache()?;

        let shared_state = Arc::new(RwLock::new(self.state));

        let server =
            server::new(move || GuiApplication::new(shared_state.clone())).bind(&self.address)?;

        println!("Listening on http://{}", server.addrs()[0]);

        Ok(server.run())
    }

    pub fn spawn(self) -> Result<SocketAddr, ApplicationError> {
        self.state.clear_cache()?;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let shared_state = Arc::new(RwLock::new(self.state));

            let server = server::new(move || GuiApplication::new(shared_state.clone()))
                .bind(&self.address)
                .expect(&format!("Failed to bind HTTP server to {}", self.address));

            tx.send(server.addrs()[0])
                .expect("Failed to send the server's listen address to the main thread");

            server.run();
        });

        let address = rx.recv()
            .expect("Failed to receive the server's listen address from its main thread");

        println!("Listening on http://{}", address);

        Ok(address)
    }
}
