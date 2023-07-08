use std::{f32::consts::{E, PI}, iter::Map, collections::HashMap};
use image::{GrayImage, Pixel, Rgb, Luma};
use num::integer::Roots;
use crate::utils::{_2d_array_to_vec, GAUSS_SMOOTH, matrix_multiply, SOBEL_X, SOBEL_Y, DIR_MAT_Y, DIR_MAT_X};

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

// see https://ieeexplore.ieee.org/document/4309999
// page 6. Equation 1.
// 'size' is the size of the neighborhood calculated as 2^k
pub fn neighborhood_average(pixels: &Vec<Vec<u8>>, x: i32, y: i32, size: u32) -> f32 {
    let mut out = 0.0f32;
    let (width, height) = (pixels.len(), pixels[0].len());
    for i in (x - 2i32.pow(size - 1))..(x + 2i32.pow(size - 1) - 1) {
        for j in (y - 2i32.pow(size - 1))..(y + 2i32.pow(size - 1) - 1) {
            // not a word has been said about going out of bounds in the paper but let's assume that it counts as 0
            if i < 0 || j < 0  || i >= width as i32 || j >= height as i32 {
                continue;
            }
            out += pixels[i as usize][j as usize] as f32;
        }
    }
    out / 2u32.pow(2 * size) as f32
}

// see https://ieeexplore.ieee.org/document/4309999
// page 6. Equation 2, 3 and 4.
// 'size' is the size of the neighborhood calculated as 2^k
pub fn s_best(pixels: &Vec<Vec<u8>>, x: i32, y: i32) -> u32 {
    let mut e_vec: Vec<(f32, u8)> = Vec::new();
    for size in 1..5 {
        let e_1 = neighborhood_average(pixels, x + 2i32.pow(size - 1), y, size);
        let e_2 = neighborhood_average(pixels, x - 2i32.pow(size - 1), y, size);
        e_vec.push(((e_1 - e_2).abs(), size as u8));
    }

    // pick best and save the size
    let mut best = e_vec[0];
    for e in e_vec {
        if e.0 < best.0 {
            best = e;
        }
    }

    best.1 as u32
}

pub fn coarseness(pixels: &GrayImage) -> f32 {
    let mut out = 0.0f32;
    let (width, height) = pixels.dimensions();

    // todo get a vec of pixels to avoid using get_pixels
    let mut pixels_vec: Vec<Vec<u8>> = Vec::new();

    for i in 0..width {
        let mut row: Vec<u8> = Vec::new();
        for j in 0..height {
            row.push(pixels[(i, j)][0]);
        }
        pixels_vec.push(row);
    }

    for i in 0..width {
        for j in 0..height {
            let s_best = s_best(&pixels_vec, i as i32, j as i32);
            out += s_best as f32;
        }
    }

    out / (width * height) as f32
}

// DIRECTIONALITY

// page 8 first equation on the right
pub fn quantized_peaks(vec: &Vec<f32>, n: i32) -> Vec<f32> {
    let mut divisor: f32 = 0.0;
    // calculating the divisor that will be reused for all peaks
    for k in 0..(n - 1) {
        for val in vec.iter() {
            if *val >= (2.0 * (k as f32) - 1.0) * 2.0 * (n as f32) && 
            *val < (2.0 * (k as f32) + 1.0) * 2.0 * (n as f32) {
                divisor += 1.0;
            }
        }
    }

    let mut histogram = vec![0.0; n as usize];
    for k in 0..(n - 1) {
        let mut count = 0;
        for val in vec.iter() {
            if *val >= (2.0 * (k as f32) - 1.0) *  PI / (2.0 * (n as f32)) && 
            *val < (2.0 * (k as f32) + 1.0) *  PI / (2.0 * (n as f32)) {
                count += 1;
            }
        }
        histogram[k as usize] = count as f32 / divisor;
    }

    // print
    for i in 0..histogram.len() {
        println!("{}: {}", i, histogram[i]);
    }

    histogram
}

#[derive(Clone, Copy)]
struct Peak {
    value: f32,
    idx: i32,
    range: (i32, i32)
}

fn find_peaks(vec: &Vec<f32>) -> Vec<Peak> {
    let mut idx_vec: Vec<i32> = Vec::new();
    for i in 0..vec.len() {
        idx_vec.push(i as i32);
    }

    // sort the idx_vec by the values in vec
    idx_vec.sort_by(|a, b| vec[*a as usize].partial_cmp(&vec[*b as usize]).unwrap());

    let mut out: Vec<Peak> = Vec::new();
    let mut peak1 = Peak {value: vec[idx_vec[idx_vec.len() - 1] as usize], idx: *idx_vec.last().unwrap(), range: (0, 0)};
    let mut peak2 = Peak {value: vec[idx_vec[idx_vec.len() - 2] as usize], idx: idx_vec[idx_vec.len() - 2], range: (0, 0)};

    // here we gather data about the peaks and range of the peaks
    if peak1.idx < peak2.idx {
        for i in (0..peak1.idx).rev() {
            if i == 0 {
                peak1.range.0 = 0;
                break;
            }
            if vec[(i - 1) as usize] > vec[i as usize] || vec[(i - 1) as usize] == vec[i as usize]{
                peak1.range.0 = i;
                break;
            }
        }
        for i in peak1.idx..peak2.idx {
            if vec[(i + 1) as usize] > vec[i as usize] || vec[(i + 1) as usize] == vec[i as usize] {
                peak1.range.1 = i;
                break;
            }
        }

        for i in (0..peak2.idx).rev() {
            if vec[(i - 1) as usize] > vec[i as usize] || vec[(i - 1) as usize] == vec[i as usize] {
                peak2.range.0 = i;
                break;
            }
        }
        for i in peak2.idx..(idx_vec.len() as i32) {
            if i == (idx_vec.len() - 1) as i32 {
                peak2.range.1 = (idx_vec.len() - 1) as i32;
                break;
            }
            if vec[(i + 1) as usize] > vec[i as usize] || vec[(i + 1) as usize] == vec[i as usize] {
                peak2.range.1 = i;
                break;
            }
        }
    } else {
        for i in (0..peak2.idx).rev() {
            if i == 0 {
                peak2.range.0 = 0;
                break;
            }
            if vec[(i - 1) as usize] > vec[i as usize] || vec[(i - 1) as usize] == vec[i as usize] {
                peak2.range.0 = i;
                break;
            }
        }
        for i in peak2.idx..peak1.idx {
            if vec[(i + 1) as usize] > vec[i as usize] || vec[(i + 1) as usize] == vec[i as usize] {
                peak2.range.1 = i;
                break;
            }
        }

        for i in (0..peak1.idx).rev() {
            if vec[(i - 1) as usize] > vec[i as usize] || vec[(i - 1) as usize] == vec[i as usize] {
                peak1.range.0 = i;
                break;
            }
        }
        for i in peak1.idx..(idx_vec.len() as i32) {
            if i == (idx_vec.len() - 1) as i32 {
                peak1.range.1 = (idx_vec.len() - 1) as i32;
                break;
            }
            if vec[(i + 1) as usize] > vec[i as usize] || vec[(i + 1) as usize] == vec[i as usize] {
                peak1.range.1 = i;
                break;
            }
        }
    }
    out.push(peak1.clone());
    // cheack if we should consider the second peak at all
    if vec[peak2.range.0 as usize] / peak2.value < 0.5 && vec[peak2.range.1 as usize] / peak2.value < 0.5 {
        if peak2.value / peak1.value > 0.2 {
            out.push(peak2);
        } 
    }

    //print
    for i in 0..out.len() {
        println!("peak {}: value: {}, idx: {}, range: {:?}", i, out[i].value, out[i].idx, out[i].range);
    }

    out
}

pub fn directionality(pixels: &GrayImage, threshold: f32, n: i32) -> f32 {
    // calculate direction of edge at each pixel
    let mut out = 0.0f32;
    let (width, height) = pixels.dimensions();

    // todo get a vec of pixels to avoid using get_pixels
    let mut pixels_vec: Vec<Vec<u8>> = Vec::new();
    let mut angles: Vec<f32> = Vec::new();
    let mut image_out = GrayImage::new(width, height);

    for i in 0..width {
        let mut row: Vec<u8> = Vec::new();
        for j in 0..height {
            row.push(pixels[(i, j)][0]);
        }
        pixels_vec.push(row);
    }

    // convolution
    for x in 0..(width - 2) {
        for y in 0..(height - 2) {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;

            for i in 0..3 {
                for j in 0..3 {
                    sum_x += pixels_vec[(x + i) as usize][(y + j) as usize] as f32 * DIR_MAT_X[i as usize][j as usize];
                    sum_y += pixels_vec[(x + i) as usize][(y + j) as usize] as f32 * DIR_MAT_Y[i as usize][j as usize];
                }
            }

            sum_x = sum_x.abs();
            sum_y = sum_y.abs();

            let pixel_val = (sum_x.powf(2.0) + sum_y.powf(2.0)).sqrt();
            image_out.put_pixel(x, y, Luma([pixel_val as u8]));

            // thats what i assume was meant by thresholding so that we dont count insignificant data
            if sum_x < threshold && sum_y < threshold {
                continue;
            }

            let arg = sum_y / sum_x;

            let angle = arg.atan() + PI / 2.0;
            angles.push(angle);
        }
    }

    let hd = quantized_peaks(&angles, n);
    let peaks = find_peaks(&hd);

    // the following calculations are slightly altered from the original paper.
    // 1. some values i simply did not understen the meaning or purpose of
    // 2. the leading '1 - ' makes no sense since higher directionality should meam values closer to 1 not the other way
    
    // the way it works now is that 
    let mut temp = 0.0;
    for i in 0..peaks.len() {
        let range_len = peaks[i].range.1 - peaks[i].range.0;
        for j in peaks[i].range.0..(peaks[i].range.1 + 1) {
            temp += (j - peaks[i].idx).pow(2) as f32 * peaks[i].value / range_len as f32;
        }
        
    }

    out = peaks.len() as f32 * temp;

    out
}