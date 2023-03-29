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

    let modular_img = ImageReader::open("res/modular.png").unwrap().decode().unwrap();
    let mountains_img = ImageReader::open("res/mountains.png").unwrap().decode().unwrap();


    // get pixel data into an 2d array of pixels
    println!("converting images to 2d arrays . . .");
    // let color_noise_pixels = color_noise_img.to_rgb32f();
    // let mono_noise_pixels = mono_noise_img.to_rgb32f();
    // let flat_pixels = flat_img.to_rgb32f();

    println!("calculating noisyness... (1 = max noise, 0 = no noise)");

    // println!("normalized std dev of color noise: {}", normalize_value(measure_rms_noise(&color_noise_pixels, 3), 0.0, 0.5));
    // println!("normalized std dev of mono noise: {}", normalize_value(measure_rms_noise(&mono_noise_pixels, 3), 0.0, 0.5));
    // println!("normalized std dev of flat: {}", normalize_value(measure_rms_noise(&flat_pixels, 3), 0.0, 0.5));

    let modular_pixels = modular_img.to_rgb32f();
    let mountains_pixels = mountains_img.to_rgb32f();

    println!("calculated noisyness of modular: {}", normalize_value(measure_rms_noise(&modular_pixels, 50), 0.0, 0.5));
    println!("calculated noisyness of mountains: {}", normalize_value(measure_rms_noise(&mountains_pixels, 50), 0.0, 0.5));
}
