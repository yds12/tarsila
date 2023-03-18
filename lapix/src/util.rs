use crate::{color, Bitmap, Color};

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
    use image::ImageFormat as Format;

    let image_format =
        Format::from_extension(std::path::Path::new(path).extension().unwrap_or_default())
            .unwrap_or(Format::Png);

    let bytes = bitmap.bytes();

    let img = image::RgbaImage::from_raw(
        bitmap.width() as u32,
        bitmap.height() as u32,
        bytes.to_owned(),
    )
    .expect("Failed to generate image from bytes");
    img.save_with_format(path, image_format)
        .expect("Failed to save image");
}
