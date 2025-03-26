#[derive(Clone, Default, Debug, PartialEq)]
pub struct Coord {
    pub long: f64,
    pub lat: f64,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Location {
    pub coord: Coord,
    pub name: String,
}

impl Coord {
    pub fn new(long: f64, lat:f64) -> Self {
        Coord {
            long,
            lat
        }
    }
}

impl Location {
    pub fn new_name(name: String) {
        todo!("Derive the Coord from the name");
    }

    pub fn new_coord(lo: f64, la: f64) {
        todo!("Derive the name from the coord");
    }

    pub fn new(name: String, long: f64, lat: f64) -> Location {
        Location {
            name,
            coord: Coord::new(long,lat)
        }
    }
}
