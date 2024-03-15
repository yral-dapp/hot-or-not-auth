use axum_extra::extract::cookie::{Cookie, SameSite};

#[cfg(feature = "ssr")]
pub async fn create_cookie<'c>(
    cookie_name: &'c str,
    cookie_value: String,
    cookie_domain: String,
    same_site: SameSite,
) -> Cookie<'c> {
    let mut cookie = Cookie::new(cookie_name, cookie_value);
    cookie.set_domain(cookie_domain.to_owned());
    cookie.set_same_site(same_site);
    cookie.set_path("/");
    set_cookie_expiry(&mut cookie);
    cookie.set_http_only(true);
    cookie.set_secure(true);
    match same_site {
        SameSite::None => cookie.set_partitioned(true),
        _ => {}
    }
    cookie
}

#[cfg(feature = "ssr")]
fn set_cookie_expiry(cookie: &mut Cookie) {
    let thirty_days = time::Duration::days(30);
    cookie.set_max_age(thirty_days);
    cookie.set_expires(time::OffsetDateTime::now_utc() + thirty_days);
}
