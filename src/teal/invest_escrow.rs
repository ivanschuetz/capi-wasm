pub const SRC: &str = r#"
#pragma version 4
// int 1

// escrow setup on project creation
global GroupSize
int 8
==

bz after_tx_group_access

gtxn 6 XferAsset
int {shares_asset_id}
==
// the escrow is opting in
bnz branch_shares_opt_in 

after_tx_group_access:

global GroupSize
int 5
==
bnz branch_invest

// otherwise exit
int 0
return

// verifies that it's an opt-in to your asset + few security checks
// see more notes in old repo
branch_shares_opt_in:
gtxn 6 XferAsset
int {shares_asset_id}
==
gtxn 6 TypeEnum
int axfer
==
&&
gtxn 6 AssetAmount
int 0
==
&&

gtxn 6 Fee
int 1000
<=
&&
gtxn 6 RekeyTo
global ZeroAddress
==
&&
gtxn 6 AssetCloseTo
global ZeroAddress
==
&&

b end_contract

branch_invest:
gtxn 0 TypeEnum
int appl
==

gtxn 3 XferAsset
int {shares_asset_id}
==
&&
gtxn 3 AssetReceiver
addr {staking_escrow_address}
==
&&

// asset transfer uses our asset id, fee <= 1000, no close/rekey address
gtxn 3 Fee
int 1000 // TODO we get the fee from tx params: ensure that this condition is always met (or change condition)
<=
&&
gtxn 3 AssetCloseTo
global ZeroAddress
==
&&
gtxn 3 RekeyTo
global ZeroAddress
==
&&

// check that algos sent match asset receive * algo price per asset
gtxn 1 Amount // algos (send)
gtxn 3 AssetAmount // asset (receive)
int {asset_price} // price (microalgos) per asset
* 
==
&&

end_contract:
"#;
