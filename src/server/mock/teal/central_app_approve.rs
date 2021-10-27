pub const SRC: &str = r#"
#pragma version 4

txn ApplicationID
int 0
==

// Note: temporary (maybe): we want to send create app in a group with the other project setup txs
global GroupSize
int 1
==
&&

bnz branch_create

global GroupSize
int 1 // central opt in
int 3 // slots
+
==
bnz branch_opt_in

// vote validation
global GroupSize
int 2
==
gtxn 0 NumAppArgs
int 2
==
&&
bz after_args_access2
gtxn 0 ApplicationArgs 0 // action
byte b64 YnJhbmNoX3ZvdGU= // branch_vote
==
gtxn 1 ApplicationArgs 0 // action
byte b64 dmFsaWRhdGVfdm90ZQ== // validate_vote
==
&&
bnz branch_validate_votes

after_args_access2:

global GroupSize
int 2
int 3 // slots
+
==
// basically also an investor setup, but when asset was acquired externally (instead of buying in the "ico")
bnz branch_staking_setup

global GroupSize
int 5
int 3 // slots
+
==
bnz branch_investor_setup

global GroupSize
int 3
==
gtxn 1 Sender // drain tx
// escrow address (from where we're draining) TODO template
addr 3BW2V2NE7AIFGSARHF7ULZFWJPCOYOJTP3NL6ZQ3TWMSK673HTWTPPKEBA
==
&&

bnz branch_drain

global GroupSize
int 3 
==
gtxn 1 Sender // harvest tx
// central address (from where investors harvest) TODO template
addr P7GEWDXXW5IONRW6XRIRVPJCT2XXEQGOBGG65VJPBUOYZEJCBZWTPHS3VQ
==
&&

bnz branch_harvest

// opt out tx group must call all the slots (to clear local state) + unstaking txs
global GroupSize
int 3 // central optout + unstake shares  + pay fee for unstake shares
int 3 // slots
+
==
bz after_tx_group_access
gtxn 0 TypeEnum // unstake shares
int appl
==
int CloseOut 
gtxn 0 OnCompletion // central opt out (TODO app ids?)
==
&&
bz after_tx_group_access
gtxn 1 TypeEnum // unstake shares
int axfer
==
bnz branch_opt_out

after_tx_group_access:

int 0
return

branch_create:
int 1
return

branch_opt_in:
int 1 // TODO remove
return

branch_staking_setup:

gtxn 0 TypeEnum // app call
int appl
==

gtxn 1 TypeEnum // stake
int axfer
==
&&

// TODO slot opt-in
// /////////////////////////////////////
// // verify that the slots are being initialized (valid state)
// // note that on investing / staking this isn't as critical
// // if a malicious sender intentionally bypasses sending these transactions, they'll not be able to retrieve their shares
// // but we could of course have bugs, that omits these transactions, so for overall consistency
// /////////////////////////////////////
// gtxn 2 TypeEnum
// int appl
// ==
// &&
// gtxn 3 TypeEnum
// int appl
// ==
// &&
// gtxn 4 TypeEnum
// int appl
// ==
// &&
// /////////////////////////////////////

// don't allow staking 0 assets 
// no particular reason, just doesn't make sense
gtxn 1 AssetAmount
int 0
!=
&&

// initialize / increment shares
gtxn 0 Sender
byte "Shares"

gtxn 0 Sender
byte "Shares"
app_local_get

gtxn 1 AssetAmount // shares bought

+
app_local_put

// initialize HarvestedTotal local state to what the shares are entitled to
// see more notes in old repo
gtxn 0 Sender // sender of app call (investor)
byte "HarvestedTotal"

gtxn 1 AssetAmount // staked xfer (this will become "holdings", if the group passes)
callsub entitled_harvest_microalgos_for_shares
app_local_put

return

branch_investor_setup:
// initialize investor's local state

// TODO slot opt-in
// /////////////////////////////////////
// // verify that the slots are being initialized (valid state)
// // note that on investing / staking this isn't as critical
// // if a malicious sender intentionally bypasses sending these transactions, they'll not be able to retrieve their shares
// // but we could of course have bugs, that omits these transactions, so for overall consistency
// /////////////////////////////////////
// gtxn 5 TypeEnum
// int appl
// ==
// gtxn 6 TypeEnum
// int appl
// ==
// &&
// gtxn 7 TypeEnum
// int appl
// ==
// &&
// /////////////////////////////////////

// initialize / increment shares
gtxn 0 Sender
byte "Shares"

gtxn 0 Sender
byte "Shares"
app_local_get

gtxn 3 AssetAmount // shares bought

+
app_local_put

// initialize already retrieved (ends with app_local_put at the end of /////// block)
gtxn 0 Sender // sender of app call (investor)
byte "HarvestedTotal"

// TODO: important: this will reset "already harvested" to "entitled amount" *each time the investor buys shares*
// see more notes in old repo

// get the asset holdings of caller
gtxn 0 Sender
byte "Shares"
app_local_get
callsub entitled_harvest_microalgos_for_shares
app_local_put

int 1
return

branch_drain:
gtxn 0 TypeEnum // app call
int appl
==

gtxn 1 TypeEnum // drain
int pay
==
&&

gtxn 2 TypeEnum // pay fee
int pay
==
&&

// Increase total received amount
byte "CentralReceivedTotal"
byte "CentralReceivedTotal"
app_global_get
gtxn 1 Amount // drain tx amount
+
app_global_put

return

branch_harvest:
gtxn 0 TypeEnum // app call
int appl
==

gtxn 1 TypeEnum // harvest
int pay
==
&&

gtxn 2 TypeEnum // pay fee
int pay
==
&&

// get the asset holdings of caller
gtxn 0 Sender
byte "Shares"
app_local_get
callsub entitled_harvest_microalgos_for_shares

// how much user has already harvested
int 0
byte "HarvestedTotal"
app_local_get // if local state doesn't exist yet, this puts a 0 on the stack

// how much user is entitled to harvest now
-

gtxn 1 Amount
>=

&&
bnz branch_update_local_state
int 0
return

branch_update_local_state:
// Increase harvested amount
int 0
byte "HarvestedTotal"
int 0
byte "HarvestedTotal"
app_local_get
gtxn 1 Amount // harvest tx amount
+
app_local_put

int 1
return

branch_validate_votes:

// check that owned shares count == votes count (from any of the app calls - we checked they're equal above)
int 0
byte "Shares"
app_local_get // if local state doesn't exist yet, this puts a 0 on the stack
gtxn 0 ApplicationArgs 1
btoi
==

return

branch_opt_out:

// check there's shares xfer
gtxn 1 TypeEnum // unstake
int axfer
==

// check shares xfer goes to the investor (app call tx sender) - review whether this check is really needed
// we can also check e.g. that all the app calls have the same sender
gtxn 1 AssetReceiver
gtxn 0 Sender
==
&&

// check shares xfer == owned shares count
gtxn 1 AssetAmount
int 0
byte "Shares"
app_local_get
==
&&

// check that all slots are being opted out (votes removed)
int CloseOut
gtxn 3 OnCompletion
==
&&
// (TODO other slots)

return

// local state (owned shares) is cleared automatically by CloseOut

// How many microalgos (share of total retrieved funds) correspond to investor's share
// Does *not* account for already harvested funds.
// arg: owned shares
entitled_harvest_microalgos_for_shares:

int {precision}
*

int {investors_share} // already multiplied with precision
*

// the asset's total supply
int {asset_supply} 

// user's holdings % of total received
/

// how much has been transferred (total) to the central
byte "CentralReceivedTotal"
app_global_get

// percentage user is entitled to from received total
*

int {precision_square} // revert mult
/

retsub

"#;
