use crate::constants;
use leptos::logging::log;
use leptos::{
    logging::{error, warn},
    *,
};
use leptos_use::{use_event_listener, use_window};
use reqwest::Url;
use wasm_bindgen::JsValue;

#[component]
pub fn staging() -> impl IntoView {
    let url = create_local_resource(move || (), |_| get_redirect_url());
    create_effect(move |_| match url.get() {
        Some(Ok(u)) => {
            let window = use_window();
            let window = window.as_ref().unwrap();
            let _new_window = window.open_with_url_and_target(&u, "_blank");

            let _ = use_event_listener(use_window(), ev::message, move |msg| {
                let message = msg.data().as_string();

                if Url::parse(&msg.origin())
                    .map(|u| u.origin() == constants::AUTH_DOMAIN.origin())
                    .unwrap_or_default()
                {
                    match message.as_deref() {
                        Some("Invalid parameters") | Some("Invalid credentials") => {
                            // TODO: send back error message to ssr
                            error!("{}", message.unwrap());
                        }
                        Some(session) => {
                            log!("session received: {}", session.len());
                            let parent = use_window().as_ref().unwrap().parent().unwrap().unwrap();
                            match parent.post_message(
                                &JsValue::from_str(&session),
                                constants::APP_DOMAIN.as_str(),
                            ) {
                                Err(error) => error!(
                                    "post result to app failed: {}",
                                    error.as_string().unwrap_or("".to_owned())
                                ),
                                Ok(_) => log!("session posted"),
                            }
                        }
                        None => {
                            // no action
                        }
                    }
                }
            });
        }
        Some(Err(error)) => warn!("Failed to generate url: {}", error),
        None => {}
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

    let mut url = app_state.auth_domain.join("verify_creds").unwrap();
    url.set_query(Some(
        format!("u={}&e={}", user_identity, expiration).as_str(),
    ));

    Ok(url.as_str().to_owned())
}
