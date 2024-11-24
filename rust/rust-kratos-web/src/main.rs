use actions::kratos::register::{
    get_registration_flow, register, GetRegisterFlowError, RegisterError,
};
use actions::kratos::verify::{get_verify_flow, verify, GetVerifyFlowError, VerifyError};
use config::API_BASE_URL;
use leptos::{
    component, create_action, create_effect, create_node_ref, create_signal, ev::SubmitEvent,
    html::Input, mount_to_body, view, window, IntoView, NodeRef, SignalGet, SignalSet, View,
};
use leptos_router::{use_navigate, use_query_map, Outlet, Route, Router, Routes};
use ory_client::models::{
    ui_node_attributes::UiNodeAttributes, ui_text::TypeEnum, RegistrationFlow, VerificationFlow,
};

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
                            <Route path="resend" view=ResendVerificationForm />
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
                <a href="/verify">Resend Verification</a>
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
fn ResendVerificationForm() -> impl IntoView {
    let query = use_query_map();
    let (flow, set_flow) = create_signal(None::<VerificationFlow>);
    let (message, set_message) = create_signal(None::<String>);
    let email_el: NodeRef<Input> = create_node_ref();
    let get_verification_flow = create_action(|flow_id: &String| get_verify_flow(flow_id.clone()));
    let resend = create_action(|args: &(String, VerificationFlow)| {
        let (email, flow) = args.clone();
        verify(Some(email.clone()), None, flow)
    });

    create_effect(move |_| {
        let query = query.get();
        match query.get("flow") {
            None => {
                if window()
                    .location()
                    .set_href(&format!(
                        "{}/self-service/verification/browser",
                        *API_BASE_URL
                    ))
                    .is_err()
                {
                    use_navigate()("/error", Default::default());
                }
                use_navigate()("/resend", Default::default());
                return;
            }
            Some(flow_id) => {
                get_verification_flow.dispatch(flow_id.to_string());
            }
        }
    });

    create_effect(move |_| {
        if let Some(x) = get_verification_flow.value().get() {
            match x {
                Ok(x) => {
                    set_flow.set(Some(x));
                }
                Err(x) => match x {
                    GetVerifyFlowError::Gone410 => {
                        use_navigate()("/resend", Default::default());
                    }
                    GetVerifyFlowError::Unknown => {
                        use_navigate()("/error", Default::default());
                    }
                },
            }
        }
    });

    create_effect(move |_| {
        if let Some(f) = flow.get() {
            set_message.set(None);
            if f.state == Some("passed_challenge".into()) {
                use_navigate()("/", Default::default());
                return;
            } else if f.state == Some("sent_email".into()) {
                use_navigate()(&format!("/verify?flow={}", f.id), Default::default());
                return;
            }

            if let Some(e) = f.ui.messages.as_ref() {
                if let Some(e) = e.last() {
                    if e.r#type == TypeEnum::Error {
                        set_message.set(Some(e.text.to_string()));
                        return;
                    }
                }
            }
        }
    });

    create_effect(move |_| {
        if let Some(x) = resend.value().get() {
            match x {
                Ok(_) => match flow.get() {
                    Some(f) => {
                        resend.value().set(None);
                        get_verification_flow.dispatch(f.id);
                    }
                    None => {
                        use_navigate()("/error", Default::default());
                    }
                },
                Err(e) => match e {
                    VerifyError::Gone410 => {
                        use_navigate()("/resend", Default::default());
                    }
                    VerifyError::BadRequest400 => match flow.get() {
                        Some(f) => {
                            resend.value().set(None);
                            get_verification_flow.dispatch(f.id);
                        }
                        None => {
                            use_navigate()("/error", Default::default());
                        }
                    },
                    VerifyError::Unknown => {
                        use_navigate()("/error", Default::default());
                    }
                },
            }
        }
    });

    view! {
        {move || match flow.get() {
            None => view! {}.into_view(),
            Some(f) => {
                view! {
                    <>
                        <h1>Resend Verification Email</h1>
                        <form on:submit=move |e: SubmitEvent| {
                            e.prevent_default();
                            resend.dispatch((email_el.get().unwrap().value(), f.clone()));
                        }>
                            <div>
                                "Email "<input type="text" node_ref=email_el />" "
                                <button>Send</button>
                            </div>
                        </form>
                        {match message.get() {
                            Some(x) => view! { <div>"Message: " {x}</div> }.into_view(),
                            None => view! {}.into_view(),
                        }}
                    </>
                }
                    .into_view()
            }
        }}
    }
}

#[component]
fn VerificationForm() -> impl IntoView {
    let query = use_query_map();
    let code_el: NodeRef<Input> = create_node_ref();
    let (flow, set_flow) = create_signal(None::<VerificationFlow>);
    let (message, set_message) = create_signal(None::<String>);
    let get_verification_flow = create_action(|flow_id: &String| get_verify_flow(flow_id.clone()));
    let verify = create_action(|args: &(String, VerificationFlow)| {
        let (code, flow) = args.clone();
        verify(None, Some(code.clone()), flow)
    });

    create_effect(move |_| {
        let query = query.get();
        match query.get("flow") {
            None => {
                use_navigate()("/resend", Default::default());
                return;
            }
            Some(flow_id) => {
                get_verification_flow.dispatch(flow_id.to_string());
            }
        }
    });

    create_effect(move |_| {
        if let Some(x) = get_verification_flow.value().get() {
            match x {
                Ok(x) => {
                    set_flow.set(Some(x));
                }
                Err(x) => match x {
                    GetVerifyFlowError::Gone410 => {
                        use_navigate()("/resend", Default::default());
                    }
                    GetVerifyFlowError::Unknown => {
                        use_navigate()("/error", Default::default());
                    }
                },
            }
        }
    });

    create_effect(move |_| {
        if let Some(f) = flow.get() {
            set_message.set(None);
            if f.state == Some("passed_challenge".into()) {
                use_navigate()("/", Default::default());
                return;
            } else if f.state == Some("choose_method".into()) {
                use_navigate()(&format!("/resend?flow={}", f.id), Default::default());
                return;
            }

            let last_message = f.ui.messages.as_ref().unwrap().last();
            if let Some(e) = last_message {
                if e.r#type == TypeEnum::Error {
                    set_message.set(Some(e.text.to_string()));
                    return;
                }
            }

            let code =
                f.ui.nodes
                    .iter()
                    .filter_map(|x| match x.attributes.as_ref() {
                        UiNodeAttributes::Input(x) => {
                            if x.name == "code" && x.value.is_some() {
                                return Some(
                                    x.value
                                        .clone()
                                        .unwrap()
                                        .unwrap()
                                        .as_str()
                                        .unwrap()
                                        .to_string(),
                                );
                            }
                            None
                        }
                        _ => None,
                    })
                    .last();
            if let Some(c) = code {
                verify.dispatch((c, f));
            }
        }
    });

    create_effect(move |_| {
        if let Some(x) = verify.value().get() {
            match x {
                Ok(_) => match flow.get() {
                    Some(f) => {
                        verify.value().set(None);
                        get_verification_flow.dispatch(f.id);
                    }
                    None => {
                        use_navigate()("/error", Default::default());
                    }
                },
                Err(e) => match e {
                    VerifyError::Gone410 => {
                        use_navigate()("/resend", Default::default());
                    }
                    VerifyError::BadRequest400 => match flow.get() {
                        Some(f) => {
                            verify.value().set(None);
                            get_verification_flow.dispatch(f.id);
                        }
                        None => {
                            use_navigate()("/error", Default::default());
                        }
                    },
                    VerifyError::Unknown => {
                        use_navigate()("/error", Default::default());
                    }
                },
            }
        }
    });

    view! {
        <div>
            <h1>Verify</h1>
            <form on:submit=move |e: SubmitEvent| {
                e.prevent_default();
                let f = flow.get().unwrap();
                verify.dispatch((code_el.get().unwrap().value(), f));
            }>
                <div>Code " "<input type="text" node_ref=code_el />" "<button>Verify</button></div>
                <div>
                    If you are unsure of the code, click " "<a href="/resend">here</a>" "
                    to request for another email
                </div>
            </form>
            {move || match message.get() {
                Some(x) => view! { <div>"Message: " {x}</div> }.into_view(),
                None => view! {}.into_view(),
            }}
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
