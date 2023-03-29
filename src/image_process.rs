use image::{Pixel, Rgb};
use crate::utils::{_2d_array_to_vec, matrix_multiply, SOBEL_X, SOBEL_Y};

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

pub fn image_pixel_standard_deviation(pixels: &image::Rgb32FImage) -> f32 {
    // let mut output = 0.0;
    // let mut count = 0;
    // let (width, height) = pixels.dimensions();
    //
    // for x in 0..(width - 2) {
    //     for y in 0..(height - 2) {
    //
    //         let mut sum = 0.0;
    //         for i in 0..3 {
    //             for j in 0..3 {
    //                 let pix = pixels.get_pixel(x + i, y + j);
    //                 sum += (pix[0] + pix[1] + pix[2]) / 3.0;
    //             }
    //         }
    //
    //         output += sum / 9.0;
    //         count += 1;
    //     }
    // }
    //
    // output /= (count) as f32;
    // output

    let mut std_dev = 0.0;
    let mut avg = 0.0;
    let (width, height) = pixels.dimensions();

    for x in 0..width {
        for y in 0..height {
            let pix = pixels.get_pixel(x, y);
            avg += (pix[0] + pix[1] + pix[2]) / 3.0;
        }
    }
    avg /= (width * height) as f32;

    for x in 0..width {
        for y in 0..height {
            let pix = pixels.get_pixel(x, y);
            std_dev += (avg - ((pix[0] + pix[1] + pix[2]) / 3.0)).powi(2);
        }
    }

    std_dev /= (width * height) as f32;
    std_dev = std_dev.sqrt();
    std_dev
}

// this measures the RMS Noise level of an image using standard deviation of 3x3 matrices of pixels
pub fn measure_rms_noise(pixels: &image::Rgb32FImage, matrix_size: u32) -> f32 {
    let mut output = 0.0;
    let mut count = 0;
    let (width, height) = pixels.dimensions();

    if matrix_size >= width || matrix_size >= height {
        panic!("matrix size is too large for image");
    }

    for x in (0..(width - (matrix_size - 1))).step_by(matrix_size as usize) {
        for y in (0..(height - (matrix_size - 1))).step_by(matrix_size as usize) {

            let mut sum = 0.0;
            let mut vec: Vec<Vec<Rgb<f32>>> = Vec::new();
            for i in 0..matrix_size {
                let mut row: Vec<Rgb<f32>> = Vec::new();
                for j in 0..matrix_size {
                    let pix = pixels.get_pixel(x + i, y + j);
                    sum += (pix[0] + pix[1] + pix[2]) / 3.0;
                    row.push(*pix);
                }
                vec.push(row);
            }
            let avg = sum / (matrix_size * matrix_size) as f32;

            // calculate standard deviation
            let mut std_dev = 0.0;
            for i in 0..matrix_size {
                for j in 0..matrix_size {
                    let pix = vec[i as usize][j as usize];
                    std_dev += (avg - ((pix[0] + pix[1] + pix[2]) / 3.0)).powi(2);
                }
            }
            std_dev /= (matrix_size * matrix_size) as f32;
            std_dev = std_dev.sqrt();
            output += std_dev;

            count += 1;
        }
    }

    output /= (count) as f32;
    output
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
            //sum_x = sum_x.abs();
            //sum_y = sum_y.abs();
            if sum_x < 0.0 {
                sum_x = 0.0;
            }

            if sum_y < 0.0 {
                sum_y = 0.0;
            }

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