pub const SRC: &str = r#"
#pragma version 4

txn ApplicationID
int 0
==

global GroupSize
int 10
==
&&

bnz branch_create

global GroupSize
int 1 // central opt in - TODO what is this? who's opting in?
==
bnz branch_opt_in

after_args_access2:

global GroupSize
int 2
==
// basically also an investor setup, but when asset was acquired externally (instead of buying in the "ico")
bnz branch_locking_setup

global GroupSize
int 5
==
bnz branch_investor_setup

global GroupSize
int 3
==
gtxn 2 Sender // drain tx
addr {customer_escrow_address}
==
&&

bnz branch_drain

global GroupSize
int 3 
==
gtxn 2 Sender // harvest tx
addr {central_escrow_address}
==
&&

bnz branch_harvest

// opt out tx group
global GroupSize
int 3 // central optout + unlock shares  + pay fee for unlock shares
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

branch_locking_setup:

gtxn 0 TypeEnum // app call
int appl
==

gtxn 1 TypeEnum // lock
int axfer
==
&&

// don't allow locking 0 assets 
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

gtxn 1 AssetAmount // locked xfer (this will become "holdings", if the group passes)
callsub entitled_harvest_amount_for_shares
app_local_put

// save the project id in local state, so we can find projects were a user invested in (with the indexer)  
// TODO rename in CapiProject or maybe something more cryptic - this key name is used to identify the state as belonging to capi / project id use case
// - we don't have app id when querying this, only the sender account and this key
gtxn 0 Sender
byte "Project"
gtxn 0 ApplicationArgs 0 // first/only arg of the application call (first in the group)
app_local_put

return

branch_investor_setup:
// initialize investor's local state

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
callsub entitled_harvest_amount_for_shares
app_local_put

// save the project id in local state, so we can find projects were a user invested in (with the indexer)  
// TODO rename in CapiProject or maybe something more cryptic - this key name is used to identify the state as belonging to capi / project id use case
// - we don't have app id when querying this, only the sender account and this key
gtxn 0 Sender
byte "Project"
gtxn 0 ApplicationArgs 0 // first/only arg of the application call (first in the group)
app_local_put

int 1
return

branch_drain:
gtxn 0 TypeEnum // app call
int appl
==

gtxn 1 TypeEnum // pay fee
int pay
==
&&

gtxn 2 TypeEnum // drain
int axfer
==
&&

// Increase total received amount
byte "CentralReceivedTotal"
byte "CentralReceivedTotal"
app_global_get
gtxn 2 AssetAmount // drain tx amount
+
app_global_put

return

branch_harvest:
gtxn 0 TypeEnum // app call
int appl
==

gtxn 1 TypeEnum // pay fee
int pay
==
&&

gtxn 2 TypeEnum // harvest
int axfer
==
&&

// get the asset holdings of caller
gtxn 0 Sender
byte "Shares"
app_local_get
callsub entitled_harvest_amount_for_shares

// how much user has already harvested
int 0
byte "HarvestedTotal"
app_local_get // if local state doesn't exist yet, this puts a 0 on the stack

// how much user is entitled to harvest now
-

gtxn 2 AssetAmount
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
gtxn 2 AssetAmount // harvest tx amount
+
app_local_put

int 1
return

branch_opt_out:

// check there's shares xfer
gtxn 1 TypeEnum // unlock
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

return

// local state (owned shares) is cleared automatically by CloseOut

// What amount (share of total retrieved funds) correspond to investor's share
// Does *not* account for already harvested funds.
// arg: owned shares
entitled_harvest_amount_for_shares:

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
