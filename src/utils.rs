pub fn normalize_value(value: f32, min: f32, max: f32) -> f32 {
    (value - min) / (max - min)
}