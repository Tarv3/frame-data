

pub fn min_f32(a: f32, b: f32) -> f32 {
    if a > b {
        b
    }
    else {
        a
    }
}

pub fn max_f32(a: f32, b: f32) -> f32 {
    if a < b {
        b
    }
    else {
        a
    }
}