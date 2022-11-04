# Note that this script isn't meant to be updated frequently, as TestNet / MainNet dependencies are permanent / more long lived (than local/sandbox ones).
# I forgot how this script is updated - appearently manually? could only find code to generate build_local.sh in core
NETWORK=test ENV=test DATA_TYPE=real FUNDS_ASSET_ID=81166440 CAPI_ADDRESS=TODO wasm-pack build --out-dir ../next/wasm --release --target web
