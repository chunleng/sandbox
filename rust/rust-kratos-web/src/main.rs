use leptos::{component, mount_to_body, view, IntoView};
use leptos_router::{Outlet, Route, Router, Routes};

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
    view! {
        <div>
            <h1>Register</h1>
            <form action="">
                <div>Email " " <input type="text" /></div>
                <div>Password " " <input type="password" /></div>
                <button>Register</button>
            </form>
        </div>
    }
}
