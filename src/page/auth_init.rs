use crate::constants::WEB_URL;
use leptos::{
    logging::{error, warn},
    *,
};
use leptos_use::{use_event_listener, use_window};
use reqwest::Url;

#[component]
pub fn staging() -> impl IntoView {
    _ = use_event_listener(use_window(), ev::message, move |msg| {
        if Url::parse(&msg.origin())
            .map(|u| u.origin() != WEB_URL.origin())
            .unwrap_or_default()
        {
            warn!("Url mismatch: {}", msg.origin());
            return;
        }
        let message = msg.data().as_string();
        match message.as_deref() {
            Some("login") => {
                let url = create_local_resource(move || (), |_| get_redirect_url());
                create_effect(move |_| match url.get() {
                    Some(Ok(u)) => {
                        let window = use_window();
                        let window = window.as_ref().unwrap();
                        let _ = window.open_with_url_and_target(&u, "_blank");
                    }
                    Some(Err(error)) => warn!("Failed to generate url: {}", error),
                    None => error!("No url generated"),
                });
            }
            Some("Invalid parameters")
            | Some("Invalid credentials")
            | Some("No server response") => {
                // TODO: send back error message to ssr
            }
            _ => {
                // no action
            }
        }
    });

    view! {
    <>
    </>
    }
}

#[server]
pub async fn get_redirect_url() -> Result<String, ServerFnError> {
    use crate::auth::identity::AppState;
    use axum_extra::extract::{cookie::Key, SignedCookieJar};
    use base64::{engine::general_purpose::URL_SAFE, Engine};

    let app_state = use_context::<AppState>().unwrap();
    let jar = leptos_axum::extract_with_state::<SignedCookieJar<Key>, AppState>(&app_state).await?;

    let user_identity = jar
        .get("user_identity")
        .ok_or(ServerFnError::new("Session not found!"))?;
    let expiration = jar
        .get("expiration")
        .ok_or(ServerFnError::new("Session not found!"))?;

    // TODO: encrypt
    let user_identity = URL_SAFE.encode(user_identity.value());
    let expiration = URL_SAFE.encode(expiration.value());

    let url = format!(
        "{}/verify_creds?u={}&e={}",
        app_state.auth_domain.as_str(),
        user_identity,
        expiration
    );

    Ok(url)
}
