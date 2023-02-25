const BASE_LINE: f64 = 221f64;
pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        1..=4 => (speed as f64 * BASE_LINE) as f64,
        5..=8 => (speed as f64 * (BASE_LINE * 90.0 / 100.0)) as f64,
        9..=10 => (speed as f64 * (BASE_LINE * 77.0 / 100.0)) as f64,
        _ => 0.0, // handle invalid speed values
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

fn main() {
    let ans = working_items_per_minute(5);
    println!("{:?}", ans);
}
