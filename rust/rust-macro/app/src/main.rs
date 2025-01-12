use declarative_macros::import_me;
use procedural_macros::calc;

mod declarative_macros;

fn main() {
    print_greeting!();
    print_greeting!("Sam");
    print_greeting!("Leo", "John");

    // This macros need to be imported to use
    import_me!();

    print!("{}", calc! {1 + (3 + 2)});

    // TODO this case doesn't work!
    // print!("{}", calc! {(1 + 3) + 2});
}
