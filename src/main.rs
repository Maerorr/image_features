mod image_process;
mod utils;

use std::io::Cursor;
use image::io::Reader as ImageReader;
use image_process::*;
use crate::utils::normalize_value;

fn main() {
    // let color_noise_img = ImageReader::open("res/color_noise.png").unwrap().decode().unwrap();
    // let mono_noise_img = ImageReader::open("res/mono_noise.png").unwrap().decode().unwrap();
    // let flat_img = ImageReader::open("res/flat.png").unwrap().decode().unwrap();
    //
    let modular_img = ImageReader::open("res/modular.png").unwrap().decode().unwrap();
    let mountains_img = ImageReader::open("res/mountains.png").unwrap().decode().unwrap();
    let warehouse_img = ImageReader::open("res/warehouse.png").unwrap().decode().unwrap();

    let modular_pixels = modular_img.to_rgb32f();
    let mountains_pixels = mountains_img.to_rgb32f();
    let warehouse_pixels = warehouse_img.to_rgb32f();

    // get pixel data into an 2d array of pixels
    //println!("converting images to 2d arrays . . .");
    // let color_noise_pixels = color_noise_img.to_rgb32f();
    // let mono_noise_pixels = mono_noise_img.to_rgb32f();
    // let flat_pixels = flat_img.to_rgb32f();

    // println!("calculating noisyness... (1 = max noise, 0 = no noise)");
    //
    // println!("calculated noisyness of color noise: {}", normalize_value(measure_rms_noise(&color_noise_pixels, 3), 0.0, 0.5));
    //println!("calculated noisyness of mono noise: {}", normalize_value(measure_rms_noise(&mono_noise_pixels, 3), 0.0, 0.5));
    // println!("calculated noisyness of flat: {}", normalize_value(measure_rms_noise(&flat_pixels, 3), 0.0, 0.5));
    println!("calculated noisyness of modular: {}", normalize_value(measure_rms_noise(&modular_pixels, 50), 0.0, 0.5));
    println!("calculated noisyness of mountains: {}", normalize_value(measure_rms_noise(&mountains_pixels, 50), 0.0, 0.5));
    println!("calculated noisyness of warehouse: {}", normalize_value(measure_rms_noise(&warehouse_pixels, 50), 0.0, 0.5));


    // SOBEL EDGE DETECTION TEST


    let mut mountains_edges = sobel_convolution(&mountains_pixels);
    mountains_edges = apply_threshold(&mountains_edges, 0.1);
    let mut modular_edges = sobel_convolution(&modular_pixels);
    modular_edges = apply_threshold(&modular_edges, 0.1);
    let mut warehouse_edges = sobel_convolution(&warehouse_pixels);
    warehouse_edges = apply_threshold(&warehouse_edges, 0.1);


    let imgbuf_mountains = image::ImageBuffer::from_fn(mountains_edges.width(), mountains_edges.height(), |x, y| {
        let pixel = mountains_edges.get_pixel(x, y);
        let r = (pixel[0] * 256.0) as u8;
        let g = (pixel[1] * 256.0) as u8;
        let b = (pixel[2] * 256.0) as u8;
        image::Rgb([r, g, b])
    });

    let imgbuf_modular = image::ImageBuffer::from_fn(modular_edges.width(), modular_edges.height(), |x, y| {
        let pixel = modular_edges.get_pixel(x, y);
        let r = (pixel[0] * 256.0) as u8;
        let g = (pixel[1] * 256.0) as u8;
        let b = (pixel[2] * 256.0) as u8;
        image::Rgb([r, g, b])
    });

    let imgbuf_warehouse = image::ImageBuffer::from_fn(warehouse_edges.width(), warehouse_edges.height(), |x, y| {
        let pixel = warehouse_edges.get_pixel(x, y);
        let r = (pixel[0] * 256.0) as u8;
        let g = (pixel[1] * 256.0) as u8;
        let b = (pixel[2] * 256.0) as u8;
        image::Rgb([r, g, b])
    });

    imgbuf_mountains.save("res/output/edged_mountains.png").unwrap();
    imgbuf_modular.save("res/output/edged_modular.png").unwrap();
    imgbuf_warehouse.save("res/output/edged_warehouse.png").unwrap();
}
