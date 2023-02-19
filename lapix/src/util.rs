use crate::{color, Bitmap, Color};

pub fn load_img_from_file(path: &str) -> image::RgbaImage {
    use image::io::Reader as ImageReader;
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    img.into_rgba8()
}

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

pub fn save_image<IMG: Bitmap>(image: IMG, path: &str) {
    let bytes = image.bytes();

    let img = image::RgbaImage::from_raw(
        image.width() as u32,
        image.height() as u32,
        bytes.to_owned(),
    )
    .expect("Failed to generate image from bytes");
    img.save(path).expect("Failed to save image");
}
