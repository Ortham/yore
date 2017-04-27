//#![deny(warnings)]

#[derive(Debug)]
pub struct Coordinate {
    // Positive latitude values are at the equator or north of it, as per ISO 6709.
    pub latitude: f64,
    // Positive longitude values are at the prime meridian or east of it, as per ISO 6709.
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Coordinate {
        Coordinate {
            latitude: latitude,
            longitude: longitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts;

    #[test]
    fn new_should_store_given_values() {
        let coordinate = Coordinate::new(consts::E, consts::PI);

        assert_eq!(consts::E, coordinate.latitude);
        assert_eq!(consts::PI, coordinate.longitude);
    }
}
