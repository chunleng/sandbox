// with #[macro_export], all modules in this crate will be able to access this
// macro without importing
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

macro_rules! import_me {
    () => {
        println!("import_me is imported!");
    };
}
pub(crate) use import_me;
