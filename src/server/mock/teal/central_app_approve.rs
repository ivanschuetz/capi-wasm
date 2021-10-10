pub const SRC: &str = r#"
#pragma version 4

txn NumAppArgs
int 1
==
bz after_args_access // without this accessing args 0 fails for calls that have no args

txna ApplicationArgs 0
byte b64 dmFsaWRhdGVfaW52ZXN0b3Jfdm90ZXM= // validate_investor_votes
==
bnz branch_validate_investor_votes

after_args_access:

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
int 1
==
bnz branch_opt_in

global GroupSize
int 2
==
// basically also an investor setup, but when asset was acquired externally (instead of buying in the "ico")
bnz branch_staking_setup

global GroupSize
int 6
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

global GroupSize
int 3 
==
bnz branch_unstake

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

// don't allow staking 0 assets 
// no particular reason, just doesn't make sense
gtxn 1 AssetAmount
int 0
!=
&&

// initialize HarvestedTotal local state to what the shares are entitled to
// see more notes in old repo
gtxn 0 Sender // sender of app call (investor)
byte "HarvestedTotal"
/////////////////////////////////////
// how many algos is investor entitled to (according to xfer) -> stack
// TODO refactor (partly) with similar blocks in this contract. 
// this differs in that we get the shares count from the xfer tx instead of the investor's holdings.
/////////////////////////////////////
gtxn 1 AssetAmount // staked xfer (this will become "holdings", if the group passes)
int 100
*

// the asset's total supply
int {asset_supply} 

// user's holdings % of total received
/

// how much has been transferred (total) to the central
byte "CentralReceivedTotal"
app_global_get

// total percentage user is entitled to from received total
*

int 100 // revert * 100
/
/////////////////////////////////////
/////////////////////////////////////
app_local_put

// initialize asset holding
gtxn 0 Sender // sender of app call (investor)
byte "Shares"
gtxn 1 AssetAmount // shares staked
app_local_put

return

branch_investor_setup:
// initialize investor's local state

// initialize asset holding
gtxn 0 Sender // sender of app call (investor)
byte "Shares"
gtxn 3 AssetAmount // shares bought
app_local_put

// initialize already retrieved (ends with app_local_put at the end of /////// block)
gtxn 0 Sender // sender of app call (investor)
byte "HarvestedTotal"

// TODO: important: this will reset "already harvested" to "entitled amount" *each time the investor buys shares*
// see more notes in old repo

/////////////////////////////////////
// how many algos is investor currently entitled to (according to share holdings) -> stack
// TODO refactor with X (search for this text)
/////////////////////////////////////
// get the asset holdings of caller
gtxn 0 Sender
// important: the asset id has to be passed to the tx foreign assets too (otherwise call fails with "logic eval error: invalid Asset reference")
int {asset_id} 
asset_holding_get AssetBalance 
pop

int 100
*

// the asset's total supply
int {asset_supply} 

// user's holdings % of total received
/

// how much has been transferred (total) to the central
byte "CentralReceivedTotal"
app_global_get

// total percentage user is entitled to from received total
*

int 100 // revert * 100
/
/////////////////////////////////////
/////////////////////////////////////
app_local_put

// TODO support same user buying more shares
// see notes in old repo

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

/////////////////////////////////////
// how many algos is investor currently entitled to (according to share holdings) -> stack
// TODO refactor with X (search for this text)
/////////////////////////////////////
// get the asset holdings of caller

int 0
byte "Shares"
app_local_get // if local state doesn't exist yet, this puts a 0 on the stack

int 100
*

// the asset's total supply
int {asset_supply} 

// user's holdings % of total received
/

// how much has been transferred (total) to the central
byte "CentralReceivedTotal"
app_global_get

// total percentage user is entitled to from received total
*

int 100 // revert mult by 100
/
/////////////////////////////////////
/////////////////////////////////////

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

branch_validate_investor_votes:

// Check that votes being transferred == owned shares
// see more notes in old repo
gtxn 1 AssetAmount // votes amount (xfer tx)
int 0
byte "Shares"
app_local_get // if local state doesn't exist yet, this puts a 0 on the stack
==

return

branch_unstake:
gtxn 0 TypeEnum // app call
int appl
==

gtxn 1 TypeEnum // unstake
int axfer
==
&&

gtxn 2 TypeEnum // pay fee
int pay
==
&&

// shares to be unstaked has to match number of held shares
gtxn 1 AssetAmount
int 0 // investor account (TODO verify: is this the tx 0 sender?)
byte "Shares"
app_local_get
==
&&
"#;
