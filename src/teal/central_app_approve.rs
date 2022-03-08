pub const SRC: &str = r#"
#pragma version 4
// int 1

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Identification
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// app create
gtxn 0 ApplicationID
int 0
==
bnz branch_create_app

// app setup (executed by the creator after creation)
global GroupSize
int 10
==
bnz branch_setup_dao

// opt in (only investors opt in)
global GroupSize
int 1
==
bnz branch_optin

global GroupSize
int 2
==
bz not_group_size_2

gtxn 0 TypeEnum // unlock shares
int appl
==
int CloseOut
gtxn 0 OnCompletion // central opt out (TODO app ids?)
==
&&
gtxn 1 TypeEnum // unlock shares
int axfer
==
&&
bnz branch_unlock

global GroupSize
int 2 
==
// the harvest sender is the central escrow
gtxn 1 Sender // harvest tx
byte "CentralEscrowAddress"
app_global_get
==
&&
bnz branch_harvest

b branch_lock

not_group_size_2:

global GroupSize
int 4
==
// draining customer escrow 
gtxn 2 Sender // drain tx
byte "CustomerEscrowAddress"
app_global_get
==
&&
bnz branch_drain

global GroupSize
int 4
==
bnz branch_invest

b failure

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Handling
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

branch_create_app:
global GroupSize
int 1
==
assert
gtxn 0 TypeEnum
int appl
==
assert
b success

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

// setup can be done only once
// if one of the global variables is set already, exit with failure
byte "CentralEscrowAddress"
app_global_get
bnz failure
// Store escrow addresses
byte "CentralEscrowAddress"
gtxn 0 ApplicationArgs 0 
app_global_put
byte "CustomerEscrowAddress"
gtxn 0 ApplicationArgs 1 
app_global_put
byte "SharesAssetId"
gtxn 0 ApplicationArgs 2 
btoi
app_global_put
byte "FundsAssetId"
gtxn 0 ApplicationArgs 3
btoi
app_global_put
// initialize to 0
byte "CentralReceivedTotal"
int 0
app_global_put

b success

branch_optin:
int OptIn
gtxn 0 OnCompletion
==
b success

// duplicated in: app_central_approval, app_capi_approval
branch_lock:
gtxn 0 TypeEnum // app call
int appl
==
assert
int NoOp
gtxn 0 OnCompletion
==
assert
gtxn 0 NumAppArgs
int 1
==
assert
gtxn 1 TypeEnum // lock
int axfer
==
assert
callsub lock_shares
callsub save_project_id
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
byte "SharesAssetId"
app_global_get
==
assert
// TODO set on state
// gtxn 1 AssetReceiver
// addr /locking_escrow_address/
// ==
// assert

gtxn 2 TypeEnum // pay share price (with funds asset)
int axfer
==
assert
// Paying with the funds asset
gtxn 2 XferAsset
byte "FundsAssetId"
app_global_get
==
assert
gtxn 2 AssetReceiver
byte "CentralEscrowAddress"
app_global_get
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

callsub lock_shares
callsub save_project_id
b success

branch_drain:
gtxn 0 TypeEnum // app call
int appl
==
assert
int NoOp
gtxn 0 OnCompletion
==
assert
gtxn 1 TypeEnum // capi app call
int appl
==
assert
gtxn 1 ApplicationID
int {capi_app_id}
==
assert
int NoOp
gtxn 1 OnCompletion
==
assert
gtxn 2 TypeEnum // drain
int axfer
==
assert
gtxn 2 XferAsset
byte "FundsAssetId"
app_global_get
==
assert
gtxn 3 TypeEnum // capi share
int axfer
==
assert
// the same user is sending both app calls
gtxn 1 Sender
gtxn 0 Sender
==
assert
// the funds are being drained to the central escrow
gtxn 2 AssetReceiver
byte "CentralEscrowAddress"
app_global_get
==
assert
// the capi fee is being sent to the capi escrow
gtxn 3 AssetReceiver
addr {capi_escrow_address}
==
assert
////////////////////////////////////////
// check that capi fee is correct
////////////////////////////////////////
// get customer escrow funds
// NOTE that the account and asset have to be passed as account / foreign assets to the app call for this to work.
// we don't need to access the tx Accounts / Assets here - it's implied.
gtxn 2 Sender
gtxn 2 XferAsset
asset_holding_get AssetBalance
pop // ignore "did (asset) exist" flag - we expect the customer escrow to always know the funds asset (TODO double check)
dup // leave one (customer escrow balance) on the stack for sanity check after

// calculate capi share
int {precision}
*
int {capi_share} // already multiplied with precision
*
int {precision_square} // revert mult
/
dup // leave one (capi share) on the stack for sanity check after

// compare calculated share with xfer
gtxn 3 AssetAmount
==
////////////////////////////////////////
assert

////////////////////////////////////////
// sanity check: amount sent to central = customer escrow balance - capi fee
////////////////////////////////////////
- // customer escrow balance - share
gtxn 2 AssetAmount // amount sent to central
==
////////////////////////////////////////
assert

// TODO allow only to drain everything at once? - no particular reason but there's no reason to drain only a part 
// other than concurrent issues - e.g. receiving a payment after fetching the balance on the frontend, which should be not so frequent, and the tx can be repeated

// Increase total received amount
byte "CentralReceivedTotal"
byte "CentralReceivedTotal"
app_global_get
gtxn 2 AssetAmount // drain tx amount
+
app_global_put

b success

// duplicated: app_central_approval, central_escrow
// except here no check for app id
branch_harvest:
gtxn 0 TypeEnum // app call
int appl
==
assert
int NoOp
gtxn 0 OnCompletion
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
// the harvested asset is the funds asset 
gtxn 1 XferAsset
byte "FundsAssetId"
app_global_get
==
assert

////////////////////////////////////////
// update already harvested local state
////////////////////////////////////////
// calculate entitled harvest amount
gtxn 0 Sender
byte "Shares"
app_local_get
callsub entitled_harvest_amount_for_shares
// subtract already harvested amount
int 0
byte "HarvestedTotal"
app_local_get // if local state doesn't exist yet, this puts a 0 on the stack
-

// is the xfer less or equal to entitled amount?
gtxn 1 AssetAmount
>=
assert

// update harvested amount
int 0
byte "HarvestedTotal"
int 0
byte "HarvestedTotal"
app_local_get
gtxn 1 AssetAmount
+
app_local_put
////////////////////////////////////////
////////////////////////////////////////

b success

branch_unlock:
// unlocking > 0 shares
// TODO similar checks in other contracts
gtxn 1 AssetAmount
int 0
>
assert
// shares receiver is the app caller
// we can also check e.g. that all the app calls have the same sender
gtxn 1 AssetReceiver
gtxn 0 Sender
==
assert
// check shares xfer == owned shares count
gtxn 1 AssetAmount
int 0
byte "Shares"
app_local_get
==
assert
// check that shares is the DAO shares asset
gtxn 1 XferAsset
byte "SharesAssetId"
app_global_get
==
assert
b success
// check xfer sender is locking escrow omitted,
// if user sends a wrong xfer sender and it happens to have the DAO asset and it passes, it's a security problem of that sender, not of this app - TODO review

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Subroutines
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// expects the 2 first txs of invest / lock to be the app call and lock (shares transfer to locking escrow)
// duplicated in: app_central_approval, app_capi_approval
lock_shares:
// sanity: don't allow locking 0 shares 
gtxn 1 AssetAmount
int 0
!=
assert

// initialize / increment shares
gtxn 0 Sender
byte "Shares"
gtxn 0 Sender
byte "Shares"
app_local_get
gtxn 1 AssetAmount // shares bought
+
app_local_put

// initialize already retrieved (ends with app_local_put at the end of /////// block)
gtxn 0 Sender // sender of app call (investor)
byte "HarvestedTotal"
// NOTE that this sets HarvestedTotal to the entitled amount each time that the investor buys/locks shares
// meaning that investors may lose pending dividend by buying or locking new shares (as everything will be practically marked as "already harvested")
// TODO improve? - a non TEAL way could be to just automatically retrieve pending dividend in the same group 
// see more notes in old repo
gtxn 1 AssetAmount // shares bought
callsub entitled_harvest_amount_for_shares
app_local_put

retsub

save_project_id:
// save the project id in local state, so we can find projects where a user invested in (with the indexer)  
// TODO rename in CapiProject or similar - this key is used to filter for txs belonging to capi / project id use case
// - we don't have the app id when querying this, only the sender account and this key
gtxn 0 Sender
byte "Project"
gtxn 0 ApplicationArgs 0 // first/only arg of the application call (first in the group)
app_local_put

retsub

// What amount (share of total retrieved funds) correspond to investor's share
// Does *not* account for already harvested funds.
// arg: owned shares
entitled_harvest_amount_for_shares:
// entitled % based on owned shares
int {precision}
*
int {investors_share} // already multiplied with precision
*
int {share_supply} 
/

// apply % to central total received
byte "CentralReceivedTotal"
app_global_get
*
int {precision_square} // revert mult by precision
/

retsub

harvested_total:
byte "HarvestedTotal"
app_local_get

success:
int 1
return

failure:
int 0
return

"#;
