mod image_process;
mod utils;
mod colorfulness;

use std::io::Cursor;
use image::io::Reader as ImageReader;

use crate::colorfulness::{lab_to_rgb, lab_to_rgb_image, colorfulness_metrics_1, rgb_to_lab_image, colorfulness_metrics_2, lab_saturation, grayscale_sd, grayscale, count_unique_colors, posterize};
use crate::image_process::{edge_pixels_ratio};
use crate::utils::{normalize_value, save_to_image, save_to_image_f32};

fn main() {
    let instant = std::time::Instant::now();
    let dull = ImageReader::open("res/dull.jpg").unwrap().decode().unwrap();
    let colorful = ImageReader::open("res/colorful.jpg").unwrap().decode().unwrap();
    let dark = ImageReader::open("res/dark.jpg").unwrap().decode().unwrap();

    let duration = instant.elapsed();
    println!("Time elapsed for loading images is: {:?}", duration);

    let dull_p = dull.to_rgb32f();
    let colorful_p = colorful.to_rgb32f();
    let dark_p = dark.to_rgb32f();

    let dull_u8 = dull.to_rgb8();
    let colorful_u8 = colorful.to_rgb8();
    let dark_u8 = dark.to_rgb8();

    let dull_grayscale = dull.to_luma8();
    let colorful_grayscale = colorful.to_luma8();
    let dark_grayscale = dark.to_luma8();

    let dull_lab = rgb_to_lab_image(&dull_p);
    let colorful_lab = rgb_to_lab_image(&colorful_p);
    let dark_lab = rgb_to_lab_image(&dark_p);

    let duration = instant.elapsed();
    println!("Time elapsed for converting images is: {:?}", duration);

    let dull_met_1 = colorfulness_metrics_1(&dull_lab);
    let colorful_met_1 = colorfulness_metrics_1(&colorful_lab);
    let dark_met_1 = colorfulness_metrics_1(&dark_lab);

    println!("\n------- Metric One -------");
    println!("dull: {}, {}", dull_met_1.0, dull_met_1.1);
    println!("colorful: {}, {}", colorful_met_1.0, colorful_met_1.1);
    println!("dark: {}, {}", dark_met_1.0, dark_met_1.1);

    let dull_met_2 = colorfulness_metrics_2(&dull_lab);
    let colorful_met_2 = colorfulness_metrics_2(&colorful_lab);
    let dark_met_2 = colorfulness_metrics_2(&dark_lab);
    println!("\n------- Metric Two -------");
    println!("dull: {}", dull_met_2);
    println!("colorful: {}", colorful_met_2);
    println!("dark: {}", dark_met_2);

    let dull_grayscale_sd = grayscale_sd(grayscale(&dull_p));
    let colorful_grayscale_sd = grayscale_sd(grayscale(&colorful_p));
    let dark_grayscale_sd = grayscale_sd(grayscale(&dark_p));

    println!("\n------- GrayscaleSD -------");
    println!("dull grayscale sd: {}", dull_grayscale_sd);
    println!("colorful grayscale sd: {}", colorful_grayscale_sd);
    println!("dark grayscale sd: {}", dark_grayscale_sd);

    let dull_posterized = posterize(&dull_u8, 6);
    let colorful_posterized = posterize(&colorful_u8, 6);
    let dark_posterized = posterize(&dark_u8, 6);

    let dull_p_colours = count_unique_colors(&dull_posterized);
    let colorful_p_colours = count_unique_colors(&colorful_posterized);
    let dark_p_colours = count_unique_colors(&dark_posterized);

    println!("\n------- PColours -------");
    println!("dull PColours: {}", dull_p_colours);
    println!("colorful PColours: {}", colorful_p_colours);
    println!("dark PColours: {}", dark_p_colours);

    let dull_blurred = imageproc::filter::gaussian_blur_f32(&dull_grayscale, 2.0);
    let colorful_blurred = imageproc::filter::gaussian_blur_f32(&colorful_grayscale, 2.0);
    let dark_blurred = imageproc::filter::gaussian_blur_f32(&dark_grayscale, 2.0);

    dull_blurred.save("res/output/dull_blurred.png").unwrap();
    colorful_blurred.save("res/output/colorful_blurred.png").unwrap();
    dark_blurred.save("res/output/dark_blurred.png").unwrap();

    let dull_edged = imageproc::edges::canny(&dull_blurred, 1.0, 27.0);
    let colorful_edged = imageproc::edges::canny(&colorful_blurred, 1.0, 27.0);
    let dark_edged = imageproc::edges::canny(&dark_blurred, 1.0, 27.0);

    dull_edged.save("res/output/dull_edged.png").unwrap();
    colorful_edged.save("res/output/colorful_edged.png").unwrap();
    dark_edged.save("res/output/dark_edged.png").unwrap();

    let dull_edge_density = edge_pixels_ratio(dull_edged);
    let colorful_edge_density = edge_pixels_ratio(colorful_edged);
    let dark_edge_density = edge_pixels_ratio(dark_edged);

    println!("\n------- Edge Density -------");
    println!("dull edge density: {}", dull_edge_density);
    println!("colorful edge density: {}", colorful_edge_density);
    println!("dark edge density: {}", dark_edge_density);

    let duration = instant.elapsed();
    println!("\n--------------------------------------------------\n\
    Time elapsed for calculating metrics is: {:?}\
    \n--------------------------------------------------", duration);

}
