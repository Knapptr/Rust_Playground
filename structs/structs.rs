#[allow(dead_code)]

fn main() {
    let person1 = Person::default();
    person1.print_age();
}

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn print_age(&self) {
        println!("{}", self.age);
    }

    fn default() -> Person {
        Person {
            name: "Tyler".to_string(),
            age: 34,
        }
    }
}
