#![deny(warnings)]

extern crate hyper;
extern crate hyper_native_tls;
extern crate tempdir;

use std::fs;
use std::io::Read;
use std::io::Write;
use std::path;
use self::tempdir::TempDir;

pub const SUBDIR_1: &'static str = "1";
pub const SUBDIR_2: &'static str = "2";
pub const JPG_FILENAME: &'static str = "file.jpg";
pub const JPEG_FILENAME: &'static str = "file.jpeg";
pub const UPPERCASE_JPG_FILENAME: &'static str = "file.JPG";
pub const UPPERCASE_JPEG_FILENAME: &'static str = "file.JPEG";
pub const PNG_FILENAME: &'static str = "file.png";
pub const INACCURATE_FILENAME: &'static str = "file.actually.a.png.jpg";
pub const JPG_URL: &'static str = "https://placehold.it/300.jpg";
pub const PNG_URL: &'static str = "https://placehold.it/300.png";
pub const JPG_WITH_EXIF_URL: &'static str = "http://www.exiv2.org/include/img_1771.jpg";

pub fn setup() -> Result<TempDir, Box<::std::error::Error>> {
    let tmp_dir: TempDir = try!(TempDir::new("orfot_test_"));

    let tmp_subdir_1 = tmp_dir.path().join(SUBDIR_1);
    let tmp_subdir_2 = tmp_dir.path().join(SUBDIR_2);
    try!(fs::create_dir_all(&tmp_subdir_1));
    try!(fs::create_dir_all(&tmp_subdir_2));

    try!(wget(JPG_URL, tmp_subdir_1.join(JPEG_FILENAME)));
    try!(wget(JPG_URL, tmp_subdir_1.join(UPPERCASE_JPG_FILENAME)));
    try!(wget(PNG_URL, tmp_subdir_1.join(PNG_FILENAME)));

    try!(wget(JPG_WITH_EXIF_URL, tmp_subdir_2.join(JPG_FILENAME)));
    try!(wget(JPG_URL, tmp_subdir_2.join(UPPERCASE_JPEG_FILENAME)));
    try!(wget(PNG_URL, tmp_subdir_2.join(INACCURATE_FILENAME)));

    Ok(tmp_dir)
}

fn wget(url: &str, destination: path::PathBuf) -> Result<(), Box<::std::error::Error>> {
    let ssl = try!(hyper_native_tls::NativeTlsClient::new());
    let connector = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(connector);

    let mut response = try!(client.get(url).send());

    let mut response_body: Vec<u8> = Vec::new();
    try!(response.read_to_end(&mut response_body));

    let mut file = try!(fs::File::create(destination));

    try!(file.write_all(response_body.as_slice()));

    Ok(())
}
