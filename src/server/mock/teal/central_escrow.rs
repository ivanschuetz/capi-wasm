pub const SRC: &str = r#"
#pragma version 4

// TODO verify app calls etc.

global GroupSize
int 3
==

bnz branch_harvest

global GroupSize
int 2
==

bnz branch_withdraw

int 0
return

branch_harvest:

int 1
return

branch_withdraw:

gtxn 0 TypeEnum // withdrawal
int pay
==

gtxn 1 TypeEnum // pay withdrawal fee
int pay
==
&&

gtxn 0 Receiver
addr {project_creator_address}
==
&&
"#;
