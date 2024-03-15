use crate::auth::identity::generate_session;
use leptos::*;

#[component]
pub fn AnonymousIdentity() -> impl IntoView {
    use crate::constants;
    use leptos::logging::{error, log};
    use leptos_use::use_window;
    use wasm_bindgen::JsValue;

    let resource = create_local_resource(move || (), |_| generate_session());
    create_effect(move |_| match resource.get() {
        Some(Ok(session_response)) => {
            let message = match serde_json::to_string(&session_response) {
                Ok(session) => {
                    log!("Session: {}", session.len());
                    session
                }
                Err(error) => error.to_string(),
            };
            let parent = use_window().as_ref().unwrap().parent().unwrap().unwrap();
            // TODO: skip for window.self
            match parent.post_message(&JsValue::from_str(&message), constants::APP_DOMAIN.as_str())
            {
                Err(error) => log!("post result: {:?}", error),
                Ok(_) => {}
            }
        }
        Some(Err(error)) => {
            error!("{}", error.to_string());
        }
        None => {}
    });

    view! {
    <>
    </>
    }
}
