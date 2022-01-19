pub fn rand_num(from: f64, to: f64) -> f64 {
    (rand::random::<u32>() as f64 / u32::MAX as f64) * (to - from) + from
}