pub const SRC: &str = r#"
#pragma version 4
// int 1

global GroupSize
int 3
==
bnz branch_drain

global GroupSize
int 10
==
return

// For now we just check that it's a payment to the central address
branch_drain:
gtxn 0 TypeEnum // tx 0: app call
int appl
==

gtxn 1 TypeEnum // tx 2: pays fee
int pay
==
&&

gtxn 2 TypeEnum // tx 1: drain
int axfer
==
&&

gtxn 2 Fee
int 1000
<=
&&

gtxn 2 RekeyTo
global ZeroAddress
==
&&

gtxn 2 AssetCloseTo
global ZeroAddress
==
&&

gtxn 2 AssetReceiver
addr {central_address}
==
&&

"#;
