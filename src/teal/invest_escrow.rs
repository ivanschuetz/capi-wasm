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
int 4
==
bnz branch_invest

b failure

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Handling
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// duplicated: app_central_approval, central_escrow, customer escrow, investing_escrow, locking escrow
// difference: app_central_approval no app_id check, escrows no state access
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

// duplicated in: app_central_approval, investing_escrow
// differences: state access, app id check
branch_invest:
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
gtxn 0 NumAppArgs
int 1
==
assert
gtxn 1 TypeEnum // receive shares
int axfer
==
assert
// Receiving the shares asset
gtxn 1 XferAsset
int {shares_asset_id}
==
assert
gtxn 1 AssetReceiver
addr {locking_escrow_address}
==
assert

gtxn 2 TypeEnum // pay share price (with funds asset)
int axfer
==
assert
// Paying with the funds asset
gtxn 2 XferAsset
int {funds_asset_id}
==
assert
gtxn 2 AssetReceiver
addr {central_escrow_address}
==
assert
gtxn 3 TypeEnum // shares optin
int axfer
==
assert
gtxn 3 AssetAmount
int 0
==
assert
// TODO is this optin check needed - if yes add it to other optins
gtxn 3 AssetReceiver
gtxn 3 Sender
==
assert

// the investor sends 3 txs (app call, pay for shares, shares optin)
gtxn 0 Sender
gtxn 2 Sender
==
assert
gtxn 2 Sender
gtxn 3 Sender
==
assert

// TODO check that gtxn 1 sender is invest escrow and receiver locking escrow? (add both to state)

// Paying the correct price for the bought shares
gtxn 2 AssetAmount // paid price
gtxn 1 AssetAmount // shares received
int {share_price}
*
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
