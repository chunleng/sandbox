use declarative_macros::import_me;
mod declarative_macros;

fn main() {
    print_greeting!();
    print_greeting!("Sam");
    print_greeting!("Leo", "John");

    // This macros need to be imported to use
    import_me!();
}
