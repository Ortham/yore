extern crate clap;
extern crate exif;
extern crate futures;
extern crate hyper;
extern crate image;
extern crate jpeg_decoder;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate tinyfiledialogs;
extern crate url;
extern crate yore;

#[macro_use]
extern crate serde_derive;

mod cli;
mod common;
mod server;

use std::path::Path;

use clap::{Arg, App};

use cli::run_cli;

use server::Server;

fn main() {
    let matches = App::new("yore")
        .version(env!("CARGO_PKG_VERSION"))
        .about(
            "Yore uses an exported Google Location History JSON file to suggest locations for
            images",
        )
        .author("Oliver Hamlet")
        .arg(
            Arg::with_name("location_history")
                .long("locations")
                .short("l")
                .value_name("FILE")
                .takes_value(true)
                .required_unless("gui")
                .help("The path to a Google Location History JSON file"),
        )
        .arg(
            Arg::with_name("interpolate")
                .long("interpolate")
                .short("i")
                .help(
                    "Interpolate between locations if an exact match is not found",
                ),
        )
        .arg(
            Arg::with_name("read-only")
                .long("read-only")
                .short("r")
                .help("Don't offer to save suggested locations"),
        )
        .arg(Arg::with_name("gui").long("gui").short("g").help(
            "Start a server for the browser-based GUI",
        ))
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .default_value("8080")
                .help("The port that the GUI server should listen on"),
        )
        .arg(
            Arg::with_name("INPUT")
                .required_unless("gui")
                .index(1)
                .help(
                    "The image or a directory of images to suggest a location for",
                ),
        )
        .get_matches();

    let photo_path = matches.value_of("INPUT").map(Path::new);
    let location_history_path = matches.value_of("location_history").map(Path::new);
    let interpolate = matches.is_present("interpolate");
    let read_only = matches.is_present("read-only");
    let use_gui = matches.is_present("gui");
    let gui_port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    if use_gui {
        let mut server = Server::new(gui_port, interpolate);

        if let Some(path) = photo_path {
            server.search_photos_path(path);
        }

        if let Some(path) = location_history_path {
            server.load_location_history(path).unwrap();
        }

        server.run().unwrap();
    } else {
        run_cli(
            photo_path.unwrap(),
            location_history_path.unwrap(),
            interpolate,
            read_only,
        ).unwrap();
    }
}