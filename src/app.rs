use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/hot-or-not-auth.css"/>

        // sets the document title
        <Title text="Auth layer for internet"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
    <div class="flex flex-col justify-between font-inter rounded-xl mx-2 min-h-[640px] border border-gray-100 shadow-screen absolute top-1/2 -translate-y-1/2 overflow-hidden w-[calc(100%-16px)] sm:w-[450px] sm:left-1/2 sm:-translate-x-1/2 bg-frameBgColor border-frameBorderColor">
        <div class="w-full h-full p-[22px] flex-grow flex flex-col sm:rounded-xl justify-between text-black flex flex-col items-center">
            <div class="w-full h-full flex flex-col flex-1" id="auth-selection">
                <div class="flex flex-col items-center w-full pt-8">
                    <img src="/logo-full.svg" class="h-[43px]"/>
                    <h5 class="font-inter first-letter:capitalize font-bold text-black text-xl mt-5 mb-3 text-sm leading-6 text-black"></h5>
                    <div class="flex items-center mt-5 space-x-1 text-sm">
                        <span>Sign in to continue to <a class="transition-opacity text-linkColor hover:opacity-50" href="https://hotornot.wtf" target="_blank" rel="noreferrer">hotornot.wtf</a></span>
                    </div>
                </div>
                <div class="mt-7">
                    <form class="space-y-[14px]">
                        <div class="rounded-md SENSITIVE_CONTENT_NO_SESSION_RECORDING">
                            <label class="text-xs text-black leading-4 text-xs">Email</label>
                            <div class="flex relative">
                                <input type="email" class="flex-1 block w-full py-[7px] placeholder:text-secondary placeholder:text-sm disabled:bg-gray-200 disabled:text-secondary disabled:drop-shadow-none shadow-none border-1 border-gray-400 disabled:border-gray-200 rounded-md focus:ring-[3px] active:ring-blue-200 focus:ring-blue-200 active:border-blue-base active:bg-blue-50 h-12" name="email" autocomplete="off webauthn"/>
                                <span class="absolute right-0 h-full -translate-y-1/2 top-1/2 items-center flex"></span>
                            </div>
                        </div>
                        <button id="email-sign-button" class="transition duration-75 text-center text-sm first-letter:capitalize hover:no-underline font-bold border rounded-md outline-none p-[15px] leading-4 cursor-pointer disabled:cursor-not-allowed focus:ring-2 focus:ring-offset-[1px] focus:ring-black p-[15px] text-white border-transparent bg-primaryButtonColor hover:shadow-md hover:shadow-primaryButtonColor/40 active:bg-primaryButtonColor active:border-primaryButtonColor focus:border-primaryButtonColor focus:bg-primaryButtonColor disabled:bg-primaryButtonColor/20 disabled:shadow-none w-full block h-12 !p-0">
                            <div class="flex items-center justify-center space-x-2">
                                <div class="text-center">Continue with email</div>
                            </div>
                        </button>
                    </form>
                    <div class="flex items-center justify-between w-full text-sm text-secondary h-9 my-5">
                        <div class="flex w-full h-[1px] bg-gray-300"></div>
                        <div class="px-2">OR</div>
                        <div class="flex w-full h-[1px] bg-gray-300"></div>
                    </div>
                    <div class="hidden">
                        <div class="S9gUrf-YoZ4jf" style="position: relative;">
                            <div></div>
                            <div id="gsi_811805_489681-wrapper" class="L5Fo6c-sM5MNb">
                                <iframe src="https://accounts.google.com/gsi/button?text=continue_with&amp;shape=rectangular&amp;theme=outline&amp;type=standard&amp;size=large&amp;client_id=&amp;iframe_id=&amp;as=" id="" title="Sign in with Google Button" style="display: block; position: relative; top: 0px; left: 0px; height: 44px; width: 220px; border: 0px; margin: -2px -10px;" tabindex="-1"></iframe>
                                <div id="overlay" class="L5Fo6c-bF1uUb" tabindex="0"></div>
                            </div>
                        </div>
                    </div>
                    <div class="w-full">
                        <button id="google-sign-button" class="transition duration-75 text-center text-sm first-letter:capitalize hover:no-underline font-bold border rounded-md outline-none p-[15px] leading-4 cursor-pointer disabled:cursor-not-allowed focus:ring-2 focus:ring-offset-[1px] focus:ring-black p-[15px] text-black bg-transparent border-secondaryButtonColor hover:text-white hover:bg-secondaryButtonColor hover:border-secondaryButtonColor hover:shadow-md active:text-white active:bg-secondaryButtonColor focus:text-white focus:bg-secondaryButtonColor disabled:shadow-none disabled:bg-secondaryButtonColor/50 disabled:border-secondaryButtonColor/50 disabled:text-secondary w-full block h-12 !p-0">
                            <div class="flex items-center justify-center space-x-2">
                                <div class="flex items-center justify-center w-6 h-6">
                                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                        <path fill="#4285F4" d="M22 12.2316C22 11.5589 21.9404 10.9204 21.8382 10.2988H12.2172V14.1387H17.7259C17.479 15.3988 16.7552 16.4631 15.6825 17.1868V19.7411H18.9689C20.8932 17.9616 22 15.3392 22 12.2316Z"></path>
                                        <path fill="#34A853" d="M12.2199 22.2165C14.9785 22.2165 17.2859 21.297 18.9717 19.7389L15.6852 17.1846C14.7657 17.7976 13.5992 18.1723 12.2199 18.1723C9.55498 18.1723 7.29871 16.3758 6.48986 13.9492H3.1012V16.5801C4.7785 19.9177 8.22676 22.2165 12.2199 22.2165Z"></path>
                                        <path fill="#FBBC05" d="M6.48712 13.9484C6.27426 13.3354 6.16358 12.6798 6.16358 11.9986C6.16358 11.3175 6.28278 10.6619 6.48712 10.0489V7.41797H3.09846C2.40029 8.79727 2.00012 10.3469 2.00012 11.9986C2.00012 13.6504 2.40029 15.2 3.09846 16.5793L6.48712 13.9484Z"></path>
                                        <path fill="#EA4335" d="M12.2199 5.82746C13.7269 5.82746 15.0722 6.34683 16.1365 7.36002L19.0483 4.44816C17.2859 2.7964 14.9785 1.7832 12.2199 1.7832C8.22676 1.7832 4.7785 4.08205 3.1012 7.41962L6.48986 10.0505C7.29871 7.62396 9.55498 5.82746 12.2199 5.82746Z"></path>
                                    </svg>
                                </div>
                                <div class="text-center">Continue with Google</div>
                            </div>
                        </button>
                    </div>
                    <button id="other-sign-button" class="transition duration-75 text-center text-sm first-letter:capitalize hover:no-underline font-bold border rounded-md outline-none p-[15px] leading-4 cursor-pointer disabled:cursor-not-allowed focus:ring-2 focus:ring-offset-[1px] focus:ring-black p-[15px] text-linkColor border-transparent hover:bg-linkColor/10 hover:underline active:bg-linkColor/20 disabled:bg-white disabled:border-transparent disabled:text-secondary w-full block h-12 !p-0">
                        <div class="flex items-center justify-center space-x-2">
                            <div class="text-center">"Other sign in options"</div>
                        </div>
                    </button>
                </div>
                <div class="flex-1"></div>
                <div class="flex items-center justify-between w-full mt-6">
                    <div class="flex text-xs font-medium text-secondary space-x-2.5">
                        <a href="./legal/terms" target="_blank" class="hover:opacity-80" rel="noreferrer">"Terms of service"</a>
                        <a href="./legal/privacy" target="_blank" class="hover:opacity-80" rel="noreferrer">"Privacy policy"</a>
                    </div>
        <!-- "logo" -->
                </div>
            </div>
        </div>
    </div>
        }
}
