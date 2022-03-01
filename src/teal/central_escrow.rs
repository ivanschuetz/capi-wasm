pub const SRC: &str = r#"
#pragma version 4
// int 1

// TODO verify app calls etc.

global GroupSize
int 2 
==
gtxn 0 TypeEnum 
int appl
==
&&
gtxn 1 TypeEnum
int axfer
==
&&
bnz branch_harvest

global GroupSize
int 2
==

bnz branch_withdraw

global GroupSize
int 10
==
return

branch_harvest:

int 1
return

branch_withdraw:

// pay fee
gtxn 0 TypeEnum
int pay
==

// it's an asset transfer
gtxn 1 TypeEnum
int axfer
==
&&

// asset has the expected id 
gtxn 1 XferAsset
int {funds_asset_id}
==
&&

// project creator is the receiver
gtxn 1 AssetReceiver
addr {project_creator_address}
==
&&

"#;
