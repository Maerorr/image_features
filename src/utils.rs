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