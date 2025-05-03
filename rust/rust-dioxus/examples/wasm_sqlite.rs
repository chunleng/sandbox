use dioxus::prelude::*;
use sqlite::SqliteExecutor;

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
        {value.read().clone().unwrap_or("None".to_string())}
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

fn use_sqlite_action<T, U>(action: impl FnOnce() -> T) -> (Signal<Option<U>>, Box<dyn FnMut()>)
where
    // TODO think about how to remove this 'static. There are some problem related to async trait
    // that needs to be resolved but however, I cannot replace Rc with a Sync Arc because of
    // closure.
    T: SqliteAction<ReturnType = U> + 'static,
{
    let mut value = use_signal(|| None::<U>);
    let mut sqlite = use_signal(action);
    (
        value,
        Box::new(move || {
            spawn(async move {
                let mut sqlite = sqlite.write();
                sqlite.execute().await;
                value.set(sqlite.get());
            });
        }),
    )
}
