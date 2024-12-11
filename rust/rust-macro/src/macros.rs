#[macro_export]
macro_rules! print_greeting {
    ( ) => {
        println!("Hello!");
    };
    ( $x:expr ) => {
        println!("Hello, {}!", $x);
    };
    ( $head:expr , $( $x:expr ),+ ) => {
        println!("Hello, {}!", concat!($head, $(" and ", $x),*));
    };
}
