use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker, console, window};

#[wasm_bindgen]
pub fn validate(value: JsValue) -> bool {
    value.as_f64().is_some()
}

#[wasm_bindgen]
pub fn startup() -> Result<(), JsValue> {
    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js")?));
    console::log_1(&"Hello from Main".into());

    let document = window()
        .ok_or::<JsValue>("No window".into())?
        .document()
        .ok_or::<JsValue>("No document".into())?;

    // If our `onmessage` callback should stay valid after exiting from the `oninput` closure
    // scope, we need to either forget it (so it is not destroyed) or store it somewhere. To avoid
    // leaking memory every time we want to receive a response from the worker, we move a handle
    // into the closure where we send the message to which we will always attach the last
    // `onmessage` callback. The initial value will not be used and we silence the warning.
    // ref: https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html#srclibrs
    #[allow(unused_assignments)]
    let mut persistent_message_callback = get_on_msg_callback();

    let callback = Closure::<dyn FnMut()>::wrap(Box::new(move || {
        console::log_1(&"calling worker".into());
        let worker = &*worker_handle.borrow();
        worker.post_message(&1.into()).unwrap();
        persistent_message_callback = get_on_msg_callback();
        worker.set_onmessage(Some(persistent_message_callback.as_ref().unchecked_ref()));
    }));
    document.set_onclick(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory as we let Javascript take ownership of it.
    callback.forget();

    Ok(())
}

fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::new(|message: MessageEvent| {
        console::log_2(&"Message received: ".into(), &message.data().into());
    })
}
