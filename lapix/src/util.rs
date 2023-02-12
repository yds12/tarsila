use crate::Bitmap;

pub fn load_img_from_file(path: &str) -> image::RgbaImage {
    use image::io::Reader as ImageReader;
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    img.into_rgba8()
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
