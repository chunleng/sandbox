use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, console, window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(err: &str);

    #[wasm_bindgen]
    fn alert(msg: &str);
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    console::log_1(&"This log is using web_sys library".into());
    error("This error is printed using wasm_bindgen library");
    alert("wasm_bindgen error is showing~");

    // Use web_sys to show an element
    let document = window().unwrap().document().unwrap();
    let e: HtmlElement = document.create_element("div")?.dyn_into()?;
    e.set_inner_text("Hello World from Wasm!");
    document.body().unwrap().append_child(&e).unwrap();

    Ok(())
}

#[wasm_bindgen]
pub fn rust_sum(a: i32, b: i32) -> i32 {
    a + b
}
