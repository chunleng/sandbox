use async_std::task::sleep;
use dioxus::prelude::*;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut cnt = use_signal(|| 0);
    let (mut call, action) = use_action(move || async move {
        sleep(Duration::from_secs(2)).await;
        cnt.with_mut(|x| *x += 1);
    });
    rsx! {
        SuspenseBoundary { fallback: |_| rsx! { "Loading" },
            Request { action, cnt }
        }
        " "
        button { onclick: move |_| call(), "Add" }
    }
}

#[component]
fn Request(action: Resource<()>, cnt: Signal<i32>) -> Element {
    action.suspend()?;

    rsx! { "{cnt} " }
}

fn use_action<F>(
    action_to_perform: impl FnMut() -> F + 'static,
) -> (impl FnMut() -> (), Resource<()>)
where
    F: Future<Output = ()> + 'static,
{
    let (tx, rx) = use_hook(|| {
        let (tx, rx) = mpsc::channel(1);
        (tx, Arc::new(Mutex::new(rx)))
    });
    let mut suspending_resource = use_resource(move || {
        let rx = rx.clone();
        async move {
            rx.lock().unwrap().recv().await;
        }
    });
    let tx_clone = tx.clone();
    use_hook(move || {
        spawn(async move {
            let _ = tx_clone.send(()).await;
        })
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
