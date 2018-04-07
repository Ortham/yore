#[cfg(windows)]
extern crate ico;
#[cfg(windows)]
extern crate nsvg;
#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
fn main() {
    let ico_path = "target/icon/icon.ico";
    let ico_entry_sizes = &[16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 128, 256];
    let svg_dpi = 96.0;

    svg_ico::svg_to_ico(
        "src/bin/icon.svg",
        svg_dpi,
        Path::new(ico_path),
        ico_entry_sizes,
    ).expect("failed to generate ICO file");

    let mut res = winres::WindowsResource::new();
    res.set_icon(ico_path);
    res.compile().expect("failed to compile Windows resources");
}

#[cfg(windows)]
mod svg_ico {
    use std::fs::{create_dir_all, File};
    use std::io;
    use std::path::Path;

    use ico;
    use nsvg;

    // nsvg has access to the SVG image height, but doesn't expose that, so it's hardcoded.
    const SVG_HEIGHT: u8 = 192;

    struct Image {
        width: u32,
        height: u32,
        data: Vec<u8>,
    }

    pub fn svg_to_ico(
        svg_path: &str,
        svg_dpi: f32,
        ico_path: &Path,
        ico_entry_sizes: &[u16],
    ) -> io::Result<()> {
        let images: Vec<Image> = ico_entry_sizes
            .iter()
            .map(|size| rasterize(svg_path, svg_dpi, *size))
            .collect();

        create_ico(ico_path, images)
    }

    // It would be more efficient to take a parsed SVG object, but nsvg deals in raw pointers to a
    // private type.
    fn rasterize(svg_path: &str, svg_dpi: f32, height_in_pixels: u16) -> Image {
        let scale = height_in_pixels as f32 / SVG_HEIGHT as f32;
        let svg = nsvg::parse_file(svg_path, "px", svg_dpi);
        let img = nsvg::rasterize(svg, scale);

        Image {
            width: img.width(),
            height: img.height(),
            data: img.into_raw(),
        }
    }

    fn create_ico(ico_path: &Path, pngs: Vec<Image>) -> io::Result<()> {
        let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

        for png in pngs {
            let image = ico::IconImage::from_rgba_data(png.width, png.height, png.data);
            icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);
        }

        if let Some(p) = ico_path.parent() {
            create_dir_all(p)?;
        }

        let file = File::create(ico_path)?;
        icon_dir.write(file)
    }
}

#[cfg(unix)]
fn main() {}
