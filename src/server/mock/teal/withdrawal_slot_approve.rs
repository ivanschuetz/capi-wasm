pub const SRC: &str = r#"
#pragma version 4

// no-op TODO review
int {id}
pop
gtxn 0 NumAppArgs
int 2
==
bz after_2_args_access
gtxn 0 ApplicationArgs 0 // action
byte b64 YnJhbmNoX2luaXRfcmVxdWVzdA== // branch_init_request
==
bnz branch_init_request

after_2_args_access:

gtxn 0 NumAppArgs
int 1
==
bz after_one_arg_access
gtxn 0 ApplicationArgs 0 // action
byte b64 YnJhbmNoX3dpdGhkcmF3 // branch_withdraw
==
bnz branch_withdraw

after_one_arg_access:

gtxn 0 NumAppArgs
int 2
==
bz after_args_access
gtxn 0 ApplicationArgs 0 // action
byte b64 YnJhbmNoX3ZvdGU= // branch_vote
==
bnz branch_vote

after_args_access:

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

global GroupSize
int 1 // central opt in
int 3 // slots
+
==
txn TypeEnum
int appl
==
&&
int OptIn
gtxn 0 OnCompletion
==
&&
bnz branch_opt_in

global GroupSize
int 1
==
int NoOp 
gtxn 0 OnCompletion
==
&&
bnz branch_create

// stake
global GroupSize
int 2 // central opt in, xfer
int 3 // slots
+
==
gtxn 0 TypeEnum
int appl
==
&&
gtxn 1 TypeEnum
int axfer
==
&&
// TODO check slots
bnz branch_invest_or_stake

// invest
global GroupSize
int 5
int 3 // slots: initializes investor's local state
+
==
bnz branch_invest_or_stake

branch_init_request:

// there's no active request (amount is 0)
byte "Amount"
app_global_get
int 0
==

gtxn 0 TypeEnum // app call
int appl
==
&&

// amount >= 0 (setting 0 amount means clearing the request)
gtxn 0 ApplicationArgs 1 // withdrawal amount
btoi
int 0
>=
&&

// sender is project creator
gtxn 0 Sender
addr {project_creator_address}
==
&&

// save request amount
byte "Amount"
gtxn 0 ApplicationArgs 1 // withdrawal amount
btoi
app_global_put

return

branch_withdraw:

// there's an active request (amount > 0)
byte "Amount"
app_global_get
int 0
>

gtxn 0 TypeEnum // app call
int appl
==
&&

gtxn 1 TypeEnum // payment
int pay
==
&&

// payment amount == requested amount
gtxn 1 Amount
byte "Amount"
app_global_get
==
&&

// payment receiver is project creator
gtxn 1 Receiver
addr {project_creator_address}
==
&&

// enough votes
byte "Votes"
app_global_get
int {vote_threshold}
>=
&&

// reset amount
byte "Amount"
int 0
app_global_put

// reset votes
byte "Votes"
int 0
app_global_put

return

branch_vote:

gtxn 0 TypeEnum // app call (this app)
int appl
==

//////////////////////////
// central app validation
// checks that vote count == owned shares count
// owned shares count is tracked in central app (otherwise would have to be stored redundantly in each voting slot), so validation there
//////////////////////////

gtxn 1 TypeEnum // app call (central)
int appl
==
&&

// check that the central app call has label expected there
gtxn 1 ApplicationArgs 0 // action
byte b64 dmFsaWRhdGVfdm90ZQ== // validate_vote
==
&&

//////////////////////////
//////////////////////////

// votes > 0
gtxn 0 ApplicationArgs 1 // vote count
btoi
int 0
>
&&

// there's an active request (amount > 0)
byte "Amount"
app_global_get
int 0
>
&&

gtxn 0 Sender
byte "LVotes"
app_local_get
int 0
==
&&
bz branch_exit_false

// increment votes
byte "Votes"
byte "Votes"
app_global_get
gtxn 0 ApplicationArgs 1 // vote count
btoi
+
app_global_put

// set local votes
gtxn 0 Sender
byte "LVotes"
gtxn 0 ApplicationArgs 1 // vote count
btoi
app_local_put

// TODO review (also in other places) whether this int 1 after setting state is needed - probably shouldn't be here)
int 1
return

branch_opt_out:

// opting out from this slot (central checks for the other slots)
// TODO either insert transaction index corresponding to this slot or just check all the slots
int CloseOut 
gtxn 3 OnCompletion
==

// if the user cleared local state, disapprove
// !! TODO confirm -- CloseOut behavior unclear: is local state cleared or not when approval program rejects?
// but it prevents the transactions going through, in this case importantly the shares xfer to the investor
// so if investor clears their local state (assumed to be normally a malicious action), they lose their shares
// appart from this acting as a disincentive, it prevents double voting, effectively burning the shares
// -> shares can be only be bought from the investing_escrow ("initial offering") or in secondary markets (after unstaking)
// here the shares are in the staking escrow, and by clearing the owner's local state, can't be unstaked anymore.
int 0
byte "Valid"
app_local_get
int 1
==
&&
bz branch_exit_false

// remove possible votes

byte "Votes"

byte "Votes"
app_global_get

int 0
byte "LVotes"
app_local_get

-

app_global_put

int 1
return

branch_create:
int 1
return

branch_opt_in:
int 1
return

branch_invest_or_stake:

// set LVotes to 0, just to increment easily later
int 0
byte "LVotes"
int 0
app_local_put


// set a "valid state" flag - this helps to prevent issues like double voting if the user maliciously (or somehow accidentally) clears their local state (ClearState)
// local state can only be cleared totally, so this flag will be removed too, making the state invalid.
// we can control flow based on this state, e.g. preventing the shares xfer when unstaking if it's invalid

int 0
byte "Valid"
int 1
app_local_put

int 1
return

branch_exit_false:
int 0
return

// The local state will be cleared automatically (CloseOut)

"#;
