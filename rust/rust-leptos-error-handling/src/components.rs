use crate::errors::AppError;
use leptos::{
    component,
    prelude::{Action, Children, Effect, ElementChild, ErrorBoundary, Get, IntoAny, View},
    view, IntoView,
};
use leptos_router::hooks::use_navigate;

#[component]
fn Loading() -> impl IntoView {
    view! { <div>"Loading"</div> }
}

#[component]
fn ShowErrorBoundary(children: Children) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <div>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| e.to_string().into_view())
                            .collect::<Vec<_>>()
                    }}
                </div>
            }
        }>
            <>{children()}</>
        </ErrorBoundary>
    }
}

#[component]
pub fn ShowErrorView() -> impl IntoView {
    let fail = Action::new(|(): &()| async { Err::<View<String>, _>(AppError::Unknown) });

    Effect::new(move || {
        fail.dispatch(());
    });

    view! {
        <p>
            <ShowErrorBoundary>
                <div>
                    "Value: "
                    {move || match fail.value().get() {
                        Some(s) => s.map(|v| { v.into_any() }),
                        None => Ok(view! { <Loading /> }.into_any()),
                    }}
                </div>
            </ShowErrorBoundary>
        </p>
    }
}

#[component]
fn RedirectErrorBoundary(children: Children) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|_| {
            use_navigate()("/error", Default::default());
            view! {}
        }>
            <>{children()}</>
        </ErrorBoundary>
    }
}

#[component]
pub fn RedirectErrorView() -> impl IntoView {
    let fail = Action::new(|(): &()| async { Err::<View<String>, _>(AppError::Unknown) });

    Effect::new(move || {
        fail.dispatch(());
    });

    view! {
        <p>
            <RedirectErrorBoundary>
                <div>
                    "Value: "
                    {move || match fail.value().get() {
                        Some(s) => s.map(|v| { v.into_any() }),
                        None => Ok(view! { <Loading /> }.into_any()),
                    }}
                </div>
            </RedirectErrorBoundary>
        </p>
    }
}
