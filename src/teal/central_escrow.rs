pub const SRC: &str = r#"
#pragma version 4
// int 1

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Identification
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

global GroupSize
int 10
==
bnz branch_setup_dao

global GroupSize
int 2 
==
bz not_group_size_2

gtxn 0 TypeEnum 
int appl
==
gtxn 0 ApplicationID 
int {app_id}
==
&&
gtxn 1 TypeEnum
int axfer
==
&&
bnz branch_harvest

b branch_withdraw

not_group_size_2:

b failure

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Handling
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// duplicated: app_central_approval, central_escrow, customer escrow, investing_escrow, locking escrow
// difference: app_central_approval no app_id check, state access at the end 
branch_setup_dao:
gtxn 0 TypeEnum // setup app call
int appl
==
assert
int NoOp
gtxn 0 OnCompletion
==
assert
gtxn 0 ApplicationID 
int {app_id}
==
assert
gtxn 0 NumAppArgs
int 4
==
assert
// min balance creator -> central escrow
gtxn 1 TypeEnum
int pay
==
assert
gtxn 1 Receiver
gtxn 0 ApplicationArgs 0 
==
assert
// min balance creator -> customer escrow
gtxn 2 TypeEnum
int pay
==
assert
gtxn 2 Receiver
gtxn 0 ApplicationArgs 1
==
assert
// min balance creator -> locking escrow
gtxn 3 TypeEnum
int pay
==
assert
// min balance creator -> invest escrow
gtxn 4 TypeEnum
int pay
==
assert
// optin locking escrow to shares
gtxn 5 TypeEnum 
int axfer
==
assert
gtxn 5 AssetAmount 
int 0
==
assert
// optin invest escrow to shares
gtxn 6 TypeEnum
int axfer
==
assert
gtxn 6 AssetAmount 
int 0
==
assert
// optin central escrow to funds asset
gtxn 7 TypeEnum
int axfer
==
assert
gtxn 7 AssetAmount 
int 0
==
assert
// optin customer escrow to funds asset
gtxn 8 TypeEnum
int axfer
==
assert
gtxn 8 AssetAmount 
int 0
==
assert
// shares transfer creator -> investor escrow
gtxn 9 TypeEnum
int axfer
==
assert
gtxn 9 XferAsset
gtxn 0 ApplicationArgs 2
btoi
==
assert
b success

// duplicated: app_central_approval, central_escrow
// except here + check for app id
branch_harvest:
gtxn 0 TypeEnum // app call
int appl
==
assert
int NoOp
gtxn 0 OnCompletion
==
assert
gtxn 0 ApplicationID 
int {app_id}
==
assert
gtxn 1 TypeEnum // harvest
int axfer
==
assert
// the dividend receiver is the app call sender 
gtxn 1 AssetReceiver
gtxn 0 Sender
==
assert
b success

branch_withdraw:
// pay fee
gtxn 0 TypeEnum
int pay
==
assert
// the project creator sends the pay fee tx
gtxn 0 Sender
addr {project_creator_address}
==
assert
// asset withdrawal
gtxn 1 TypeEnum
int axfer
==
assert
// asset has the expected id 
gtxn 1 XferAsset
int {funds_asset_id}
==
assert
// project creator is the receiver
gtxn 1 AssetReceiver
addr {project_creator_address}
==
assert
b success

success:
int 1
return

failure:
int 0
return

"#;
