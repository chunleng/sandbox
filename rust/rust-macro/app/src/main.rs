use declarative_macros::import_me;
use procedural_macros::{Getter, calc};

mod declarative_macros;

#[derive(Getter)]
struct Person {
    name: String,
    age: usize,
}

fn main() {
    print_greeting!();
    print_greeting!("Sam");
    print_greeting!("Leo", "John");

    // This macros need to be imported to use
    import_me!();

    println!("{}", calc! {1 + (3 + 2)});

    // TODO this case doesn't work!
    // print!("{}", calc! {(1 + 3) + 2});
    let person = Person {
        name: "John".to_string(),
        age: 32,
    };
    println!("person: {}, age: {}", person.get_name(), person.get_age());
}
