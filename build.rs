#[cfg(windows)]
extern crate svg_to_ico;
#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
fn main() {
    let ico_path = "target/icon/icon.ico";
    let ico_entry_sizes = &[16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 128, 256];
    let svg_dpi = 96.0;

    svg_to_ico::svg_to_ico(
        Path::new("src/bin/icon.svg"),
        svg_dpi,
        Path::new(ico_path),
        ico_entry_sizes,
    ).expect("failed to generate ICO file");

    let mut res = winres::WindowsResource::new();
    res.set_icon(ico_path);
    res.compile().expect("failed to compile Windows resources");
}

#[cfg(unix)]
fn main() {}
