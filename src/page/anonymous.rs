use crate::auth::identity;
use leptos::*;

#[component]
pub fn AnonymousIdentity() -> impl IntoView {
    use leptos::logging::{error, log, warn};
    use wasm_bindgen::JsValue;
    use web_sys::window;

    let resource = create_local_resource(move || (), |_| identity::generate_session());
    create_effect(move |_| match resource.get() {
        Some(Ok(session_response)) => {
            let message = match serde_json::to_string(&session_response) {
                Ok(session) => {
                    leptos::logging::log!("Session: {}", session);
                    session
                }
                Err(error) => error.to_string(),
            };
            let opener = window().unwrap().parent().unwrap().unwrap();
            match opener.post_message(&JsValue::from_str(&message), "*") {
                Err(error) => log!("post result: {:?}", error),
                Ok(_) => {}
            }
        }
        Some(Err(error)) => {
            warn!("{}", error.to_string());
        }
        None => error!("No session generated"),
    });

    view! {
    <>
    </>
    }
}
