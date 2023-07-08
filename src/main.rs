mod image_process;
mod utils;
mod colorfulness;

use std::io::Cursor;
use image::io::Reader as ImageReader;
use image_process::directionality;

use crate::colorfulness::{lab_to_rgb, lab_to_rgb_image, colorfulness_metrics_1_3, rgb_to_lab_image, colorfulness_metrics_2, lab_saturation, grayscale_sd, grayscale, count_unique_colors, posterize};
use crate::image_process::{coarseness, edge_pixels_ratio, sobel_convolution};
use crate::utils::{normalize_value, save_to_image, save_to_image_f32};

fn main() {
    let image = ImageReader::open("res/urban.jpg").unwrap().decode().unwrap();

    let image_f32 = image.to_rgb32f();

    let image_u8 = image.to_rgb8();

    let image_grayscale = image.to_luma8();

    let image_lab = rgb_to_lab_image(&image_f32);

    // let urban_coarseness = coarseness(&urban_grayscale);
    // println!("calculated coarseness of urban image");

    // println!("------- Coarseness -------");
    // println!("urban: {}", urban_coarseness);

    // let urban_met_1 = colorfulness_metrics_1_3(&urban_lab);
    
    // println!("\n------- Metric One -------");
    // println!("urban: {}, {}", urban_met_1.0, urban_met_1.1);

    // let urban_met_2 = colorfulness_metrics_2(&urban_lab);
    // println!("\n------- Metric Two -------");
    // println!("urban: {}", urban_met_2);

    // let urban_grayscale_sd = grayscale_sd(grayscale(&urban_p));

    // println!("\n------- GrayscaleSD -------");
    // println!("urban grayscale sd: {}", urban_grayscale_sd);
    
    // let urban_posterized = posterize(&urban_u8, 6);
    
    // urban_posterized.save("res/output/urban_posterized.png").unwrap();
    
    // let urban_p_colours = count_unique_colors(&urban_posterized);
    
    // println!("\n------- PColours -------");
    // println!("urban PColours: {}", urban_p_colours);
    
    // let urban_blurred = imageproc::filter::gaussian_blur_f32(&urban_grayscale, 2.0);
    
    // urban_blurred.save("res/output/urban_blurred.png").unwrap();
    
    //let edged = imageproc::edges::canny(&image_grayscale, 1.0, 27.0);
    
    // let urban_edge_density = edge_pixels_ratio(urban_edged);
    
    // println!("\n------- Edge Density -------");
    // println!("urban edge density: {}", urban_edge_density);

    let dir = directionality(&image_grayscale, 0.12, 16);
    print!("dir: {}", dir);


}
