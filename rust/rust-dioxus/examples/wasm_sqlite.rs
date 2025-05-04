use std::{
    fmt::Debug,
    future::Future,
    sync::{Arc, Mutex},
};

use dioxus::prelude::*;
use sqlite::SqliteExecutor;
use tokio::sync::mpsc;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let (value, mut action) = use_sqlite_action(|| SqliteExecutor::new());
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        button { onclick: move |_| { action() }, "Send Event" }
        br {}
        "Returned result: "
        SuspenseBoundary { fallback: |_| rsx! { "Loading" }, {value} }
    }
}

trait SqliteAction {
    type ReturnType;

    fn get(&self) -> Option<Self::ReturnType>;
    async fn execute(&mut self);
}

#[cfg(target_family = "wasm")]
mod sqlite {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::time::Duration;

    use async_std::task::sleep;
    use dioxus::logger::tracing::info;
    use dioxus::prelude::*;
    use serde::Deserialize;
    use wasm_bindgen::prelude::*;
    use web_sys::MessageEvent;
    use web_sys::Worker;
    use web_sys::{WorkerOptions, WorkerType};

    use super::SqliteAction;

    const WORKER_JS: Asset = asset!(
        "/assets/sqlite.js",
        JsAssetOptions::new().with_minify(false) // dioxus minify does not work well with JS module files
    );

    #[derive(Debug)]
    pub struct SqliteExecutor {
        result: Rc<RefCell<Option<String>>>,
        worker: Worker,
        rx: Receiver<()>,
    }

    #[derive(Deserialize)]
    struct Cnt {
        count: String,
    }

    impl SqliteExecutor {
        pub fn new() -> Self {
            let worker_options = WorkerOptions::new();
            worker_options.set_type(WorkerType::Module);
            let worker = Worker::new_with_options(&WORKER_JS.to_string(), &worker_options).unwrap();
            let (tx, rx) = mpsc::channel();

            let self_ = Self {
                result: Rc::new(RefCell::new(None)),
                worker,
                rx,
            };

            let result_binding = self_.result.clone();

            let onmessage: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |event: MessageEvent| {
                    let mut r = result_binding.borrow_mut();
                    *r = Some(
                        serde_wasm_bindgen::from_value::<Cnt>(event.data())
                            .unwrap()
                            .count,
                    );
                    tx.send(()).unwrap();
                    info!("Message received: {:?}", event.data());
                }));
            self_
                .worker
                .set_onmessage(Some(&onmessage.as_ref().unchecked_ref()));

            // This closure ownership now belongs to JS
            onmessage.forget();
            self_
        }
    }
    impl SqliteAction for SqliteExecutor {
        type ReturnType = String;

        fn get(&self) -> Option<Self::ReturnType> {
            self.result.borrow().clone()
        }
        async fn execute(&mut self) {
            self.worker.post_message(&"Init DB".into()).unwrap();

            // Because this is blocking, I use sleep here to make sure that processing is given to
            // another process using try_recv and sleep
            while let Err(_) = self.rx.try_recv() {
                sleep(Duration::from_millis(10)).await;
            }
        }
    }
}

#[cfg(not(target_family = "wasm"))]
mod sqlite {
    use std::cell::RefCell;
    use std::rc::Rc;

    use diesel::dsl::count_star;
    use diesel::prelude::*;
    use dioxus::logger::tracing::info;

    use super::SqliteAction;

    table! {
        dummy (id) {
            id -> Int4,
        }
    }

    #[derive(Clone)]
    pub struct SqliteExecutor {
        conn: Rc<RefCell<SqliteConnection>>,
        result: Option<String>,
    }

    // ref: https://github.com/DioxusLabs/dioxus/discussions/3475#discussioncomment-11713062
    #[cfg(target_os = "android")]
    fn internal_storage_dir() -> String {
        use jni::objects::{JObject, JString};
        use jni::JNIEnv;

        let (tx, rx) = std::sync::mpsc::channel();

        fn run(env: &mut JNIEnv<'_>, activity: &JObject<'_>) -> String {
            let files_dir = env
                .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])
                .unwrap()
                .l()
                .unwrap();
            let files_dir: JString<'_> = env
                .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
                .unwrap()
                .l()
                .unwrap()
                .into();
            let files_dir: String = env.get_string(&files_dir).unwrap().into();
            files_dir
        }

        dioxus::mobile::wry::prelude::dispatch(move |env, activity, _webview| {
            tx.send(run(env, activity)).unwrap()
        });

        rx.recv().unwrap()
    }

    impl SqliteExecutor {
        pub fn new() -> Self {
            #[cfg(not(target_os = "android"))]
            let db_path = "file:/var/tmp/test.db"; // Full path is needed for iOS
            #[cfg(target_os = "android")]
            let db_path = format!("{}/{}", internal_storage_dir(), "test.db");

            Self {
                conn: Rc::new(RefCell::new(SqliteConnection::establish(&db_path).unwrap())),
                result: None,
            }
        }
    }

    impl SqliteAction for SqliteExecutor {
        type ReturnType = String;
        fn get(&self) -> Option<Self::ReturnType> {
            self.result.clone()
        }
        async fn execute(&mut self) {
            let conn = &mut *self.conn.borrow_mut();
            diesel::sql_query("CREATE TABLE IF NOT EXISTS dummy ( id SERIAL PRIMARY KEY)")
                .execute(conn)
                .unwrap();

            diesel::sql_query("INSERT INTO dummy DEFAULT VALUES")
                .execute(conn)
                .unwrap();

            use dummy::dsl::dummy;
            let res: i64 = dummy.select(count_star()).first(conn).unwrap();

            info!("{}", res.to_string());
            self.result = Some(res.to_string());
        }
    }
}

// Suspense does not react when using restart in 0.6.3, as it does not trigger the status in the
// use_resource.
//
// The following PR will resolve this issue
// https://github.com/DioxusLabs/dioxus/issues/2812
fn use_sqlite_action<T, U: Debug>(action: impl FnOnce() -> T) -> (Element, Box<dyn FnMut()>)
where
    // TODO think about how to remove this 'static. There are some problem related to async trait
    // that needs to be resolved but however, I cannot replace Rc with a Sync Arc because of
    // closure.
    T: SqliteAction<ReturnType = U> + 'static,
{
    let sqlite = use_signal(action);
    let (call, action) = use_action(move || {
        let mut sqlite = sqlite;
        async move {
            let mut sqlite = sqlite.write();
            sqlite.execute().await;
        }
    });
    (
        rsx! {
            SqliteValueSuspense { value: format!("{:?}", sqlite.read().get()), fetcher: action }
        },
        Box::new(call),
    )
}

/// Dioxus does not have a use_action so this is temporarily filling in for the lack of function.
///
/// It returns a function that can be executed and a resource to track the state of execution.
fn use_action<F>(
    action_to_perform: impl FnMut() -> F + 'static,
) -> (impl FnMut() -> (), Resource<()>)
where
    F: Future<Output = ()> + 'static,
{
    let (tx, rx) = use_hook(|| {
        let (tx, rx) = mpsc::channel(10);
        (tx, Arc::new(Mutex::new(rx)))
    });
    let mut suspending_resource = use_resource(move || {
        let rx = rx.clone();
        async move {
            rx.lock().unwrap().recv().await;
        }
    });
    let tx2 = tx.clone();
    spawn(async move {
        let _ = tx2.send(()).await;
    });
    let action = Arc::new(Mutex::new(action_to_perform));
    (
        move || {
            let tx = tx.clone();
            let action = action.clone();
            spawn(async move {
                suspending_resource.restart();
                let action_to_perform = action.lock();
                if let Ok(mut action) = action_to_perform {
                    action().await;
                }
                let _ = tx.send(()).await;
            });
        },
        suspending_resource,
    )
}

// Seems like suspense work on components only. I have no idea how to resolve this so I am just
// going to create one for triggering suspense.
#[component]
fn SqliteValueSuspense(value: String, fetcher: Resource<()>) -> Element {
    fetcher.suspend()?;

    rsx! {
        {value}
    }
}
