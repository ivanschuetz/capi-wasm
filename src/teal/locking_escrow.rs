pub const SRC: &str = r#"
#pragma version 4
// int 1

// asset opt-ins on project creation
global GroupSize
int 10
==
bz branch_after_group_access
gtxn 5 XferAsset
int {shares_asset_id}
==
bnz branch_shares_opt_in

branch_after_group_access:

global GroupSize
int 2
==
bz after_tx_group_access
gtxn 0 TypeEnum // unlock shares
int appl
==
int CloseOut 
gtxn 0 OnCompletion // central opt out (TODO app ids?)
==
&&
bz after_tx_group_access
gtxn 1 TypeEnum // unlock shares
int axfer
==
bnz branch_unlock
after_tx_group_access:

int 0
return

branch_shares_opt_in:
gtxn 5 XferAsset
int {shares_asset_id}
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

branch_unlock:
gtxn 0 TypeEnum
int appl
==
gtxn 1 TypeEnum
int axfer
==
&&

"#;
