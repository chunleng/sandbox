use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let sqlite = use_sqlite();
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        button {
            onclick: move |_| {
                sqlite.execute();
            },
            "Send Event"
        }
    }
}

trait SqliteHandler {
    fn execute(&self);
}

#[cfg(target_family = "wasm")]
mod sqlite {
    use dioxus::logger::tracing::info;
    use dioxus::prelude::*;
    use wasm_bindgen::prelude::*;
    use web_sys::MessageEvent;
    use web_sys::Worker;
    use web_sys::{WorkerOptions, WorkerType};

    use super::SqliteHandler;

    const WORKER_JS: Asset = asset!(
        "/assets/sqlite.js",
        JsAssetOptions::new().with_minify(false) // dioxus minify does not work well with JS module files
    );

    #[derive(Clone)]
    pub struct SqliteExecutor {
        worker: Worker,
    }

    impl SqliteExecutor {
        pub fn new() -> Self {
            let worker_options = WorkerOptions::new();
            worker_options.set_type(WorkerType::Module);
            let worker = Worker::new_with_options(&WORKER_JS.to_string(), &worker_options).unwrap();

            let onmessage: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |event: MessageEvent| {
                    info!("Message received: {:?}", event.data());
                }));
            worker.set_onmessage(Some(&onmessage.as_ref().unchecked_ref()));

            // This closure ownership now belongs to JS
            onmessage.forget();
            Self { worker }
        }
    }
    impl SqliteHandler for SqliteExecutor {
        fn execute(&self) {
            self.worker.post_message(&"Init DB".into()).unwrap();
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

    use super::SqliteHandler;

    table! {
        dummy (id) {
            id -> Int4,
        }
    }

    #[derive(Clone)]
    pub struct SqliteExecutor {
        conn: Rc<RefCell<SqliteConnection>>,
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
            }
        }
    }

    impl SqliteHandler for SqliteExecutor {
        fn execute(&self) {
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
        }
    }
}

fn use_sqlite() -> impl SqliteHandler {
    use_hook(|| sqlite::SqliteExecutor::new())
}
