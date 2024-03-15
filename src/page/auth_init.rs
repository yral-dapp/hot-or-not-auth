use crate::constants;
use leptos::{
    logging::{error, warn},
    *,
};
use leptos_use::{use_event_listener, use_window};
use reqwest::Url;

#[component]
pub fn staging() -> impl IntoView {
    _ = use_event_listener(use_window(), ev::message, move |msg| {
        let message = msg.data().as_string();
        let url_origin = Url::parse(&msg.origin());
        if url_origin
            .map(|u| u.origin() != constants::APP_DOMAIN.origin())
            .unwrap_or_default()
        {
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
                _ => {
                    // no action
                }
            }
        } else if url_origin
            .map(|u| u.origin() != constants::AUTH_DOMAIN.origin())
            .unwrap_or_default()
        {
            match message.as_deref() {
                Some("Invalid parameters")
                | Some("Invalid credentials")
                | Some("No server response") => {
                    // TODO: send back error message to ssr
                }
                Some(session) => {
                    let window = use_window();
                    let window = window.as_ref().unwrap();
                    let opener = window.opener().unwrap();
                    let opener = Window::from(opener);
                    match opener
                        .post_message(&JsValue::from_str(&session), constants::APP_DOMAIN.as_str())
                    {
                        Err(error) => log!(
                            "post result to app failed: {}",
                            error.as_string().unwrap_or("".to_owned())
                        ),
                        Ok(_) => {}
                    }
                }
                _ => {
                    // no action
                }
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
