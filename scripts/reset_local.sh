# executes the core test to reset and prepare local network for manual testing
# note that this tests also generates reset_local.sh in this project: this is a bit of a "hack",
# normally we don't want core to depend on WASM
# but there were issues implementing the test here (tokio "core" dependency, see core for explanation)

cd ../../core # core repo location
cargo test --package base --lib -- testing::network_test_util::test::reset_and_fund_local_network --exact --nocapture --ignored
cd -
