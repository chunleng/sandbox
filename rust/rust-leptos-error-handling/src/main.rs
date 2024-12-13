use components::{RedirectErrorView, ShowErrorView};
use leptos::prelude::ElementChild;
use leptos::{mount::mount_to_body, view};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod errors;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <main>
                    <Routes fallback=|| { "Not found." }>
                        <Route path=path!("show") view=ShowErrorView />
                        <Route path=path!("redirect") view=RedirectErrorView />
                        <Route path=path!("error") view=|| view! { "Error" } />
                    </Routes>
                </main>
            </Router>
        }
    })
}
