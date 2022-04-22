
# Build for CSS devs. It returns mock UI data - it doesn't need configuring a local network/sandbox or adding test data on testnet
#
# it's still configured to use testnet - to fetch tx params (there may be other uses in the future), which are still needed to create mock txs
#
# (the mock txs are needed for the UI, as JS passes them to the wallets, 
# and having to sign with the wallet is prob also good for the CSS devs to get a more realistic idea of the flows)
#
# the asset and apps and not used, so the ids can be anything
# 
# note that we don't update the WASM TEAL here, as it's not used
#
NETWORK=test ENV=test DATA_TYPE=mock FUNDS_ASSET_ID=123 CAPI_APP_ID=123 CAPI_ASSET_ID=123 wasm-pack build --out-dir ../wasm-build --release
