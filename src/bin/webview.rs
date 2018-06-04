#![windows_subsystem = "windows"]

extern crate actix_web;
extern crate clap;
extern crate directories;
extern crate exif;
extern crate futures;
extern crate image;
extern crate jpeg_decoder;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate tinyfiledialogs;
extern crate web_view;
extern crate yore;

#[macro_use]
extern crate serde_derive;

mod common;
mod server;

use std::path::Path;

use clap::{App, Arg};

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
                .help("The path to a Google Location History JSON file"),
        )
        .arg(
            Arg::with_name("interpolate")
                .long("interpolate")
                .short("i")
                .help("Interpolate between locations if an exact match is not found"),
        )
        .arg(
            Arg::with_name("INPUT")
                .index(1)
                .help("The image or a directory of images to suggest a location for"),
        )
        .get_matches();

    let photo_path = matches.value_of("INPUT").map(Path::new);
    let location_history_path = matches.value_of("location_history").map(Path::new);
    let interpolate = matches.is_present("interpolate");

    let mut server = Server::new(0, interpolate);

    if let Some(path) = photo_path {
        server.search_photos_path(path);
    }

    if let Some(path) = location_history_path {
        server.load_location_history(path).unwrap();
    }

    run_webview(server);
}

fn run_webview(server: Server) {
    let address = server.spawn().unwrap();

    let size = (800, 600);
    let resizable = true;
    let debug = true;
    let init_cb = |_| {};
    let userdata = ();

    web_view::run(
        "Yore",
        web_view::Content::Url(format!("http://{}", address)),
        Some(size),
        resizable,
        debug,
        init_cb,
        /* frontend_cb: */
        |_, _, _| {},
        userdata,
    );
}
