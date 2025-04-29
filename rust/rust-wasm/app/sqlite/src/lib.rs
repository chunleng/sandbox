use std::{cell::RefCell, rc::Rc};

use diesel::{dsl::count_star, prelude::*};
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker, WorkerOptions, WorkerType, window};

#[wasm_bindgen]
pub fn startup() -> Result<(), JsValue> {
    let worker_options = WorkerOptions::new();
    worker_options.set_type(WorkerType::Module);
    let worker_handle = Rc::new(RefCell::new(Worker::new_with_options(
        "./worker.js",
        &worker_options,
    )?));

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

table! {
    dummy (id) {
        id -> Int4,
    }
}

#[wasm_bindgen(js_name = addAndCount)]
pub async fn add_and_count() -> String {
    // NOTE: sahpool_vfs is also supported, but however, it's pretty new as of writing and many
    // browser has yet added support for it
    // sqlite_wasm_rs::sahpool_vfs::install(None, false)
    //     .await
    //     .unwrap();
    // let mut conn = SqliteConnection::establish("file:post2.db?vfs=opfs-sahpool")
    //     .unwrap_or_else(|_| panic!("Error connecting to post.db"));

    sqlite_wasm_rs::relaxed_idb_vfs::install(None, false)
        .await
        .unwrap();
    let mut conn = SqliteConnection::establish("file:post.db?vfs=relaxed-idb")
        .unwrap_or_else(|_| panic!("Error connecting to post.db"));

    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS dummy (
            id SERIAL PRIMARY KEY
        )",
    )
    .execute(&mut conn)
    .unwrap();

    diesel::sql_query("INSERT INTO dummy DEFAULT VALUES")
        .execute(&mut conn)
        .unwrap();

    use dummy::dsl::dummy;
    let res: i64 = dummy.select(count_star()).first(&mut conn).unwrap();

    res.to_string()
}

fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::new(|message: MessageEvent| {
        let document = window()
            .ok_or::<JsValue>("No window".into())
            .unwrap()
            .document()
            .ok_or::<JsValue>("No document".into())
            .unwrap();
        let el = document.create_element("p").unwrap();
        el.set_inner_html(&format!(
            "Current count: {}",
            message.data().as_string().unwrap()
        ));
        document.body().unwrap().append_with_node_1(&el).unwrap();
    })
}
