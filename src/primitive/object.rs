fn main() {
    println!("primitive/object!!!");

    struct Person {
        name: String,
        age: i32,
    }

    let person = Person {
        name: String::from("miso"),
        age: 21,
    };

    println!("name: {}/{}", person.name, person.age);
}
