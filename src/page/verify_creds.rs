use leptos::{logging::error, *};

#[component]
pub fn verify_creds() -> impl IntoView {
    use leptos_router::use_params_map;
    use leptos_use::use_window;

    let params = use_params_map();
    let user_identity = move || params.with(|params| params.get("u").cloned());
    let expiration = move || params.with(|params| params.get("e").cloned());

    if user_identity().is_none() || expiration().is_none() {
        handle_error("Invalid parameters");
    }

    let resource = create_local_resource(
        move || {
            (
                user_identity().unwrap().clone(),
                expiration().unwrap().clone(),
            )
        },
        |(u, e)| verify_payload(u, e),
    );

    create_effect(move |_| match resource.get() {
        Some(Ok(redirect)) => {
            use_window()
                .as_ref()
                .unwrap()
                .location()
                .set_href(&redirect)
                .unwrap();
        }
        Some(Err(error)) => {
            error!("Error verifying credentials: {}", error.to_string());
            handle_error("Invalid credentials");
        }
        None => {
            error!("No server response!");
            handle_error("No server response");
        }
    });

    view! {
        <>
        </>
    }
}

fn handle_error(message: &str) {
    use crate::constants;
    use leptos_use::use_window;
    use wasm_bindgen::JsValue;

    let window = use_window();
    let window = window.as_ref().unwrap();
    let opener = window.parent().unwrap().unwrap();
    match opener.post_message(
        &JsValue::from_str(&message),
        &constants::AUTH_URL.host_str().unwrap(),
    ) {
        Err(error) => error!("post result: {:?}", error),
        Ok(_) => {}
    }
    let _ = window.close();
}

#[server]
pub async fn verify_payload(
    user_identity: String,
    expiration: String,
) -> Result<String, ServerFnError> {
    use crate::auth::{cookie, identity::AppState};
    use axum::response::IntoResponse;
    use axum_extra::extract::{
        cookie::{Key, SameSite},
        SignedCookieJar,
    };
    use base64::{engine::general_purpose::URL_SAFE, Engine as _};
    use http::header;
    use leptos_axum::ResponseOptions;

    let user_identity = String::from_utf8(URL_SAFE.decode(user_identity).unwrap()).unwrap();
    let expiration = String::from_utf8(URL_SAFE.decode(expiration).unwrap()).unwrap();
    // TODO: decrypt

    let app_state = use_context::<AppState>().unwrap();
    let mut jar =
        leptos_axum::extract_with_state::<SignedCookieJar<Key>, AppState>(&app_state).await?;

    // TODO: validate from KV
    let auth_domain = app_state.auth_domain.host_str().unwrap().to_owned();

    let user_cookie = cookie::create_cookie(
        "user_identity",
        user_identity,
        auth_domain.to_owned(),
        SameSite::Strict,
    )
    .await;
    jar = jar.add(user_cookie);

    let expiration =
        cookie::create_cookie("expiration", expiration, auth_domain, SameSite::Strict).await;
    jar = jar.add(expiration);

    let jar_into_response = jar.into_response();
    let response = expect_context::<ResponseOptions>();
    for header_value in jar_into_response.headers().get_all(header::SET_COOKIE) {
        response.append_header(header::SET_COOKIE, header_value.clone());
    }
    Ok(format!("{}", app_state.auth_domain.as_str()))
}