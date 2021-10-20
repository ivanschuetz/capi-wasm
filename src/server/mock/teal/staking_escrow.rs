pub const SRC: &str = r#"
#pragma version 4
// asset opt-ins on project creation
global GroupSize
int 2
==
bz branch_after_group_access
gtxn 0 XferAsset
int {shares_asset_id}
==
bnz branch_shares_and_votes_opt_in

branch_after_group_access:

global GroupSize
int 2
==
bnz branch_vote

global GroupSize
int 3
int 3 // slots
+
==
bnz branch_unstake

int 0
return

branch_shares_and_votes_opt_in:
gtxn 0 XferAsset
int {shares_asset_id}
==
gtxn 0 TypeEnum
int axfer
==
&&
gtxn 0 AssetAmount
int 0
==
&&

gtxn 0 Fee
int 1000
<=
&&
gtxn 0 RekeyTo
global ZeroAddress
==
&&
gtxn 0 AssetCloseTo
global ZeroAddress
==
&&

return

branch_vote:
// TODO more checks
gtxn 0 TypeEnum
int appl
==
gtxn 1 TypeEnum
int axfer
==
&&

return

branch_unstake:
gtxn 0 TypeEnum
int appl
==
gtxn 1 TypeEnum
int axfer
==
&&
gtxn 2 TypeEnum
int pay
==
&&
// TODO check slots

"#;
