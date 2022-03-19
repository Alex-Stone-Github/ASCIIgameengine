use std::{thread, time};

pub fn lerp(x1: f32, x2: f32, t: f32) -> f32 {
    x1 + (x2 - x1) * t
}
pub fn sleep(ms: i32) {
    let duration = time::Duration::from_millis(ms as u64);
    thread::sleep(duration);
}
