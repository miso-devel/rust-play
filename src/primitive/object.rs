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

    impl Person {
        fn change_name(&self) -> String {
            return self.name.clone() + "さん";
        }
        fn change_age(&self) -> i32 {
            return self.age + 10;
        }
    }

    println!("{}", person.change_name());
    println!("{}", person.change_age());
}
