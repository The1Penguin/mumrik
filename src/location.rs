use monsoon::Params;
use monsoon::Result;

#[derive(Clone, Debug)]
pub struct Location {
    pub coord: Params,
    pub name: String,
}

impl Location {
    pub fn new_name(name: String) {
        todo!("Derive the Coord from the name");
    }

    pub fn new_coord(lat: f64, long: f64) {
        todo!("Derive the name from the coord");
    }

    pub fn new(name: String, lat: f64, long: f64) -> Result<Location> {
        let coord = Params::new(lat, long, None)?;
        Ok(Location {
            name,
            coord
        })
    }
}
