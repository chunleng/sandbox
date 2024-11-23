use actions::kratos::register::{
    get_registration_flow, register, GetRegisterFlowError, RegisterError,
};
use leptos::{
    component, create_action, create_effect, create_node_ref, create_signal, ev::SubmitEvent,
    html::Input, mount_to_body, view, window, IntoView, NodeRef, SignalGet, SignalSet, View,
};
use leptos_router::{use_navigate, use_query_map, Outlet, Route, Router, Routes};
use ory_client::models::{ui_node_attributes::UiNodeAttributes, RegistrationFlow};

mod actions;
mod config;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=KratosPage>
                            <Route path="" view=LoginForm />
                            <Route path="register" view=RegistrationForm />
                            <Route path="verify" view=VerificationForm />
                            <Route path="err" view=ErrorPage />
                        </Route>
                    </Routes>
                </main>
            </Router>
        }
    })
}

#[component]
fn KratosPage() -> impl IntoView {
    view! {
        <div>
            <div>
                <a href="/">Login</a>
                " "
                <a href="/register">Register</a>
                " "
                <a href="/verify">Verify</a>
            </div>
            <Outlet />
        </div>
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    view! {
        <div>
            <h1>Login</h1>
            <form action="">
                <div>Email " "<input type="text" /></div>
                <div>Password " "<input type="password" /></div>
                <button>Login</button>
            </form>
        </div>
    }
}

#[component]
fn VerificationForm() -> impl IntoView {
    view! {
        <div>
            <h1>Verify</h1>
            <form action="">
                <div>Email " "<input type="text" /></div>
                <button>Send Verification Email</button>
            </form>
            <br />
            <form action="">
                <div>Code " "<input type="text" /></div>
                <button>Verify</button>
            </form>
        </div>
    }
}

#[component]
fn RegistrationForm() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();
    let (flow, set_flow) = create_signal(None::<RegistrationFlow>);
    let get_registration_flow =
        create_action(|flow_id: &String| get_registration_flow(flow_id.clone()));
    let register = create_action(|args: &(String, String, String, String)| {
        let (email, password, flow_id, csrf_token) = args.clone();
        register(
            email.clone(),
            password.clone(),
            flow_id.clone(),
            csrf_token.clone(),
        )
    });
    let email_el: NodeRef<Input> = create_node_ref();
    let password_el: NodeRef<Input> = create_node_ref();

    create_effect(move |_| {
        let query = query.get();
        match query.get("flow") {
            None => {
                if window()
                    .location()
                    .set_href("http://127.0.0.1:4433/self-service/registration/browser")
                    .is_err()
                {
                    navigate("/error", Default::default());
                }
            }
            Some(flow_id) => match get_registration_flow.value().get() {
                Some(Ok(f)) => {
                    set_flow.set(Some(f));
                }
                Some(Err(e)) => match e {
                    GetRegisterFlowError::Gone410 => {
                        navigate("/register", Default::default());
                    }
                    GetRegisterFlowError::Unknown => {
                        navigate("/error", Default::default());
                    }
                },
                _ => {
                    get_registration_flow.dispatch(flow_id.to_string());
                }
            },
        }
    });

    view! {
        {move || match flow.get() {
            None => view! {}.into_view(),
            Some(f) => {
                view! {
                    <div>
                        <h1>Register</h1>
                        <form on:submit=move |e: SubmitEvent| {
                            e.prevent_default();
                            let csrf_token = f
                                .ui
                                .nodes
                                .iter()
                                .flat_map(|x| {
                                    if let UiNodeAttributes::Input(y) = x.attributes.as_ref() {
                                        if y.name == "csrf_token" {
                                            return vec![
                                                y
                                                    .value
                                                    .clone()
                                                    .unwrap()
                                                    .unwrap()
                                                    .as_str()
                                                    .unwrap()
                                                    .to_string(),
                                            ];
                                        }
                                    }
                                    vec![]
                                })
                                .last()
                                .unwrap();
                            register
                                .dispatch((
                                    email_el.get().unwrap().value(),
                                    password_el.get().unwrap().value(),
                                    f.id.to_string(),
                                    csrf_token,
                                ));
                        }>
                            <div>
                                Email " "
                                <input type="text" name="traits.email" node_ref=email_el />
                            </div>
                            <div>
                                Password " "
                                <input type="password" name="password" node_ref=password_el />
                            </div>
                            {match register.value().get() {
                                Some(Ok(_)) => {
                                    view! { <div>Registration done successfully!</div> }.into_view()
                                }
                                Some(Err(RegisterError::ValidationError { messages })) => {
                                    view! {
                                        <div>
                                            <ul>
                                                {messages
                                                    .iter()
                                                    .map(|x| { view! { <li>{x}</li> }.into_view() })
                                                    .collect::<Vec<View>>()
                                                    .into_view()}
                                            </ul>
                                        </div>
                                    }
                                        .into_view()
                                }
                                Some(Err(_)) => view! { <div>Error!</div> }.into_view(),
                                None => view! {}.into_view(),
                            }}
                            <input type="hidden" name="method" value="password" />
                            <button>Register</button>
                        </form>
                    </div>
                }
                    .into_view()
            }
        }}
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    view! { <p>"Error"</p> }
}
