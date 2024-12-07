use actions::kratos::login::{get_login_flow, login, GetLoginFlowError, LoginError};
use actions::kratos::logout::{logout, LogoutError};
use actions::kratos::register::{
    get_registration_flow, register, GetRegisterFlowError, RegisterError,
};
use actions::kratos::session::{whoami, WhoAmIError};
use actions::kratos::verify::{get_verify_flow, verify, GetVerifyFlowError, VerifyError};
use config::API_BASE_URL;
use leptos::{
    component, create_action, create_effect, create_node_ref, create_signal, ev::SubmitEvent,
    html::Input, mount_to_body, view, window, IntoView, NodeRef, SignalGet, SignalSet, View,
};
use leptos_router::{use_navigate, use_query_map, Outlet, Route, Router, Routes};
use ory_client::models::LoginFlow;
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
                            <Route path="user" view=UserPage />
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
    let logout = create_action(|(): &()| logout());
    create_effect(move |_| match logout.value().get() {
        Some(Ok(_)) => {}
        Some(Err(LogoutError::Unknown)) => {
            use_navigate()("/error", Default::default());
        }
        None => {}
    });
    view! {
        <div>
            <div>
                <a href="/">Login</a>
                " "
                <a href="/register">Register</a>
                " "
                <a href="/verify">Resend Verification</a>
                " "
                <a href="/user">User</a>
                " "
                <button on:click=move |_| {
                    logout.dispatch(());
                }>Logout</button>
            </div>
            <Outlet />
        </div>
    }
}

#[component]
fn LoginForm() -> impl IntoView {
    let query = use_query_map();
    let email_el: NodeRef<Input> = create_node_ref();
    let password_el: NodeRef<Input> = create_node_ref();
    let (flow, set_flow) = create_signal(None::<LoginFlow>);
    let (message, set_message) = create_signal(None::<View>);
    let get_login_flow = create_action(|flow_id: &String| get_login_flow(flow_id.clone()));
    let login = create_action(|args: &(String, String, LoginFlow)| {
        let (email, password, f) = args.clone();
        login(email.clone(), password.clone(), f)
    });

    create_effect(move |_| match query.get().get("flow") {
        None => {
            if window()
                .location()
                .set_href(&format!("{}/self-service/login/browser", *API_BASE_URL))
                .is_err()
            {
                use_navigate()("/error", Default::default());
            }
        }
        Some(flow_id) => {
            get_login_flow.dispatch(flow_id.to_string());
        }
    });

    create_effect(move |_| {
        if let Some(x) = get_login_flow.value().get() {
            match x {
                Ok(f) => set_flow.set(Some(f)),
                Err(x) => match x {
                    GetLoginFlowError::Gone410 => {
                        use_navigate()("/", Default::default());
                    }
                    GetLoginFlowError::Unknown => {
                        use_navigate()("/error", Default::default());
                    }
                },
            }
        }
    });

    create_effect(move |_| {
        if let Some(f) = flow.get() {
            set_message.set(None);
            let mut messages: Vec<String> =
                f.ui.nodes
                    .iter()
                    .flat_map(|x| x.messages.iter().map(|x| x.text.clone()))
                    .collect();
            if messages.len() == 0 {
                messages =
                    f.ui.messages
                        .iter()
                        .flat_map(|x| x.iter().map(|x| x.text.clone()))
                        .collect();
            }
            set_message.set(Some(
                view! {
                    <ul>
                        {messages
                            .iter()
                            .map(|x| { view! { <li>{x}</li> }.into_view() })
                            .collect::<View>()}
                    </ul>
                }
                .into_view(),
            ));
        }
    });

    create_effect(move |_| {
        if let Some(x) = login.value().get() {
            match x {
                Ok(_) => {
                    use_navigate()("/user", Default::default());
                }
                Err(e) => match e {
                    LoginError::Gone410 => {
                        use_navigate()("/login", Default::default());
                    }
                    LoginError::BadRequest400 => match flow.get() {
                        Some(f) => {
                            login.value().set(None);
                            get_login_flow.dispatch(f.id);
                        }
                        None => {
                            use_navigate()("/error", Default::default());
                        }
                    },
                    LoginError::Unknown => {
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
                    <div>
                        <h1>Login</h1>
                        <form on:submit=move |e: SubmitEvent| {
                            e.prevent_default();
                            login
                                .dispatch((
                                    email_el.get().unwrap().value(),
                                    password_el.get().unwrap().value(),
                                    f.clone(),
                                ));
                        }>
                            <div>Email " "<input type="text" node_ref=email_el /></div>
                            <div>Password " "<input type="password" node_ref=password_el /></div>
                            <button>Login</button>
                            {move || message.get()}
                        </form>
                    </div>
                }
                    .into_view()
            }
        }}
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
    let (flow, set_flow) = create_signal(None::<RegistrationFlow>);
    let (message, set_message) = create_signal(None::<View>);
    let get_registration_flow =
        create_action(|flow_id: &String| get_registration_flow(flow_id.clone()));
    let register = create_action(|args: &(String, String, RegistrationFlow)| {
        let (email, password, f) = args.clone();
        register(email.clone(), password.clone(), f)
    });
    let email_el: NodeRef<Input> = create_node_ref();
    let password_el: NodeRef<Input> = create_node_ref();

    create_effect(move |_| match query.get().get("flow") {
        None => {
            if window()
                .location()
                .set_href(&format!(
                    "{}/self-service/registration/browser",
                    *API_BASE_URL
                ))
                .is_err()
            {
                use_navigate()("/error", Default::default());
            }
        }
        Some(flow_id) => {
            get_registration_flow.dispatch(flow_id.to_string());
        }
    });

    create_effect(move |_| {
        if let Some(x) = get_registration_flow.value().get() {
            match x {
                Ok(f) => set_flow.set(Some(f)),
                Err(x) => match x {
                    GetRegisterFlowError::Gone410 => {
                        use_navigate()("/resend", Default::default());
                    }
                    GetRegisterFlowError::Unknown => {
                        use_navigate()("/error", Default::default());
                    }
                },
            }
        }
    });

    create_effect(move |_| {
        if let Some(f) = flow.get() {
            set_message.set(None);
            let mut messages: Vec<String> =
                f.ui.nodes
                    .iter()
                    .flat_map(|x| x.messages.iter().map(|x| x.text.clone()))
                    .collect();
            if messages.len() == 0 {
                messages =
                    f.ui.messages
                        .iter()
                        .flat_map(|x| x.iter().map(|x| x.text.clone()))
                        .collect();
            }
            set_message.set(Some(
                view! {
                    <ul>
                        {messages
                            .iter()
                            .map(|x| { view! { <li>{x}</li> }.into_view() })
                            .collect::<View>()}
                    </ul>
                }
                .into_view(),
            ));
        }
    });

    create_effect(move |_| {
        if let Some(x) = register.value().get() {
            match x {
                Ok(_) => {
                    set_message.set(Some(
                        view! {
                            <div>
                                "Registration Successful, Click "<a href="/register">here</a>
                                " to register another user"
                            </div>
                        }
                        .into_view(),
                    ));
                    register.value().set(None);
                }
                Err(e) => match e {
                    RegisterError::Gone410 => {
                        use_navigate()("/register", Default::default());
                    }
                    RegisterError::BadRequest400 => match flow.get() {
                        Some(f) => {
                            register.value().set(None);
                            get_registration_flow.dispatch(f.id);
                        }
                        None => {
                            use_navigate()("/error", Default::default());
                        }
                    },
                    RegisterError::Unknown => {
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
                    <div>
                        <h1>Register</h1>
                        <form on:submit=move |e: SubmitEvent| {
                            e.prevent_default();
                            register
                                .dispatch((
                                    email_el.get().unwrap().value(),
                                    password_el.get().unwrap().value(),
                                    f.clone(),
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
                            <button>Register</button>
                        </form>
                        {message}
                    </div>
                }
                    .into_view()
            }
        }}
    }
}

#[component]
fn UserPage() -> impl IntoView {
    let load_user_info = create_action(|(): &()| whoami());
    create_effect(move |_| {
        load_user_info.dispatch(());
    });
    view! {
        <p>
            {move || match load_user_info.value().get() {
                Some(x) => {
                    match x {
                        Ok(x) => {
                            view! {
                                <>
                                    <h1>"User Information"</h1>
                                    <div>{x}</div>
                                </>
                            }
                                .into_view()
                        }
                        Err(WhoAmIError::Unauthorized401) => {
                            use_navigate()("/", Default::default());
                            view! {}.into_view()
                        }
                        Err(WhoAmIError::Unknown) => {
                            use_navigate()("/error", Default::default());
                            view! {}.into_view()
                        }
                    }
                        .into_view()
                }
                None => view! {}.into_view(),
            }}
        </p>
    }
}

#[component]
fn ErrorPage() -> impl IntoView {
    view! { <p>"Error"</p> }
}
