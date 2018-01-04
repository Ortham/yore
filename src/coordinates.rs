//#![deny(warnings)]

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Coordinates {
    // Positive latitude values are at the equator or north of it, as per ISO 6709.
    latitude: f64,
    // Positive longitude values are at the prime meridian or east of it, as per ISO 6709.
    longitude: f64,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Coordinates {
        Coordinates {
            latitude: latitude,
            longitude: longitude,
        }
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn map_url(&self) -> String {
        format!(
            "<https://www.google.co.uk/maps/place/{},{}>",
            self.latitude,
            self.longitude
        )
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64::consts;

    #[test]
    fn new_should_store_given_values() {
        let coordinates = Coordinates::new(consts::E, consts::PI);

        assert_eq!(consts::E, coordinates.latitude());
        assert_eq!(consts::PI, coordinates.longitude());
    }
}
