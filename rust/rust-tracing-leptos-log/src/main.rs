use leptos::{
    component, create_action, create_signal, mount_to_body, view, IntoView, SignalGetUntracked,
    SignalSet, SignalUpdate,
};
use tracing::{event, Level};
use tracing_subscriber::{prelude::*, registry};

use leptos_logger::LeptosLoggingLayer;

mod leptos_logger;

fn main() {
    registry().with(LeptosLoggingLayer).init();
    mount_to_body(|| HelloWorld)
}

#[component]
fn HelloWorld() -> impl IntoView {
    let (count, set_count) = create_signal(1);
    let (clicked, set_clicked) = create_signal(false);
    let handle_click = create_action(move |&level: &Level| async move {
        set_count.update(|i: &mut i32| *i = *i + 1);
        set_clicked.set(true);

        if level == Level::INFO {
            event! {
                Level::INFO,
                "You have clicked the informational button and you have clicked on the buttons {count} times",
                count=count.get_untracked()
            }
        } else if level == Level::WARN {
            event! {
                Level::WARN,
                "You have clicked the warning button and you have clicked on the buttons {count} times",
                count=count.get_untracked()
            }
        } else if level == Level::ERROR {
            event! {
                Level::ERROR,
                "You have clicked the error button and you have clicked on the buttons {count} times",
                count=count.get_untracked()
            }
        }
    });

    event! {
        Level::TRACE,
        initial_count=count.get_untracked(),
        initial_clicked=clicked.get_untracked()
    }
    view! { <>
        <div><button on:click=move |_| { handle_click.dispatch(Level::INFO); }>"Informational"</button></div>
        <div><button on:click=move |_| { handle_click.dispatch(Level::WARN); }>"Warning"</button></div>
        <div><button on:click=move |_| { handle_click.dispatch(Level::ERROR); }>"Error"</button></div>
    </> }
}
