use crate::auth::agent_js::SessionResponse;
use cfg_if::cfg_if;
use leptos::SignalGet;
use leptos::*;
use leptos_router::{use_query, Params};
use oauth2::TokenResponse;

cfg_if! {
if #[cfg(feature="ssr")] {
use axum::{http::header, response::IntoResponse};
use axum_extra::extract::cookie::{SameSite, Cookie, Key, PrivateCookieJar, SignedCookieJar};
use crate::auth::{identity::{AppState, generate_session}};
use leptos_axum::ResponseOptions;
use oauth2::{reqwest::{async_http_client}, AuthorizationCode, CsrfToken, PkceCodeVerifier, PkceCodeChallenge, Scope};
use tracing::log::{info, error};
}
}

#[server]
async fn google_auth_url() -> Result<String, ServerFnError> {
    let app_state =
        use_context::<AppState>().ok_or_else(|| ServerFnError::new("Context not found!"))?;

    // enable after integration
    let signed_jar: SignedCookieJar =
        leptos_axum::extract_with_state::<SignedCookieJar<Key>, AppState>(&app_state).await?;
    /*
    let _user_identity = match signed_jar.get("user_identity") {
        Some(val) => Some(val.value().to_owned()),
        None => None,
    }
    .ok_or_else(|| ServerFnError::new("User Session not found."))?;
    */

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

    let mut pkce_verifier = Cookie::new("pkce_verifier", pkce_verifier.to_owned());
    pkce_verifier.set_domain(app_state.auth_cookie_domain.clone());
    pkce_verifier.set_same_site(SameSite::Strict);
    pkce_verifier.set_http_only(true);
    jar = jar.remove(Cookie::from("pkce_verifier"));
    jar = jar.add(pkce_verifier.clone());
    let mut csrf_token = Cookie::new("csrf_token", csrf_token.to_owned());
    csrf_token.set_domain(app_state.auth_cookie_domain);
    csrf_token.set_same_site(SameSite::Strict);
    csrf_token.set_http_only(true);
    jar = jar.remove(Cookie::from("csrf_token"));
    jar = jar.add(csrf_token.clone());

    let jar_into_response = jar.into_response();

    let response = expect_context::<ResponseOptions>();
    for header_value in jar_into_response.headers().get_all(header::SET_COOKIE) {
        response.append_header(header::SET_COOKIE, header_value.clone());
    }

    Ok(auth_url.to_string())
}

#[component]
pub fn Login() -> impl IntoView {
    let g_auth = Action::<GoogleAuthUrl, _>::server();
    g_auth.dispatch(GoogleAuthUrl {});

    create_effect(move |_| {
        if let Some(Ok(redirect)) = g_auth.value().get() {
            window().location().set_href(&redirect).unwrap();
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

    let response = expect_context::<ResponseOptions>();
    for header_value in jar_into_response.headers().get_all(header::SET_COOKIE) {
        response.append_header(header::SET_COOKIE, header_value.clone());
    }
    info!("aftr pkce: {}", pkce_verifier.len());

    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier);
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;

    info!("token_result: {:?}", &token_result);
    let access_token = token_result.access_token().secret();
    let expires_in = token_result.expires_in().unwrap().as_secs();
    match token_result.refresh_token() {
        Some(secret) => info!("secret: {:?}", secret),
        None => {}
    }
    let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
    let response = app_state
        .reqwest_client
        .get(user_info_url)
        .bearer_auth(access_token)
        .send()
        .await?;
    let sub_openid = if response.status().is_success() {
        let response_json: serde_json::Value = response.json().await?;
        info!("response_json: {response_json:?}");
        response_json["sub"]
            .as_str()
            .expect("openid sub to parse to string")
            .to_string()
    } else {
        error!("Response status failed: {:?}", response);
        return Err(ServerFnError::ServerError(format!(
            "Response from google has status of {}",
            response.status()
        )));
    };

    let access_token = token_result.access_token().secret();
    info!("aftr access_token: {:?}", access_token.len());
    // TODO: add to user map for reference
    let session_response = generate_session().await?;

    Ok(session_response)
}

#[component]
pub fn OAuth2Response() -> impl IntoView {
    use leptos::logging::log;
    use wasm_bindgen::JsValue;
    use web_sys::{window, Window};

    let handle_oauth2_redirect = Action::<GoogleVerifyResponse, _>::server();
    create_effect(move |_| {
        if let Some(Ok(session_response)) = handle_oauth2_redirect.value().get() {
            let message = match serde_json::to_string(&session_response) {
                Ok(session) => session,
                Err(error) => error.to_string(),
            };
            let opener = window().unwrap().opener().unwrap();
            let opener = Window::from(opener);
            match opener.post_message(&JsValue::from_str(&message), "*") {
                Err(error) => log!(
                    "post result: {}",
                    error.as_string().unwrap_or("".to_owned())
                ),
                Ok(_) => {}
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
