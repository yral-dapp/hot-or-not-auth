use leptos::*;
use leptos_router::A;

#[component]
pub fn LandingPage() -> impl IntoView {
    // use crate::providers::google::GoogleAuthUrl;

    // let oauth2_url = Action::<GoogleAuthUrl, _>::server();
    // create_effect(move |_| {
    //     if let Some(Ok(redirect)) = oauth2_url.value().get() {
    //         // let navigate = leptos_router::use_navigate();
    //         // navigate(&redirect, Default::default());
    //         window().location().set_href(&redirect).unwrap();
    //     }
    // });

    view! {
    <div class="fade-in absolute z-[100] block h-full w-full bg-black/90 text-white">
        <div class="flex h-full w-full flex-col items-center justify-center space-y-20 overflow-y-auto">
            <span class="text-3xl font-bold">
                "Join Hot or Not"
            </span>
            <span class="text-3xl font-bold">
                <img src="/logo-full.svg" class="h-[129px]"/>
            </span>
            <div class="flex w-full max-w-md flex-col items-center space-y-4 px-8">
                <div class="py-4">"Create an account using"</div>
                <A class="flex items-center duration-200 transition-all rounded-full !select-none justify-center
        focus:outline-none px-4 py-3 font-semibold text-white bg-orange-500 shadow-button-primary focus:bg-orange-700 
        flex h-12 w-full items-center space-x-3 !bg-white font-normal !text-black"
        // on:click=move|_| oauth2_url.dispatch(GoogleAuthUrl{})>
        href = "/google_login">
                    <svg class="h-6 w-6" id="google-logo" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 186.7 190.5">
                        <path fill="#4285f4" d="M95.3 78v36.8h51.2a44 44 0 0 1-19.1 28.7l30.9 24a93 93 0 0 0 28.4-70c0-6.8-.6-13.3-1.7-19.6z"/>
                        <path fill="#34a853" d="m41.9 113.4-7 5.3L10.2 138a95.2 95.2 0 0 0 85 52.6 91 91 0 0 0 63-23l-30.8-24a56.9 56.9 0 0 1-85.5-30z"/>
                        <path fill="#fbbc05" d="M10.2 52.6a94 94 0 0 0 0 85.3L42 113.3a57 57 0 0 1 0-36.1z"/>
                        <path fill="#ea4335" d="M95.3 38A52 52 0 0 1 131.7 52L159 25A91.4 91.4 0 0 0 95.2 0a95 95 0 0 0-85 52.6L42 77.2a56.9 56.9 0 0 1 53.4-39.3z"/>
                    </svg>
                    <span>"Login with Google"</span>
                </A>
                <A class="flex items-center duration-200 transition-all rounded-full !select-none justify-center
        focus:outline-none px-4 py-3 font-semibold text-white border-white/40 focus:bg-white/20 border-2 bg-transparent
        h-12 w-full space-x-2 py-3" href="/internetcomputer">
                    <svg class="w-8" id="dfinity-logo" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 197 97">
                        <path fill="#F15A24" d="M148.7.4c-10.9 0-22.8 5.8-35.4 17.3-6 5.4-11.2 11.2-15 15.9l12.8 14.3c3.6-4.5 8.9-10.6 14.9-16.1 11.2-10.2 18.5-12.3 22.7-12.3a28.9 28.9 0 1 1 0 57.8L146 77a35 35 0 0 0 14.2 3.6c28.8 0 34.4-19.5 34.8-20.9A48 48 0 0 0 148.7.4Z"/>
                        <path fill="#ED1E79" d="M48.1 96.3c10.9 0 22.8-5.8 35.4-17.3 6-5.4 11.2-11.2 15-15.9L85.7 48.8c-3.6 4.5-8.9 10.6-14.9 16.1-11.2 10.2-18.5 12.3-22.7 12.3a28.9 28.9 0 1 1 0-57.8l2.7.3a35 35 0 0 0-14.2-3.6C7.8 16.1 2.2 35.6 1.8 37a48 48 0 0 0 46.3 59.3Z"/>
                        <path fill="#29ABE2" d="M70 32.2c-3.1-3-18.5-15.2-33.3-15.6-26.3-.6-34 18-34.6 20.5C7.1 16.1 25.9.5 48.2.4c18.2 0 36.5 17.4 50.1 33.2l.1-.1 12.8 14.3s7.6 8.8 15.7 16.6c3.1 3 18.5 15.1 33.2 15.5 27 .7 34.4-19 34.9-20.5a47.8 47.8 0 0 1-46.1 36.9c-18.2 0-36.5-17.5-50.2-33.2l-.1.1-12.8-14.3c-.1-.1-7.7-8.9-15.8-16.7Zm-68 5V37c.1.1.1.2 0 .2Z"/>
                     </svg>
                    <span>"Internet Identity"</span>
                </A>
            </div>
        </div>
    </div>
        }
}
