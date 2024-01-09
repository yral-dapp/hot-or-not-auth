dfx start --clean --background -vv
dfx canister create verify_principal_backend
dfx canister create verify_principal_frontend
dfx build
dfx canister install --all
