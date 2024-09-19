
enum Fruit {
    Apple,
    Orange,
    Banana,
    Strawberry,
    Kiwi
}

pub fn describe_fruit(fruit: Fruit) {
    match fruit {
        Fruit::Apple => println!("This is apple!"),
        Fruit::Orange => println!("This is orange!"),
        Fruit::Banana => println!("This is banana!"),
        Fruit::Strawberry => println!("This is strawberry!"),
        Fruit::Kiwi => println!("This is kiwi"),
        _ => panic!("Unknown fruit"),       
    }
}

fn main() {
    let apple = Fruit::Apple;
    describe_fruit(apple);
}
