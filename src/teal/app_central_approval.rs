pub const SRC: &str = r#"
#pragma version 5
gtxn 0 ApplicationID
int 0
==
bnz main_l20
global GroupSize
int 10
==
bnz main_l19
global GroupSize
int 1
==
bnz main_l18
global GroupSize
int 2
==
bnz main_l11
global GroupSize
int 4
==
bnz main_l6
err
main_l6:
global GroupSize
int 4
==
gtxn 2 Sender
byte "CustomerEscrowAddress"
app_global_get
==
&&
bnz main_l10
global GroupSize
int 4
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
gtxn 0 NumAppArgs
int 1
==
assert
gtxn 1 TypeEnum
int axfer
==
assert
gtxn 1 XferAsset
byte "SharesAssetId"
app_global_get
==
assert
gtxn 2 TypeEnum
int axfer
==
assert
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
gtxn 3 TypeEnum
int axfer
==
assert
gtxn 3 XferAsset
byte "SharesAssetId"
app_global_get
==
assert
gtxn 3 AssetAmount
int 0
==
assert
gtxn 3 AssetReceiver
gtxn 3 Sender
==
assert
gtxn 0 Sender
gtxn 2 Sender
==
assert
gtxn 2 Sender
gtxn 3 Sender
==
assert
gtxn 2 AssetAmount
gtxn 1 AssetAmount
int TMPL_SHARE_PRICE
*
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
int TMPL_PRECISION__
*
int TMPL_INVESTORS_SHARE
*
int TMPL_SHARE_SUPPLY
/
byte "CentralReceivedTotal"
app_global_get
*
int TMPL_PRECISION_SQUARE
/
gtxn 0 Sender
byte "HarvestedTotal"
app_local_get
-
app_local_put
gtxn 0 Sender
byte "Project"
gtxna 0 ApplicationArgs 0
app_local_put
int 1
return
main_l10:
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
gtxn 2 XferAsset
byte "FundsAssetId"
app_global_get
==
assert
gtxn 3 TypeEnum
int axfer
==
assert
gtxn 3 XferAsset
byte "FundsAssetId"
app_global_get
==
assert
gtxn 0 Sender
gtxn 1 Sender
==
assert
gtxn 2 AssetReceiver
byte "CentralEscrowAddress"
app_global_get
==
assert
gtxn 3 AssetReceiver
addr TMPL_CAPI_ESCROW_ADDRESS
==
assert
gtxn 2 Sender
gtxn 2 XferAsset
asset_holding_get AssetBalance
store 1
store 0
gtxn 2 Sender
gtxn 2 XferAsset
asset_holding_get AssetBalance
store 3
store 2
gtxn 3 AssetAmount
load 0
int TMPL_PRECISION__
*
int TMPL_CAPI_SHARE
*
int TMPL_PRECISION_SQUARE
/
==
assert
byte "CentralReceivedTotal"
byte "CentralReceivedTotal"
app_global_get
gtxn 2 AssetAmount
+
app_global_put
int 1
return
main_l11:
gtxn 0 TypeEnum
int appl
==
gtxn 0 OnCompletion
int CloseOut
==
&&
gtxn 1 TypeEnum
int axfer
==
&&
bnz main_l17
gtxn 1 Sender
byte "CentralEscrowAddress"
app_global_get
==
bnz main_l16
global GroupSize
int 2
==
bnz main_l15
err
main_l15:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
==
assert
gtxn 0 NumAppArgs
int 1
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
byte "SharesAssetId"
app_global_get
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
int TMPL_PRECISION__
*
int TMPL_INVESTORS_SHARE
*
int TMPL_SHARE_SUPPLY
/
byte "CentralReceivedTotal"
app_global_get
*
int TMPL_PRECISION_SQUARE
/
gtxn 0 Sender
byte "HarvestedTotal"
app_local_get
-
app_local_put
gtxn 0 Sender
byte "Project"
gtxna 0 ApplicationArgs 0
app_local_put
int 1
return
main_l16:
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
byte "FundsAssetId"
app_global_get
==
assert
gtxn 0 Sender
gtxn 1 AssetReceiver
==
assert
gtxn 0 Sender
byte "Shares"
app_local_get
int TMPL_PRECISION__
*
int TMPL_INVESTORS_SHARE
*
int TMPL_SHARE_SUPPLY
/
byte "CentralReceivedTotal"
app_global_get
*
int TMPL_PRECISION_SQUARE
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
main_l17:
gtxn 1 AssetAmount
int 0
>
assert
gtxn 1 AssetReceiver
gtxn 0 Sender
==
assert
gtxn 1 AssetAmount
gtxn 0 Sender
byte "Shares"
app_local_get
==
assert
gtxn 1 XferAsset
byte "SharesAssetId"
app_global_get
==
assert
int 1
return
main_l18:
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
main_l19:
gtxn 0 TypeEnum
int appl
==
assert
gtxn 0 OnCompletion
int NoOp
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
byte "CentralReceivedTotal"
int 0
app_global_put
byte "CentralEscrowAddress"
gtxna 0 ApplicationArgs 0
app_global_put
byte "CustomerEscrowAddress"
gtxna 0 ApplicationArgs 1
app_global_put
byte "SharesAssetId"
gtxna 0 ApplicationArgs 2
btoi
app_global_put
byte "FundsAssetId"
gtxna 0 ApplicationArgs 3
btoi
app_global_put
int 1
return
main_l20:
gtxn 0 TypeEnum
int appl
==
assert
int 1
return
"#;