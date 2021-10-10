pub const SRC: &str = r#"
#pragma version 4
global GroupSize
int 6
==
gtxn 2 XferAsset
int {shares_asset_id}
==
&&
gtxn 3 XferAsset
int {votes_asset_id}
==
&&
bnz branch_shares_and_votes_opt_in

global GroupSize
int 6
==
bnz branch_swap

// otherwise exit
int 0
return

// verifies that it's an opt-in to your asset + few security checks
// see more notes in old repo
branch_shares_and_votes_opt_in:
/////////////////
/////// shares
/////////////////
gtxn 2 XferAsset
int {shares_asset_id}
==
gtxn 2 TypeEnum
int axfer
==
&&
gtxn 2 AssetAmount
int 0
==
&&

gtxn 2 Fee
int 1000
<=
&&
gtxn 2 RekeyTo
global ZeroAddress
==
&&
gtxn 2 AssetCloseTo
global ZeroAddress
==
&&

/////////////////
/////// vote
/////////////////

gtxn 3 XferAsset
int {votes_asset_id}
==
&&
gtxn 3 TypeEnum
int axfer
==
&&
gtxn 3 AssetAmount
int 0
==
&&

gtxn 3 Fee
int 1000
<=
&&
gtxn 3 RekeyTo
global ZeroAddress
==
&&
gtxn 3 AssetCloseTo
global ZeroAddress
==
&&

/////////////////
/////////////////

b end_contract

branch_swap:
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

// check that votes count == shares count ("twin assets")
gtxn 3 AssetAmount
gtxn 4 AssetAmount
==
&&

end_contract:

"#;
