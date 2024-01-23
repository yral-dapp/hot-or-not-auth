use leptos::*;

#[server(endpoint = "google_login")]
async fn google_auth_url() -> Result<String, ServerFnError> {
    use crate::auth::identity::IdentityKeeper;
    use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
    use tracing::log::info;

    info!("Google Login");
    let client = use_context::<IdentityKeeper>()
        .ok_or_else(|| ServerFnError::new("Context not found!"))?
        .oauth2_client;

    // Generate a PKCE challenge.
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        // .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();
    leptos_axum::redirect(auth_url.as_str());
    Ok(auth_url.to_string())
}

#[component]
pub fn Login() -> impl IntoView {
    // let oauth2_url = Action::<GoogleAuthUrl, _>::server();
    // oauth2_url.dispatch(GoogleAuthUrl {});
    // leptos::logging::log!("dispatched!");
    // create_effect(move |_| {
    //     if let Some(Ok(redirect)) = oauth2_url.value().get() {
    //         // let navigate = leptos_router::use_navigate();
    //         // navigate(&redirect, Default::default());
    //         leptos::logging::log!("navigated! {}", redirect);
    //         // window().location().set_href(&redirect).unwrap();
    //     }
    // });

    view! {
        <iframe src="https://accounts.google.com/o/oauth2/v2/auth/oauthchooseaccount?response_type=code&client_id=504622551087-ohtmmpnnlhifhht8bdmajfn0psk4i638.apps.googleusercontent.com&state=yniV_6w-ZqIw0RQHBOuqCA&code_challenge=dkAcdpkpiJS6mck9wY9JRvdSTphSb96W072iFp4tsm4&code_challenge_method=S256&redirect_uri=http%3A%2F%2Flocalhost%3A3000%2Fgoogle_oauth2_response&scope=openid%20email&service=lso&o2v=2&theme=glif&flowName=GeneralOAuthFlow"
        height = "100%" width = "100%">
        </iframe>
    }
}

#[component]
pub fn OAuth2Response() -> impl IntoView {
    view! {
        <div>
        </div>
    }
}
