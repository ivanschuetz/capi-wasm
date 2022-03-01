pub const SRC: &str = r#"
#pragma version 4
// int 1

global GroupSize
int 4
==
bnz branch_drain

global GroupSize
int 10
==
return

// For now we just check that it's a payment to the central address
branch_drain:
gtxn 0 TypeEnum
int appl
==

gtxn 1 TypeEnum
int appl
==
&&

gtxn 2 TypeEnum
int axfer
==
&&

gtxn 3 TypeEnum
int axfer
==
&&

"#;
