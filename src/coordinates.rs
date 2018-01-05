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
            "<https://www.google.co.uk/maps/place/{}%2C{}>",
            self.latitude,
            self.longitude
        )
    }

    pub fn distance_in_km(&self, other: Coordinates) -> f64 {
        const RADIUS_OF_EARTH_IN_KM: f64 = 6371.0;

        let delta_lat = (self.latitude.to_radians() - other.latitude.to_radians()).abs();
        let delta_long = (self.longitude.to_radians() - other.longitude.to_radians()).abs();

        let a = haversine(delta_lat) +
            self.latitude.to_radians().cos() * other.latitude.to_radians().cos() *
                haversine(delta_long);

        2.0 * RADIUS_OF_EARTH_IN_KM * a.sqrt().asin()
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude)
    }
}

fn haversine(angle_in_radians: f64) -> f64 {
    (1.0 - angle_in_radians.cos()) / 2.0
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

    #[test]
    fn distance_between_two_points_on_earth_should_be_calculated_correctly() {
        let louvre = Coordinates::new(48.861022222222, 2.335825);
        let machu_picchu = Coordinates::new(-13.163333, -72.545556);

        let distance = louvre.distance_in_km(machu_picchu);

        assert_eq!(10036.0, distance.round());
    }
}
