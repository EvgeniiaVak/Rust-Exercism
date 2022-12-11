const PRODUCTION_RATE: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => PRODUCTION_RATE * (speed as f64),
        5..=8 => PRODUCTION_RATE * (speed as f64) * 0.9,
        9..=10 => PRODUCTION_RATE * (speed as f64) * 0.77,
        _ => panic!("Speed must be between 0 and 10"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
