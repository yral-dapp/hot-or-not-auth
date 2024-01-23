use cfg_if::cfg_if;
use leptos::SignalGet;
use leptos::*;
use leptos_router::{use_query, NavigateOptions, Params};
use oauth2::TokenResponse;

cfg_if! {
if #[cfg(feature="ssr")] {
use axum::{http::header, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, Key, PrivateCookieJar};
use crate::auth::identity::IdentityKeeper;
use leptos_axum::ResponseOptions;
use oauth2::{reqwest::{http_client}, AuthorizationCode, CsrfToken, PkceCodeVerifier, PkceCodeChallenge, Scope};
use tracing::log::info;
}
}

#[server]
async fn google_auth_url() -> Result<String, ServerFnError> {
    let identity_keeper =
        use_context::<IdentityKeeper>().ok_or_else(|| ServerFnError::new("Context not found!"))?;
    let mut jar: PrivateCookieJar =
        leptos_axum::extract_with_state::<PrivateCookieJar<Key>, IdentityKeeper, ServerFnErrorErr>(
            &identity_keeper,
        )
        .await?;
    let client = identity_keeper.oauth2_client;

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let pkce_verifier = pkce_verifier.secret();
    let csrf_token = csrf_token.secret();

    info!("b4 pkce sec: {}", pkce_verifier);
    info!("b4 csrf sec: {}", csrf_token);

    let mut pkce_verifier = Cookie::new("pkce_verifier", pkce_verifier.to_owned());
    pkce_verifier.set_domain("hot-or-not-web-leptos-ssr.fly.dev");
    pkce_verifier.set_http_only(true);
    jar = jar.add(pkce_verifier.clone());
    let mut csrf_token = Cookie::new("csrf_token", csrf_token.to_owned());
    csrf_token.set_domain("hot-or-not-web-leptos-ssr.fly.dev");
    csrf_token.set_http_only(true);
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
) -> Result<(String, u64), ServerFnError> {
    let identity_keeper =
        use_context::<IdentityKeeper>().ok_or_else(|| ServerFnError::new("Context not found!"))?;
    let mut jar: PrivateCookieJar =
        leptos_axum::extract_with_state::<PrivateCookieJar<Key>, IdentityKeeper, ServerFnErrorErr>(
            &identity_keeper,
        )
        .await?;
    let client = identity_keeper.oauth2_client;
    let csrf_token: Option<String> = match jar.get("csrf_token") {
        Some(val) => Some(val.value().to_owned()),
        None => None,
    };
    match csrf_token.clone() {
        Some(csrf) => {
            if !csrf.eq(&provided_csrf) {
                return Err(ServerFnError::new("Invalid CSRF token!"));
            }
        }
        None => return Err(ServerFnError::new("No CSRF token!")),
    }
    let pkce_verifier: Option<String> = match jar.get("pkce_verifier") {
        Some(val) => Some(val.value().to_owned()),
        None => None,
    };
    info!("aftr pkce sec: {}", pkce_verifier.clone().unwrap());
    info!("aftr csrf sec: {}", csrf_token.clone().unwrap());

    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier.unwrap());
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)?;

    info!("{:?}", &token_result);
    let access_token = token_result.access_token().secret();
    let expires_in = token_result.expires_in().unwrap().as_secs();
    let refresh_secret = token_result.refresh_token().unwrap().secret();
    let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
    let client = reqwest::Client::new();
    let response = client
        .get(user_info_url)
        .bearer_auth(access_token)
        .send()
        .await?;
    let email = if response.status().is_success() {
        let response_json: serde_json::Value = response.json().await?;
        leptos::logging::log!("{response_json:?}");
        response_json["email"]
            .as_str()
            .expect("email to parse to string")
            .to_string()
    } else {
        return Err(ServerFnError::ServerError(format!(
            "Response from google has status of {}",
            response.status()
        )));
    };

    let access_token = token_result.access_token().secret();
    info!("aftr access_token: {:?}", access_token);

    Ok((email, expires_in as u64))
}

#[component]
pub fn OAuth2Response() -> impl IntoView {
    let handle_g_auth_redirect = Action::<GoogleVerifyResponse, _>::server();
    let (email, set_email) = create_signal("".to_owned());

    let query = use_query::<OAuthParams>();
    let navigate = leptos_router::use_navigate();
    create_effect(move |_| {
        if let Some(Ok((email, expires_in))) = handle_g_auth_redirect.value().get() {
            leptos::logging::log!("{}", email);
            leptos::logging::log!("{}", expires_in);
            set_email.set(email);
            // navigate("/", NavigateOptions::default());
        }
    });

    create_effect(move |_| {
        if let Ok(OAuthParams { code, state }) = query.get_untracked() {
            handle_g_auth_redirect.dispatch(GoogleVerifyResponse {
                provided_csrf: state.unwrap(),
                code: code.unwrap(),
            });
        } else {
            leptos::logging::log!("error parsing oauth params");
        }
    });
    view! {
        <div>
            "email: " {email.get()}
        </div>
    }
}

#[derive(Params, Debug, PartialEq, Clone)]
pub struct OAuthParams {
    pub code: Option<String>,
    pub state: Option<String>,
}
