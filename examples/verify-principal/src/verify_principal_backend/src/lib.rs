use ic_cdk::{api::caller, query};

#[query]
fn get_principal_id() -> String {
    caller().to_text()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;

        candid::export_service!();
        let dir = env::current_dir().unwrap();
        write(dir.join("verify_principal_backend.did"), __export_service()).expect("Write failed.");
    }
}
