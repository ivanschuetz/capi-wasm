# Note that this script isn't meant to be updated frequently, as TestNet / MainNet dependencies are permanent / more long lived (than local/sandbox ones).
# I forgot how this script is updated - appearently manually? could only find code to generate build_local.sh in core
cargo test --package wasm --lib -- teal::update_teal::test::update_teal --exact --nocapture
NETWORK=test ENV=test DATA_TYPE=real FUNDS_ASSET_ID=81166440 CAPI_APP_ID=81166595 CAPI_ASSET_ID=81166572 wasm-pack build --out-dir ../wasm-build --release