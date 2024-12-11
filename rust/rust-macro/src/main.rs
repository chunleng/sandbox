mod macros;

fn main() {
    print_greeting!();
    print_greeting!("Sam");
    print_greeting!("Leo", "John");
}
