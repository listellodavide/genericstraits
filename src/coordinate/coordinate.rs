mod coordinate;

pub struct Coordinate<T> {
    pub latitude: T,
    pub longitude: T,
}

impl<T: std::fmt::Display> Coordinate<T> {
    pub fn print(&self) {
        println!("latitude: {}, longitude: {}", self.latitude, self.longitude);
    }
}

pub fn test_coordinates() {
    
    let location1 = Coordinate {
        latitude: 27.34556,
        longitude: 74.34
    };
    
    println!("latitude: {}, longitute: {}", location1.latitude, location1.longitude);

    let location2 = Coordinate {
        latitude: 7.47412,
        longitude: 19.181921
    };

    println!("latitude: {}, longitute: {}", location1.latitude, location1.longitude);

    let location3 = Coordinate {
        latitude: 747412433,
        longitude: 1918192
    };

    println!("latitude: {}, longitute: {}", location1.latitude, location1.longitude);

}