use crate::{color, Bitmap, Color, Error, Result};
use image::{codecs, ImageEncoder, ImageFormat, ImageOutputFormat};
use std::fmt::Debug;
use std::path::PathBuf;

/// Holds a function that takes a path as input and outputs the bytes of the
/// project file found at that path.
pub struct LoadProject(pub fn(PathBuf) -> Vec<u8>);
impl Debug for LoadProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str("LoadProject(fn(PathBuf) -> Vec<u8>>)")
    }
}
impl PartialEq for LoadProject {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Clone for LoadProject {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl From<fn(PathBuf) -> Vec<u8>> for LoadProject {
    fn from(val: fn(PathBuf) -> Vec<u8>) -> Self {
        Self(val)
    }
}
/// Holds a function that takes a path and a set of bytes as input as saves
/// those bytes as a project file at that path
pub struct SaveProject(pub fn(PathBuf, Vec<u8>));
impl Debug for SaveProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str("SaveProject(fn(PathBuf, Vec<u8>))")
    }
}

impl PartialEq for SaveProject {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Clone for SaveProject {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

/// Load an image from a file in the specified path
pub fn load_img_from_file(path: &str) -> Result<image::RgbaImage> {
    use image::io::Reader as ImageReader;
    let img = ImageReader::open(path)?.decode()?;

    Ok(img.into_rgba8())
}

/// Create an image satisfying [`Bitmap`] from a raw [`image::RgbaImage`]
pub fn img_from_raw<IMG: Bitmap>(raw: image::RgbaImage) -> IMG {
    let mut img = IMG::new(
        (raw.width() as i32, raw.height() as i32).into(),
        color::TRANSPARENT,
    );
    for (x, y, pixel) in raw.enumerate_pixels() {
        let color = Color::new(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
        img.set_pixel((x as i32, y as i32).into(), color);
    }

    img
}

/// Save an image to the specified file path
pub fn save_image<IMG: Bitmap>(bitmap: IMG, path: &str) -> Result<()> {
    let bytes = bitmap.bytes();
    let width = bitmap.width() as u32;
    let height = bitmap.height() as u32;
    let color = image::ColorType::Rgba8;

    let file = std::fs::File::create(path)?;
    let buffer = std::io::BufWriter::new(file);

    match ImageFormat::from_path(path)
        .unwrap_or(ImageFormat::Png)
        .into()
    {
        ImageOutputFormat::Png => {
            codecs::png::PngEncoder::new(buffer).write_image(bytes, width, height, color)?
        }
        ImageOutputFormat::Jpeg(_) => codecs::jpeg::JpegEncoder::new_with_quality(buffer, 100)
            .write_image(bytes, width, height, color)?,
        _ => return Err(Error::UnsupportedImageFormat),
    };

    Ok(())
}
