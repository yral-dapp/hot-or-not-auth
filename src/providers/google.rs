use crate::auth::agent_js::SessionResponse;
use cfg_if::cfg_if;
use leptos::*;
use leptos_router::{use_query, Params};

cfg_if! {
if #[cfg(feature="ssr")] {
    use crate::{
        auth::{
            cookie,
            identity::{get_session_response, AppState},
        },
        store::cloudflare::{delete_kv, read_kv, write_kv},
    };
    use axum::{http::header, response::IntoResponse};
    use axum_extra::extract::cookie::{Cookie, Key, PrivateCookieJar, SameSite, SignedCookieJar};
    use chrono::{Duration, Utc};
    use leptos_axum::ResponseOptions;
    use oauth2::TokenResponse;
    use oauth2::{
        reqwest::async_http_client, AuthorizationCode, CsrfToken, PkceCodeChallenge,
        PkceCodeVerifier, Scope,
    };
    use std::collections::HashMap;
    use tracing::log::{error, info};
}
}

#[server]
async fn google_auth_url() -> Result<String, ServerFnError> {
    let app_state =
        use_context::<AppState>().ok_or_else(|| ServerFnError::new("Context not found!"))?;

    // enable after integration
    let signed_jar: SignedCookieJar =
        leptos_axum::extract_with_state::<SignedCookieJar<Key>, AppState>(&app_state).await?;
    let user_identity = match signed_jar.get("user_identity") {
        Some(val) => Some(val.value().to_owned()),
        None => None,
    }
    .ok_or_else(|| ServerFnError::new("User Session not found."))?;

    let mut jar: PrivateCookieJar =
        leptos_axum::extract_with_state::<PrivateCookieJar<Key>, AppState>(&app_state).await?;
    let client = app_state.oauth2_client;

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let pkce_verifier = pkce_verifier.secret();
    let csrf_token = csrf_token.secret();

    info!("b4 pkce sec: {}", pkce_verifier.len());
    info!("b4 csrf sec: {}", csrf_token.len());

    let auth_domain = app_state.auth_domain.host_str().unwrap().to_owned();

    let pkce_verifier = cookie::create_cookie(
        "pkce_verifier",
        pkce_verifier.to_owned(),
        auth_domain.to_owned(),
        SameSite::None,
    )
    .await;
    // jar = jar.remove(Cookie::from("pkce_verifier"));
    jar = jar.add(pkce_verifier);
    let csrf_token = cookie::create_cookie(
        "csrf_token",
        csrf_token.to_owned(),
        auth_domain,
        SameSite::None,
    )
    .await;
    // jar = jar.remove(Cookie::from("csrf_token"));
    jar = jar.add(csrf_token);

    let jar_into_response = jar.into_response();

    let response = expect_context::<ResponseOptions>();
    for header_value in jar_into_response.headers().get_all(header::SET_COOKIE) {
        response.append_header(header::SET_COOKIE, header_value.clone());
    }

    Ok(auth_url.to_string())
}

#[component]
pub fn Login() -> impl IntoView {
    use leptos_use::use_window;

    let g_auth = Action::<GoogleAuthUrl, _>::server();
    g_auth.dispatch(GoogleAuthUrl {});

    create_effect(move |_| {
        if let Some(Ok(redirect)) = g_auth.value().get() {
            use_window()
                .as_ref()
                .unwrap()
                .location()
                .set_href(&redirect)
                .unwrap();
        }
    });

    view! {
    <div>
    </div>
    }
}

#[server]
async fn google_verify_response(
    provided_csrf: String,
    code: String,
) -> Result<SessionResponse, ServerFnError> {
    let app_state =
        use_context::<AppState>().ok_or_else(|| ServerFnError::new("Context not found!"))?;
    let mut jar: PrivateCookieJar =
        leptos_axum::extract_with_state::<PrivateCookieJar<Key>, AppState>(&app_state).await?;
    let mut signed_jar: SignedCookieJar =
        leptos_axum::extract_with_state::<SignedCookieJar<Key>, AppState>(&app_state).await?;

    let user_identity = match signed_jar.get("user_identity") {
        Some(val) => Some(val.value().to_owned()),
        None => None,
    }
    .ok_or_else(|| ServerFnError::new("User Session not found."))?;

    let client = app_state.oauth2_client;
    let csrf_token = jar
        .get("csrf_token")
        .map(|cookie| cookie.value().to_owned())
        .ok_or_else(|| ServerFnError::new("No CSRF token found!"))?;
    if !csrf_token.eq(&provided_csrf) {
        return Err(ServerFnError::new("Invalid CSRF token!"));
    }
    jar = jar.remove(Cookie::from("csrf_token"));
    info!("aftr csrf: {}", csrf_token.len());
    let pkce_verifier = jar
        .get("pkce_verifier")
        .map(|cookie| cookie.value().to_owned())
        .ok_or_else(|| ServerFnError::new("No Verifier found!"))?;
    jar = jar.remove(Cookie::from("pkce_verifier"));
    let jar_into_response = jar.into_response();

    let response_options = expect_context::<ResponseOptions>();
    for header_value in jar_into_response.headers().get_all(header::SET_COOKIE) {
        response_options.append_header(header::SET_COOKIE, header_value.clone());
    }
    info!("aftr pkce: {}", pkce_verifier.len());

    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier);
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;

    info!(
        "token_result: f you need any inputs from my side.{:?}",
        &token_result
    );
    let access_token = token_result.access_token().secret();
    // TODO: check against delegate session. set whichever is lower
    let expires_in = token_result.expires_in().unwrap().as_secs();
    match token_result.refresh_token() {
        Some(secret) => info!("secret: {:?}", secret),
        None => {}
    }
    let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
    let reqwest_response = app_state
        .reqwest_client
        .get(user_info_url)
        .bearer_auth(access_token)
        .send()
        .await?;
    let access_token = token_result.access_token().secret();
    info!("aftr access_token: {:?}", access_token.len());

    let sub_openid = if reqwest_response.status().is_success() {
        let response_json: serde_json::Value = reqwest_response.json().await?;
        info!("response_json: {response_json:?}");
        response_json["sub"]
            .as_str()
            .expect("openid sub to parse to string")
            .to_string()
    } else {
        error!("Response status failed: {:?}", reqwest_response);
        return Err(ServerFnError::ServerError(format!(
            "Response from google has status of {}",
            reqwest_response.status()
        )));
    };

    let oauth2_kv_key = format!("google_sub_id_{}", sub_openid);
    let user_identity = match read_kv(&oauth2_kv_key, &app_state.cloudflare_config).await {
        Some(user_public_key) => {
            if !user_identity.eq(&user_public_key) {
                // returning user with different temporary session
                // delete current temp session
                let _ignore = delete_kv(&user_identity, &app_state.cloudflare_config).await;
                Some(user_public_key)
            } else {
                Some(user_identity)
            }
        }
        None => {
            let _ignore = write_kv(
                &oauth2_kv_key,
                &user_identity,
                HashMap::with_capacity(0),
                &app_state.cloudflare_config,
            )
            .await;
            Some(user_identity)
        }
    };
    let session_response = get_session_response(user_identity, &app_state.cloudflare_config).await;
    let auth_domain = app_state.auth_domain.host_str().unwrap().to_owned();

    let user_cookie = cookie::create_cookie(
        "user_identity",
        session_response.user_identity.to_owned(),
        auth_domain.to_owned(),
        SameSite::None,
    )
    .await;
    signed_jar = signed_jar.add(user_cookie);

    let expiration = Utc::now() + Duration::days(30);
    let exp_cookie = cookie::create_cookie(
        "expiration",
        expiration.to_string(),
        auth_domain,
        SameSite::None,
    )
    .await;
    signed_jar = signed_jar.add(exp_cookie);

    let signed_jar_into_response = signed_jar.into_response();
    for header_value in signed_jar_into_response
        .headers()
        .get_all(header::SET_COOKIE)
    {
        response_options.append_header(header::SET_COOKIE, header_value.clone());
    }

    Ok(session_response)
}

#[component]
pub fn OAuth2Response() -> impl IntoView {
    use crate::constants;
    use leptos::logging::log;
    use leptos_use::use_window;
    use wasm_bindgen::JsValue;
    use web_sys::Window;

    let handle_oauth2_redirect = Action::<GoogleVerifyResponse, _>::server();
    create_effect(move |_| {
        if let Some(Ok(session_response)) = handle_oauth2_redirect.value().get() {
            let message = match serde_json::to_string(&session_response) {
                Ok(session) => {
                    log!("Session: {}", session.len());
                    session
                }
                Err(error) => error.to_string(),
            };
            let window = use_window();
            let window = window.as_ref().unwrap();
            let opener = window.opener().unwrap();
            let opener = Window::from(opener);
            match opener.post_message(
                &JsValue::from_str(&message),
                constants::AUTH_DOMAIN.as_str(),
            ) {
                Err(error) => {
                    log!(
                        "post result to auth failed: {}",
                        error.as_string().unwrap_or("".to_owned())
                    );
                    // let _ = window.close();
                }
                Ok(_) => {
                    let _ = window.close();
                }
            }
        }
    });

    let query = use_query::<OAuthParams>();
    create_effect(move |_| {
        if let Ok(OAuthParams { code, state }) = query.get_untracked() {
            handle_oauth2_redirect.dispatch(GoogleVerifyResponse {
                provided_csrf: state.unwrap(),
                code: code.unwrap(),
            });
        } else {
            leptos::logging::log!("error parsing oauth params");
        }
    });
    view! {
        <div>
        </div>
    }
}

#[derive(Params, Debug, PartialEq, Clone)]
pub struct OAuthParams {
    pub code: Option<String>,
    pub state: Option<String>,
}
