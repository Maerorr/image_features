use std::collections::HashSet;
use float_cmp::approx_eq;
use image::{Rgb32FImage, RgbImage};
use crate::utils::{mean, std_dev, std_dev_2d_vec};

// L ranges from 0 to 100
// a ranges from -128 to 127
// b ranges from -128 to 127
pub struct LabPixel {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl LabPixel {
    fn new(l: f32, a: f32, b: f32) -> LabPixel {
        LabPixel { l, a, b }
    }
}

// converts a pixel from RGB to cieLAB
pub fn rgb_to_lab(r: f32, g: f32, b: f32) -> LabPixel {
    let x = 0.412453 * r + 0.357580 * g + 0.180423 * b;
    let y = 0.212671 * r + 0.715160 * g + 0.072169 * b;
    let z = 0.019334 * r + 0.119193 * g + 0.950227 * b;

    let x = x / 0.950456;
    let y = y / 1.0;
    let z = z / 1.088754;

    let x = if x > 0.008856 { x.powf(1.0 / 3.0) } else { 7.787 * x + 16.0 / 116.0 };
    let y = if y > 0.008856 { y.powf(1.0 / 3.0) } else { 7.787 * y + 16.0 / 116.0 };
    let z = if z > 0.008856 { z.powf(1.0 / 3.0) } else { 7.787 * z + 16.0 / 116.0 };

    let l = 116.0 * y - 16.0;
    let a = 500.0 * (x - y);
    let b = 200.0 * (y - z);

    LabPixel::new(l, a, b)
}

pub fn lab_to_rgb(l: f32, a: f32, b: f32) -> [f32; 3] {
    let y = (l + 16.0) / 116.0;
    let x = a / 500.0 + y;
    let z = y - b / 200.0;

    let x = if x.powi(3) > 0.008856 {
        x.powi(3)
    } else {
        (x - 16.0 / 116.0) / 7.787
    };
    let y = if l > 7.9996248 {
        y.powi(3)
    } else {
        l / 903.3
    };
    let z = if z.powi(3) > 0.008856 {
        z.powi(3)
    } else {
        (z - 16.0 / 116.0) / 7.787
    };

    let x = x * 0.950456;
    let y = y * 1.0;
    let z = z * 1.088754;

    let r = 3.240479 * x - 1.537150 * y - 0.498535 * z;
    let g = -0.969256 * x + 1.875992 * y + 0.041556 * z;
    let b = 0.055648 * x - 0.204043 * y + 1.057311 * z;

    [r, g, b]
}

pub fn lab_to_rgb_image(image: &Vec<Vec<LabPixel>>) -> Rgb32FImage {
    let mut output = Rgb32FImage::new(image[0].len() as u32, image.len() as u32);
    for (i, row) in image.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            let rgb = lab_to_rgb(pixel.l, pixel.a, pixel.b);
            output.put_pixel(j as u32, i as u32, image::Rgb(rgb));
        }
    }
    output
}

// converts an image from RGB to an vec of cieLAB pixels
pub fn rgb_to_lab_image(image: &Rgb32FImage) -> Vec<Vec<LabPixel>> {
    let mut output = Vec::new();
    for (i, row) in image.rows().enumerate() {
        let mut row_vec = Vec::new();
        for (j, pixel) in row.enumerate() {
            let rgb = pixel.0;
            let lab = rgb_to_lab(rgb[0], rgb[1], rgb[2]);
            row_vec.push(lab);
        }
        output.push(row_vec);
    }
    output
}

pub fn chroma(lab: &LabPixel) -> f32 {
    (lab.a.powi(2) + lab.b.powi(2)).sqrt()
}

pub fn mean_of_chroma(image: &Vec<Vec<LabPixel>>) -> f32 {
    let mut sum = 0.0;
    for row in image {
        for pixel in row {
            sum += chroma(pixel);
        }
    }
    sum / (image.len() * image[0].len()) as f32
}

// this returns the colorfulness metrics one and three from table one from:
// https://www.researchgate.net/publication/243135534_Measuring_Colourfulness_in_Natural_Images
// metric one is standard deviations of a and b in CIELAB color space + the mean of Chroma
// metric two is the trigonometric len between standard deviations of a and b + the mean of chroma
pub fn colorfulness_metrics_1_3(image: &Vec<Vec<LabPixel>>) -> (f32, f32) {
    let mut output_1 = 0.0f32;
    let mut output_3 = 0.0f32;
    let mean_of_chroma = mean_of_chroma(image);

    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();

    for row in image {
        for pixel in row {
            vec_a.push(pixel.a);
            vec_b.push(pixel.b);
        }
    }

    let std_dev_of_a = std_dev(&vec_a);
    let std_dev_of_b = std_dev(&vec_b);

    let trig_len_of_std_dev = (std_dev_of_a.powi(2) + std_dev_of_b.powi(2)).sqrt();

    if std_dev_of_a > std_dev_of_b {
        output_1 = std_dev_of_a + 1.46 * std_dev_of_b + 1.34 * mean_of_chroma;
    } else {
        output_1 = std_dev_of_b + 1.46 * std_dev_of_a + 1.34 * mean_of_chroma;
    }

    output_3 = trig_len_of_std_dev + 0.94 * mean_of_chroma;

    (output_1, output_3)
}

pub fn lab_saturation(lab: &LabPixel) -> f32 {
    // division by zero check
    if approx_eq!(f32, lab.l, 0.0, (0.0, 2)) {
        return 0.0;
    }
    let c = chroma(lab);
    c / lab.l
}

// calculates colorfulness metric three from table 1 from:
// https://dl.acm.org/doi/pdf/10.1145/2470654.2481281
pub fn colorfulness_metrics_2(image: &Vec<Vec<LabPixel>>) -> f32 {
    let mut output = 0.0f32;

    // calculate saturation of each pixel
    let mut vec_s = Vec::new();
    for row in image {
        for pixel in row {
            vec_s.push(lab_saturation(pixel));
        }
    }

    output = mean(&vec_s) + std_dev(&vec_s);

    output
}

pub fn grayscale(image: &Rgb32FImage) -> Vec<Vec<f32>> {
    let mut output = Vec::new();
    for (i, pixel) in image.pixels().enumerate() {
        let gray = (pixel[0] + pixel[1] + pixel[2]) / 3.0;
        if i % image.width() as usize == 0 {
            output.push(Vec::new());
        }
        output.last_mut().unwrap().push(gray);
    }
    output
}

pub fn grayscale_sd(image: Vec<Vec<f32>>) -> f32 {
    std_dev_2d_vec(&image)
}

pub fn posterize(image: &RgbImage, levels: u8) -> RgbImage {
    let mut output = RgbImage::new(image.width(), image.height());
    let div = 255 / levels;
    for (i, pixel) in image.pixels().enumerate() {
        let r = (pixel[0] / div) * div;
        let g = (pixel[1] / div) * div;
        let b = (pixel[2] / div) * div;
        output.put_pixel(i as u32 % image.width(), i as u32 / image.width(), image::Rgb([r, g, b]));
    }
    output
}

pub fn count_unique_colors(image: &RgbImage) -> usize {
    let mut colors = HashSet::new();
    for pixel in image.pixels() {
        colors.insert(pixel);
    }
    colors.len()
}