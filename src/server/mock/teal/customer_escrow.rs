pub const SRC: &str = r#"
#pragma version 4
global GroupSize
int 3
==
bnz branch_harvest

// exit TODO difference with jumping to end_contract (less lines?), check more examples
int 0
return

// For now we just check that it's a payment to the central address
branch_harvest:
gtxn 0 TypeEnum // tx 0: app call
int appl
==

gtxn 1 TypeEnum // tx 1: drain
int pay
==
&&

gtxn 1 Fee
int 1000
<=
&&

gtxn 1 RekeyTo
global ZeroAddress
==
&&

gtxn 1 AssetCloseTo
global ZeroAddress
==
&&

gtxn 1 Receiver
addr {central_address}
==
&&

gtxn 2 TypeEnum // tx 2: pays fee
int pay
==
&&

"#;
