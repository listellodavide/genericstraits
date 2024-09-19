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

impl Coordinate<f64> {
    // calculate distance as a spherical straight line between two coordinates
    // https://en.wikipedia.org/wiki/Earth_radius
    pub fn distance_in_miles(&self, other:&Coordinate<f64>) -> f64 {
        let lat1= self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let lon1= self.longitude.to_radians();
        let lon2 = other.longitude.to_radians();

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;
        let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        return 3958.0 * c ; // radius_earth in miles * arc_distance in miles
    }
    
    pub fn distance_in_km(&self, other:&Coordinate<f64>) -> f64 {
        return self.distance_in_miles(other) * 1.6093448; // radius_earth in miles * arc_distance in miles * miles to km conversion
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