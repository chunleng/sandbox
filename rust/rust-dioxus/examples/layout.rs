use dioxus::prelude::*;
use rust_dioxus::page::PageNotFound;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(|| {
        rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            Router::<Route> {}
        }
    });
}

#[derive(Debug, Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        App,
        #[route("/greet/:name")]
        Greet { name: String },
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn MainLayout() -> Element {
    rsx! {
        h1 { "Demo Page" }
        Outlet::<Route> {}
    }
}

#[component]
fn App() -> Element {
    rsx! {
        Link {
            to: Route::Greet {
                name: "John".to_string(),
            },
            "Greet John"
        }
        div { class: "font-bold",
            "Hello"
            br {}
            "World"
        }
    }
}

#[component]
fn Greet(name: String) -> Element {
    let nav = navigator();
    rsx! {
        div { "Hello {name}" }
        button {
            onclick: move |_| {
                nav.push("../");
            },
            "Go Home"
        }
    }
}
