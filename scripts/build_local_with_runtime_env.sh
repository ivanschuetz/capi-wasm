# creates a local build that fetches the capi-related environment variables (funds asset, capi address, maybe others in the future) at runtime
# this means that the developer has to pass them to the frontend somehow
# this is in order to allow people to run the scripts to reset the local environment (create new funds asset etc.), without granting access to the WASM source.
NETWORK=sandbox_private RUNTIME_ENV=1 DATA_TYPE=real wasm-pack build --out-dir ../next/wasm --release --target web
