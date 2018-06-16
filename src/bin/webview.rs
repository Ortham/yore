#![windows_subsystem = "windows"]

extern crate actix_web;
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
#[macro_use]
extern crate structopt;

mod common;

use std::path::PathBuf;

use structopt::StructOpt;

use common::server::Server;

#[derive(StructOpt)]
#[structopt(
    name = "yore",
    about = "Yore uses an exported Google Location History JSON file to suggest locations for
            images"
)]
struct Options {
    #[structopt(
        short = "l",
        long = "locations",
        parse(from_os_str),
        help = "The path to a Google Location History JSON file"
    )]
    location_history_path: Option<PathBuf>,

    #[structopt(
        short = "i",
        long = "interpolate",
        help = "Interpolate between locations if an exact match is not found"
    )]
    interpolate: bool,

    #[structopt(
        parse(from_os_str), help = "The image or a directory of images to suggest a location for"
    )]
    photo_path: Option<PathBuf>,
}

fn main() {
    let options = Options::from_args();

    let mut server = Server::new(0, options.interpolate);

    if let Some(path) = options.photo_path {
        server.search_photos_path(&path);
    }

    if let Some(path) = options.location_history_path {
        server.load_location_history(&path).unwrap();
    }

    run_webview(server);
}

fn run_webview(server: Server) {
    let address = server.spawn().unwrap();

    let size = (800, 600);
    let resizable = true;
    let debug = true;
    let init_cb = |_| {};

    web_view::run(
        "Yore",
        web_view::Content::Url(format!("http://{}", address)),
        Some(size),
        resizable,
        debug,
        init_cb,
        /* frontend_cb: */
        |_, _, _| {},
        (),
    );
}
