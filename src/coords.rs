use std::fmt::{Display, Formatter};

const EARTH_RADIUS: f64 = 6371000.785f64;

#[derive(Clone, Copy, PartialEq)]
pub struct Coords {
    pub lat: f64,
    pub lon: f64
}

impl Display for Coords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.lat, self.lon)
    }
}

impl Coords {
    pub fn deg(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    pub fn rad(lat: f64, lon: f64) -> Self {
        Self::deg(lat.to_degrees(), lon.to_degrees())
    }

    pub fn set_lat_rad(&mut self, lat: f64) {
        self.set_lat_deg(lat.to_degrees());
    }

    pub fn set_lon_rad(&mut self, lon: f64) {
        self.set_lon_deg(lon.to_degrees());
    }

    pub fn set_lat_deg(&mut self, lat: f64) {
        self.lat = lat;
    }

    pub fn set_lon_deg(&mut self, lon: f64) {
        self.lon = lon;
    }

    pub fn to_radians(&self) -> Self {
        Self { lat: self.lat.to_radians(), lon: self.lon.to_radians() }
    }

    pub fn distance_to(&self, other: &Coords) -> f64 {
        let coords1r = self.to_radians();
        let coords2r = other.to_radians();

        let zeta = f64::acos(
            f64::sin(coords1r.lat) * f64::sin(coords2r.lat) +
                f64::cos(coords1r.lat) * f64::cos(coords2r.lat) * f64::cos(coords2r.lon - coords1r.lon));

        zeta * EARTH_RADIUS
    }

    pub fn euclidean_distance_to(&self, other: &Coords) -> f64 {
        let lat_dif = other.lat - self.lat;
        let lon_dif = other.lon - self.lon;
        f64::sqrt(lat_dif * lat_dif + lon_dif * lon_dif)
    }

    pub fn default() -> Self {
        Self { lat: 0f64, lon: 0f64 }
    }
}
