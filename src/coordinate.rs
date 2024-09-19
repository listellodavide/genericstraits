mod sub;

struct Coordinate<T, U> {
    latitude: T,
    longitude: U,
}

fn test_coordinates() {
    
    let location1 = Coordinate {
        latitude: -27.34556,
        longitude: 74
    };
    
    println!("latitude: {}, longitute: {}", location1.longitude, location1.longitude);
    
}