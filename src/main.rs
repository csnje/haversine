use std::env;
use std::error::Error;

// Mean radius of earth
const EARTH_RADIUS: f64 = 6371008.7714;

// Distance (meters) between two points (latitude and longitude in radians)
fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let a = ((lat2 - lat1) / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * ((lon2 - lon1) / 2.0).sin().powi(2);
    //2.0 * EARTH_RADIUS * a.sqrt().atan2((1.0 - a).sqrt())
    2.0 * EARTH_RADIUS * a.sqrt().asin()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the expected four arguments, converting to radians
    let args: Vec<f64> = env::args()
        .skip(1)
        .take(4)
        .map(|i| i.parse::<f64>().unwrap().to_radians())
        .collect();

    println!("{}", haversine(args[0], args[1], args[2], args[3]));

    Ok(())
}
