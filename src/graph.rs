pub struct Node {
    pub lat: f64, //Breitengrad
    pub long: f64, //Längengrad
}

impl Node {
    pub fn new(lat: f64, long: f64) -> Self {
        Self { lat, long }
    }
}

