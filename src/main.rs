mod image_process;
mod utils;
mod colorfulness;

use std::io::Cursor;
use image::io::Reader as ImageReader;

use crate::colorfulness::{lab_to_rgb, lab_to_rgb_image, colorfulness_metrics_1, rgb_to_lab_image, colorfulness_metrics_2, lab_saturation, grayscale_sd, grayscale, count_unique_colors, posterize};
use crate::utils::{normalize_value, save_to_image, save_to_image_f32};

fn main() {
    let eye_img = ImageReader::open("res/ladder.png").unwrap().decode().unwrap();

    let eye_p = eye_img.to_rgb32f();
    let eye_u8 = eye_img.to_rgb8();
    let eye_lab = rgb_to_lab_image(&eye_p);

    let metrics_1 = colorfulness_metrics_1(&eye_lab);
    println!("metric 1: {}, {}", metrics_1.0, metrics_1.1);
    let metric_2 = colorfulness_metrics_2(&eye_lab);
    println!("metric 2: {}", metric_2);
    let grayscale_sd = grayscale_sd(grayscale(&eye_p));
    println!("grayscale sd: {}", grayscale_sd);
    let posterized = posterize(&eye_u8, 6);
    let p_colours = count_unique_colors(&posterized);
    println!("PColours: {}", p_colours);

    save_to_image(&posterized, "res/output/posterized.png");
}
