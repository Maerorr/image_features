use image::Rgb32FImage;

pub fn normalize_value(value: f32, min: f32, max: f32) -> f32 {
    (value - min) / (max - min)
}

pub fn matrix_multiply(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut output: Vec<Vec<f32>> = Vec::new();
    for i in 0..a.len() {
        let mut row: Vec<f32> = Vec::new();
        for j in 0..b[0].len() {
            let mut sum = 0.0;
            for k in 0..a[0].len() {
                sum += a[i][k] * b[k][j];
            }
            row.push(sum);
        }
        output.push(row);
    }
    output
}

pub fn _2d_array_to_vec(array: &[[f32;3];3]) -> Vec<Vec<f32>> {
    let mut output: Vec<Vec<f32>> = Vec::new();
    for i in 0..array.len() {
        let mut row: Vec<f32> = Vec::new();
        for j in 0..array[0].len() {
            row.push(array[i][j]);
        }
        output.push(row);
    }
    output
}

pub fn save_to_image_f32(image: &Rgb32FImage, name: &str) {
    let imgbuf = image::ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        let pixel = image.get_pixel(x, y);
        let r = (pixel[0] * 256.0) as u8;
        let g = (pixel[1] * 256.0) as u8;
        let b = (pixel[2] * 256.0) as u8;
        image::Rgb([r, g, b])
    });

    imgbuf.save(name).unwrap();
}

pub fn save_to_image(image: &image::RgbImage, name: &str) {
    image.save(name).unwrap();
}

pub fn std_dev(values: &Vec<f32>) -> f32 {
    let mean = mean(values);
    let mut sum = 0.0;
    for val in values {
        sum += (val - mean).powi(2);
    }
    (sum / values.len() as f32).sqrt()
}

pub fn std_dev_2d_vec(values: &Vec<Vec<f32>>) -> f32 {
    let mut sum = 0.0;
    for i in 0..values.len() {
        for j in 0..values[0].len() {
            sum += values[i][j].powi(2);
        }
    }
    (sum / (values.len() * values[0].len()) as f32).sqrt()
}

pub fn mean(values: &Vec<f32>) -> f32 {
    values.iter().sum::<f32>() / values.len() as f32
}

pub fn mean_2d_vec(values: &Vec<Vec<f32>>) -> f32 {
    let mut sum = 0.0;
    for vec in values {
        for val in vec {
            sum += val;
        }
    }
    sum / (values.len() * values[0].len()) as f32
}

pub const SOBEL_X: [[f32;3];3] = [
    [1.0, 0.0, -1.0],
    [2.0, 0.0, -2.0],
    [1.0, 0.0, -1.0],
];

pub const SOBEL_Y: [[f32;3];3] = [
    [1.0, 2.0, 1.0],
    [0.0, 0.0, 0.0],
    [-1.0, -2.0, -1.0],
];