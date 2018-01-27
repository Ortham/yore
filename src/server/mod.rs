use std::fs::File;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::path::Path;
use std::sync::{Arc, RwLock};

use hyper::server::Http;
use yore::golo::load_location_history;

mod error;
mod image;
mod orientation;
mod responses;
mod service;
mod uri;

use self::service::{GuiService, GuiServiceState};
use super::ApplicationError;

pub fn run_server(
    port: u16,
    root_path: &Path,
    location_history_path: &Path,
    interpolate: bool,
) -> Result<(), ApplicationError> {
    println!("Loading location history...");
    let location_history_file = File::open(location_history_path)?;
    let location_history = unsafe { load_location_history(&location_history_file)? };

    println!("Scanning for photos...");
    let state = GuiServiceState::new(root_path, location_history, interpolate);
    let shared_state = Arc::new(RwLock::new(state));

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let server = Http::new().bind(&address, move || {
        Ok(GuiService::new(shared_state.clone()))
    })?;

    println!("Listening on http://{}", address);
    server.run()?;

    Ok(())
}
