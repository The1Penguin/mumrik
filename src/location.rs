use monsoon::Moonsoon;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Location {
    pub coord: Params,
    pub name: String,
}


impl Location {
    pub fn new_name(name: String) {
        todo!("Derive the Coord from the name");
    }

    pub fn new_coord(lo: f64, la: f64) {
        todo!("Derive the name from the coord");
    }

    pub fn new(name: String, long: f64, lat: f64) -> Result<Location> {
        let coord = Params::new(lat, lon, None)?;
        Location {
            name,
            coord
        }
    }
}
