pub const SRC: &str = r#"
#pragma version 5
global GroupSize
int 3
==
bnz main_l10
global GroupSize
int 2
==
bnz main_l3
err
main_l3:
gtxn 0 NumAppArgs
int 1
==
bnz main_l5
err
main_l5:
gtxna 0 ApplicationArgs 0
byte "harvest"
==
bnz main_l9
gtxn 0 TypeEnum
int appl
==
gtxn 0 NumAppArgs
int 1
==
&&
gtxna 0 ApplicationArgs 0
byte "unlock"
==
&&
gtxn 1 TypeEnum
int axfer
==
&&
bnz main_l8
err
main_l8:
gtxn 0 OnCompletion
int CloseOut
==
assert
gtxn 0 ApplicationID
int TMPL_CAPI_APP_ID
==
assert
gtxn 1 TypeEnum
int axfer
==
assert
gtxn 1 XferAsset
int TMPL_CAPI_ASSET_ID
==
assert
gtxn 0 Sender
gtxn 1 AssetReceiver
==
assert
int 1
return
main_l9:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 ApplicationID
int TMPL_CAPI_APP_ID
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
gtxn 0 Sender
gtxn 1 AssetReceiver
==
assert
int 1
return
main_l10:
gtxn 0 TypeEnum
int pay
==
assert
gtxn 1 TypeEnum
int axfer
==
assert
gtxn 1 XferAsset
int TMPL_CAPI_ASSET_ID
==
assert
gtxn 2 TypeEnum
int axfer
==
assert
gtxn 2 XferAsset
int TMPL_FUNDS_ASSET_ID
==
assert
int 1
return
"#;