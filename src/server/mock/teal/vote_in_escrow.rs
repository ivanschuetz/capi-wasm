pub const SRC: &str = r#"
#pragma version 4
global GroupSize
int 6
==
bnz branch_opt_in

global GroupSize
int 4
==
bnz branch_consume_votes

branch_opt_in:
// verify vote in opt in tx
// TODO use templates to insert transaction indices too (everywhere)
gtxn 5 XferAsset
int {votes_asset_id}
==
gtxn 5 TypeEnum
int axfer
==
&&
gtxn 5 AssetAmount
int 0
==
&&

gtxn 5 Fee
int 1000
<=
&&

gtxn 5 RekeyTo
global ZeroAddress
==
&&
gtxn 5 AssetCloseTo
global ZeroAddress
==
&&

return

branch_consume_votes:

gtxn 0 TypeEnum // withdraw algos
int pay
==
gtxn 1 TypeEnum // pay withdraw algos fee
int pay
==
&&
gtxn 2 TypeEnum // consume votes
int axfer
==
&&
gtxn 3 TypeEnum // pay consume votes fee
int pay
==
&&

// consume votes checks
gtxn 2 XferAsset
int {votes_asset_id}
==
&&
gtxn 2 AssetAmount
int {votes_threshold_units}
>=
&&
gtxn 2 AssetReceiver
addr {votes_out_address}
==
&&

"#;
