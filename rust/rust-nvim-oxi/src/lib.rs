mod calc;
use nvim_oxi::{Dictionary, Object, print};

#[nvim_oxi::plugin]
fn sample() -> Dictionary {
    print!("sample function called!");
    Dictionary::from_iter([("calc", Object::from(calc::module()))])
}


