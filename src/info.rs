use crate::color::ColorInfo;
use image::{DynamicImage, ImageFormat};
use std::fs::metadata;
use std::path::PathBuf;


pub fn print_info(image: &DynamicImage, path: PathBuf, do_short: bool) {
    let (height, width) = (image.height(), image.width());
    let format = ImageFormat::from_path(&path).expect("Error decoding image format: ");

    let meta = metadata(&path).unwrap();
    let size = meta.len();

    let color_info = ColorInfo::from_image(image);

    println!("Image file: {:?}", path.as_os_str());
    println!("File size: {} bytes", size);
    println!("Dimensions: {}x{}", width, height);
    println!("Format: {}", format.to_mime_type());

    if !do_short {
        println!("Color space: {}", color_info.color_type);
        println!("Bit depth: {}", color_info.bit_depth);
        println!("Alpha: {}", color_info.is_alpha);
    }
}
