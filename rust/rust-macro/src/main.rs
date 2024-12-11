use macros::import_me;

mod macros;

fn main() {
    print_greeting!();
    print_greeting!("Sam");
    print_greeting!("Leo", "John");

    // This macros need to be imported to use
    import_me!();
}
