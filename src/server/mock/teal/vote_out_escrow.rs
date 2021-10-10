pub const SRC: &str = r#"
#pragma version 4

global GroupSize
int 6
==
bnz branch_opt_in

global GroupSize
int 2
==
bnz branch_reclaim

int 0
return

branch_opt_in:
// verify vote out opt in tx
// TODO use templates to insert transaction indices too (everywhere)
gtxn 4 XferAsset
int {votes_asset_id}
==
gtxn 4 TypeEnum
int axfer
==
&&
gtxn 4 AssetAmount
int 0
==
&&

gtxn 4 Fee
int 1000
<=
&&

gtxn 4 RekeyTo
global ZeroAddress
==
&&
gtxn 4 AssetCloseTo
global ZeroAddress
==
&&

return

branch_reclaim:
gtxn 0 TypeEnum
int axfer
==
gtxn 0 XferAsset
int {votes_asset_id}
==
&&
// the only valid place to transfer votes is back to the staking escrow
gtxn 0 AssetReceiver
addr {staking_escrow_address}
==
&&
gtxn 1 TypeEnum
int pay
==
&&

return

"#;
