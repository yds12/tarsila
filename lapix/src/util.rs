pub fn load_img_from_file(path: &str) -> image::RgbaImage {
    use image::io::Reader as ImageReader;
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    img.into_rgba8()
}
