use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use actix_web::server;
use directories::ProjectDirs;

use common::ApplicationError;

mod actix;
mod error;
mod image;
mod orientation;
mod responses;
mod state;

use self::actix::build_server_app;
use self::state::GuiState;

pub struct Server {
    address: SocketAddr,
    state: GuiState,
}

impl Server {
    pub fn new(port: u16, interpolate: bool) -> Server {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let mut state = GuiState::new(ProjectDirs::from("", "", "Yore").cache_dir());
        state.set_interpolate(interpolate);

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

    #[allow(dead_code)]
    pub fn run(self) -> Result<(), ApplicationError> {
        self.state.clear_cache()?;

        let shared_state = Arc::new(RwLock::new(self.state));

        let server =
            server::new(move || build_server_app(shared_state.clone())).bind(&self.address)?;

        println!("Listening on http://{}", server.addrs()[0]);

        server.run();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn spawn(self) -> Result<SocketAddr, ApplicationError> {
        self.state.clear_cache()?;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let shared_state = Arc::new(RwLock::new(self.state));

            let server = server::new(move || build_server_app(shared_state.clone()))
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::net::TcpStream;

    #[test]
    fn spawn_should_start_a_server_in_the_background() {
        let mut server = Server::new(8080, true);

        server.search_photos_path(Path::new("tests/assets"));
        server
            .load_location_history(Path::new("tests/assets/location_history.json"))
            .unwrap();
        let address = server.spawn().unwrap();

        assert_eq!(address.ip(), IpAddr::from([127, 0, 0, 1]));
        assert_eq!(address.port(), 8080);
        assert!(TcpStream::connect(address).is_ok());
    }
}
