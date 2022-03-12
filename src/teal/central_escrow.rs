pub const SRC: &str = r#"
#pragma version 5
global GroupSize
int 10
==
bnz main_l8
global GroupSize
int 2
==
bnz main_l3
err
main_l3:
gtxn 0 TypeEnum
int appl
==
gtxn 0 ApplicationID
int TMPL_CENTRAL_APP_ID
==
&&
gtxn 1 TypeEnum
int axfer
==
&&
bnz main_l7
global GroupSize
int 2
==
bnz main_l6
err
main_l6:
gtxn 0 TypeEnum
int pay
==
assert
gtxn 0 Sender
addr TMPL_PROJECT_CREATOR
==
assert
gtxn 1 TypeEnum
int axfer
==
assert
gtxn 1 XferAsset
int TMPL_FUNDS_ASSET_ID
==
assert
gtxn 1 AssetReceiver
addr TMPL_PROJECT_CREATOR
==
assert
int 1
return
main_l7:
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 1 XferAsset
int TMPL_FUNDS_ASSET_ID
==
assert
gtxn 0 Sender
gtxn 1 AssetReceiver
==
assert
int 1
return
main_l8:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 ApplicationID
int TMPL_CENTRAL_APP_ID
==
assert
gtxn 0 NumAppArgs
int 4
==
assert
gtxn 1 TypeEnum
int pay
==
assert
gtxn 1 Receiver
gtxna 0 ApplicationArgs 0
==
assert
gtxn 2 TypeEnum
int pay
==
assert
gtxn 2 Receiver
gtxna 0 ApplicationArgs 1
==
assert
gtxn 3 TypeEnum
int pay
==
assert
gtxn 4 TypeEnum
int pay
==
assert
gtxn 5 TypeEnum
int axfer
==
assert
gtxn 5 AssetAmount
int 0
==
assert
gtxn 6 TypeEnum
int axfer
==
assert
gtxn 6 AssetAmount
int 0
==
assert
gtxn 7 TypeEnum
int axfer
==
assert
gtxn 7 AssetAmount
int 0
==
assert
gtxn 8 TypeEnum
int axfer
==
assert
gtxn 8 AssetAmount
int 0
==
assert
gtxn 9 TypeEnum
int axfer
==
assert
gtxn 9 XferAsset
gtxna 0 ApplicationArgs 2
btoi
==
assert
int 1
return
"#;