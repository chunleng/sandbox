use std::sync::mpsc;

use leptos::{ev::MouseEvent, prelude::*};

fn main() {
    mount_to_body(|| {
        view! { <ShowClickLog /> }
    })
}

#[component]
fn ShowClickLog() -> impl IntoView {
    let trigger = ArcTrigger::new();
    let (tx, rx) = mpsc::channel();

    view! {
        <button on:click={
            let trigger = trigger.clone();
            let tx = tx.clone();
            move |_: MouseEvent| {
                tx.send("A").unwrap();
                trigger.notify();
            }
        }>"Show Message A"</button>
        <button on:click={
            let trigger = trigger.clone();
            let tx = tx.clone();
            move |_: MouseEvent| {
                tx.send("B").unwrap();
                trigger.notify();
            }
        }>"Show Message B"</button>
        {move || {
            trigger.track();
            rx.try_recv().unwrap_or_else(|_| view! { "Click on any button" })
        }}
    }
}
