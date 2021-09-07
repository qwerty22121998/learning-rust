// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const RATE: u64 = 221;

pub fn success_date(speed: u8) -> f64 {
    match speed {
        1..=4 => 1f64,
        5..=8 => 0.9,
        _ => 0.77
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (RATE * speed as u64) as f64 * success_date(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60f64) as u32
}
