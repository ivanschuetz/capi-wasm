pub const SRC: &str = r#"
#pragma version 4

// escrow setup on project creation
global GroupSize
int 2
==
gtxn 1 XferAsset
int {shares_asset_id}
==
&&
// the escrow is opting in
bnz branch_shares_and_votes_opt_in 

global GroupSize
int 5
int 3 // slots: when swapping (i.e. buying shares) the investor inits local state in the withdrawal request slots 
+
==
bnz branch_invest

// otherwise exit
int 0
return

// verifies that it's an opt-in to your asset + few security checks
// see more notes in old repo
branch_shares_and_votes_opt_in:
gtxn 1 XferAsset
int {shares_asset_id}
==
gtxn 1 TypeEnum
int axfer
==
&&
gtxn 1 AssetAmount
int 0
==
&&

gtxn 1 Fee
int 1000
<=
&&
gtxn 1 RekeyTo
global ZeroAddress
==
&&
gtxn 1 AssetCloseTo
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
