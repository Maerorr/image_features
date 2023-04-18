use std::f32::consts::{E, PI};
use image::{GrayImage, Pixel, Rgb};
use num::complex::ComplexFloat;
use crate::utils::{_2d_array_to_vec, GAUSS_SMOOTH, matrix_multiply, SOBEL_X, SOBEL_Y};

pub fn rgb_image_to_2d_vec(pixels: &image::RgbImage) -> Vec<Vec<Rgb<u8>>> {
    let mut out: Vec<Vec<Rgb<u8>>> = Vec::new();
    for x in 0..pixels.width() {
        let mut row: Vec<Rgb<u8>> = Vec::new();
        for y in 0..pixels.height() {
            row.push(*pixels.get_pixel(x, y));
        }
        out.push(row);
    }
    out
}

pub fn sobel_convolution(pixels: &image::Rgb32FImage) -> image::Rgb32FImage {
    let mut output = image::Rgb32FImage::new(pixels.width(), pixels.height());
    let (width, height) = pixels.dimensions();

    for x in 0..(width - 2) {
        for y in 0..(height - 2) {

            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for i in 0..3 {
                for j in 0..3 {
                    let pix = pixels.get_pixel(x + i, y + j);
                    sum_x += ((pix[0] + pix[1] + pix[2]) / 3.0) * SOBEL_X[i as usize][j as usize];
                    sum_y += ((pix[0] + pix[1] + pix[2]) / 3.0) * SOBEL_Y[i as usize][j as usize];
                }
            }
            sum_x = sum_x.abs();
            sum_y = sum_y.abs();

            sum_x = sum_x.powi(2);
            sum_y = sum_y.powi(2);

            let sum = (sum_x.powi(2) + sum_y.powi(2)).sqrt();
            let pix = Rgb::<f32>([sum, sum, sum]);
            output.put_pixel(x + 1, y + 1, pix);
        }
    }

    output
}

pub fn apply_threshold(pixels: &image::Rgb32FImage, threshold: f32) -> image::Rgb32FImage {
    let mut output = image::Rgb32FImage::new(pixels.width(), pixels.height());
    let (width, height) = pixels.dimensions();

    for x in 0..width {
        for y in 0..height {
            let pix = pixels.get_pixel(x, y);
            let sum = (pix[0] + pix[1] + pix[2]) / 3.0;
            if sum > threshold {
                output.put_pixel(x, y, Rgb::<f32>([1.0, 1.0, 1.0]));
            } else {
                output.put_pixel(x, y, Rgb::<f32>([0.0, 0.0, 0.0]));
            }
        }
    }

    output
}

pub fn apply_fuzzy_threshold(pixels: &image::Rgb32FImage, t_low: f32, t_high: f32) -> Vec<image::Rgb32FImage> {
    let mut level1 = image::Rgb32FImage::new(pixels.width(), pixels.height());
    let mut level2 = image::Rgb32FImage::new(pixels.width(), pixels.height());
    let mut level3 = image::Rgb32FImage::new(pixels.width(), pixels.height());
    let (width, height) = pixels.dimensions();

    for x in 0..width {
        for y in 0..height {
            let pix = pixels.get_pixel(x, y);
            let sum = (pix[0] + pix[1] + pix[2]) / 3.0;
            if sum > t_high {
                level1.put_pixel(x, y, Rgb::<f32>([1.0, 1.0, 1.0]));
                level2.put_pixel(x, y, Rgb::<f32>([0.0, 0.0, 0.0]));
                level3.put_pixel(x, y, Rgb::<f32>([0.0, 0.0, 0.0]));
            } else if sum > t_low {
                level1.put_pixel(x, y, Rgb::<f32>([0.0, 0.0, 0.0]));
                level2.put_pixel(x, y, Rgb::<f32>([1.0, 1.0, 1.0]));
                level3.put_pixel(x, y, Rgb::<f32>([0.0, 0.0, 0.0]));
                // prevents completely white images
            } else if sum > 0.01 {
                level1.put_pixel(x, y, Rgb::<f32>([0.0, 0.0, 0.0]));
                level2.put_pixel(x, y, Rgb::<f32>([0.0, 0.0, 0.0]));
                level3.put_pixel(x, y, Rgb::<f32>([1.0, 1.0, 1.0]));
            }
        }
    }

    vec![level1, level2, level3]
}

pub fn edge_pixels_ratio(pixels: GrayImage) -> f32 {
    let mut white = 0;
    let (width, height) = pixels.dimensions();

    for x in 0..width {
        for y in 0..height {
            let pix = pixels.get_pixel(x, y);
            if pix[0] > 245 {
                white += 1;
            }
        }
    }

    white as f32 / (pixels.width() * pixels.height()) as f32
}