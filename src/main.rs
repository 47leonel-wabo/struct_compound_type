fn main() {
    let aya = Person::new(
        String::from("Aya Mokoko"),
        22 as i8,
        Genre::FEMALE,
        String::from("Manager"),
        Address::new(
            "25 st-green".to_string(),
            String::from("Dokoti"),
            String::from("Douala"),
        ),
    );

    println!("{:#?}", aya);
    println!("{:?} is adult: {}", aya.name, aya.is_adult());
}

#[derive(Debug)]
struct Address {
    street: String,
    town: String,
    city: String,
}

impl Address {
    fn new(street: String, town: String, city: String) -> Address {
        Address { street, town, city }
    }
}

#[derive(Debug)]
enum Genre {
    FEMALE,
    MALE,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i8,
    genre: Genre,
    occupation: String,
    address: Address,
}

impl Person {
    fn new(name: String, age: i8, genre: Genre, occupation: String, address: Address) -> Person {
        Person { name, age, genre, occupation, address }
    }
    fn is_adult(&self) -> bool { self.age >= 18 }
}