pub const SRC: &str = r#"
#pragma version 5
gtxn 0 TypeEnum
int appl
==
gtxn 0 ApplicationID
int 0
==
&&
bnz main_l16
global GroupSize
int 1
==
bnz main_l15
global GroupSize
int 2
==
bnz main_l6
global GroupSize
int 4
==
bnz main_l5
err
main_l5:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 1 TypeEnum
int appl
==
assert
gtxn 1 OnCompletion
int NoOp
==
assert
gtxn 2 TypeEnum
int axfer
==
assert
gtxn 3 TypeEnum
int axfer
==
assert
gtxn 0 Sender
gtxn 1 Sender
==
assert
byte "ReceivedTotal"
byte "ReceivedTotal"
app_global_get
gtxn 3 AssetAmount
+
app_global_put
int 1
return
main_l6:
gtxn 0 NumAppArgs
int 1
==
bnz main_l10
global GroupSize
int 2
==
bnz main_l9
err
main_l9:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 1 TypeEnum
int axfer
==
assert
gtxn 0 Sender
gtxn 1 Sender
==
assert
gtxn 1 XferAsset
int TMPL_CAPI_ASSET_ID
==
assert
gtxn 1 AssetAmount
int 0
>
assert
gtxn 0 Sender
byte "Shares"
gtxn 0 Sender
byte "Shares"
app_local_get
gtxn 1 AssetAmount
+
app_local_put
gtxn 0 Sender
byte "HarvestedTotal"
gtxn 0 Sender
byte "Shares"
app_local_get
int TMPL_SHARE_SUPPLY
*
int TMPL_SHARE_SUPPLY
/
byte "ReceivedTotal"
app_global_get
*
int TMPL_PRECISION
/
gtxn 0 Sender
byte "HarvestedTotal"
app_local_get
-
app_local_put
int 1
return
main_l10:
gtxna 0 ApplicationArgs 0
byte "harvest"
==
bnz main_l14
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
bnz main_l13
err
main_l13:
gtxn 0 OnCompletion
int CloseOut
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
gtxn 1 AssetAmount
gtxn 0 Sender
byte "Shares"
app_local_get
==
assert
int 1
return
main_l14:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
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
gtxn 0 Sender
byte "Shares"
app_local_get
int TMPL_SHARE_SUPPLY
*
int TMPL_SHARE_SUPPLY
/
byte "ReceivedTotal"
app_global_get
*
int TMPL_PRECISION
/
gtxn 0 Sender
byte "HarvestedTotal"
app_local_get
-
gtxn 1 AssetAmount
>=
assert
gtxn 0 Sender
byte "HarvestedTotal"
gtxn 0 Sender
byte "HarvestedTotal"
app_local_get
gtxn 1 AssetAmount
+
app_local_put
int 1
return
main_l15:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int OptIn
==
assert
int 1
return
main_l16:
int 1
return
"#;