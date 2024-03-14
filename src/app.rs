use crate::{
    error_template::{AppError, ErrorTemplate},
    page::{anonymous, auth_init, login, verify_creds},
    providers::*,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/hot-or-not-auth.css"/>

        // sets the document title
        <Title text="Auth layer for internet"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=login::LandingPage/>
                    <Route path="/anonymous_identity" view=anonymous::AnonymousIdentity/>
                    <Route path="/auth_init" view=auth_init::Staging/>
                    <Route path="/verify_creds" view=verify_creds::VerifyCreds/>
                    <Route path="/google_login" view=google::Login/>
                    <Route path="/google_oauth2_response" view=google::OAuth2Response/>
                    <Route path="/internetcomputer" view=internetcomputer::Login/>
                </Routes>
            </main>
        </Router>
    }
}
