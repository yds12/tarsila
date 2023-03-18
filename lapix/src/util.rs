use crate::{color, Bitmap, Color};
use image::{codecs, ImageEncoder, ImageFormat, ImageOutputFormat};

/// Load an image from a file in the specified path
pub fn load_img_from_file(path: &str) -> image::RgbaImage {
    use image::io::Reader as ImageReader;
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    img.into_rgba8()
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
pub fn save_image<IMG: Bitmap>(bitmap: IMG, path: &str) {
    let bytes = bitmap.bytes();
    let width = bitmap.width() as u32;
    let height = bitmap.height() as u32;
    let color = image::ColorType::Rgba8;

    let file = std::fs::File::create(path).expect("Failed to create file from path");
    let buffer = std::io::BufWriter::new(file);

    let result = match ImageFormat::from_path(path)
        .unwrap_or(ImageFormat::Png)
        .into()
    {
        ImageOutputFormat::Png => {
            codecs::png::PngEncoder::new(buffer).write_image(bytes, width, height, color)
        }
        ImageOutputFormat::Jpeg(_) => codecs::jpeg::JpegEncoder::new_with_quality(buffer, 100)
            .write_image(bytes, width, height, color),
        _ => panic!("File type is not supported"),
    };

    result.expect("Failed to save image");
}
