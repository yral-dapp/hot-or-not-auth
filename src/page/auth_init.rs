use crate::constants;
use leptos::{
    logging::{error, log, warn},
    *,
};
use leptos_use::{use_event_listener, use_window};

/// When user wants to login this opens in iframe
#[component]
pub fn staging() -> impl IntoView {
    use reqwest::Url;
    use wasm_bindgen::JsValue;

    let url = create_local_resource(move || (), |_| get_redirect_url());
    create_effect(move |_| match url.get() {
        Some(Ok(u)) => {
            let window = use_window();
            let window = window.as_ref().unwrap();
            let _new_window = window.open_with_url_and_target(&u, "yral_auth");

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
                            let message = session.to_owned();
                            let session = session.to_owned();

                            let resource = create_local_resource(
                                move || session.clone(),
                                |session| update_session(session),
                            );
                            create_effect(move |_| match resource.get() {
                                Some(Ok(())) => {}
                                Some(Err(_)) => {}
                                None => {}
                            });

                            let parent = use_window().as_ref().unwrap().parent().unwrap().unwrap();
                            match parent.post_message(
                                &JsValue::from_str(&message),
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
                } else {
                    log!("ignored from: {:?}", msg.origin());
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

#[server]
pub async fn update_session(session: String) -> Result<(), ServerFnError> {
    use crate::auth::{agent_js, cookie, identity::AppState};
    use axum::{http::header, response::IntoResponse};
    use axum_extra::extract::{
        cookie::{Key, SameSite},
        SignedCookieJar,
    };
    use chrono::{Duration, Utc};
    use leptos_axum::ResponseOptions;

    let session = serde_json::from_str::<agent_js::SessionResponse>(&session)
        .map_err(|e| ServerFnError::new(format!("{:?}", e)))?;
    let app_state =
        use_context::<AppState>().ok_or_else(|| ServerFnError::new("Context not found!"))?;
    let mut signed_jar: SignedCookieJar =
        leptos_axum::extract_with_state::<SignedCookieJar<Key>, AppState>(&app_state).await?;

    let cookie_domain = app_state.cookie_domain.host_str().unwrap().to_owned();

    let user_cookie = cookie::create_cookie(
        "user_identity",
        session.user_identity.to_owned(),
        cookie_domain.to_owned(),
        SameSite::None,
    )
    .await;
    signed_jar = signed_jar.add(user_cookie);

    let expiration = match session.delegation_identity._delegation.delegations.get(0) {
        Some(signed_delegation) => signed_delegation.delegation.expiration.to_owned(),
        None => {
            let expiration = Utc::now() + Duration::days(30);
            expiration
                .timestamp_nanos_opt()
                .unwrap()
                .unsigned_abs()
                .to_string()
        }
    };

    let exp_cookie =
        cookie::create_cookie("expiration", expiration, cookie_domain, SameSite::None).await;
    signed_jar = signed_jar.add(exp_cookie);

    let signed_jar_into_response = signed_jar.into_response();
    let response_options = expect_context::<ResponseOptions>();
    for header_value in signed_jar_into_response
        .headers()
        .get_all(header::SET_COOKIE)
    {
        response_options.append_header(header::SET_COOKIE, header_value.clone());
    }

    Ok(())
}
