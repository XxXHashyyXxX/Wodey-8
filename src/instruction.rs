use crate::{ram::Address, register::RegisterCode};

pub enum Instruction {
    Add {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },
    Sub {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },
    Mul {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },
    Div {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },

    And {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },
    Or {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },
    Xor {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },
    Not {
        dest: RegisterCode,
        source: RegisterCode,
    },
    Shl {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },
    Shr {
        dest: RegisterCode,
        source: RegisterCode,
        other: RegisterCode,
    },

    Cmp {
        value: RegisterCode,
    },

    Load {
        dest: RegisterCode,
        addr: Address,
    },
    Loadr {
        dest: RegisterCode,
        source: RegisterCode,
    },
    Store {
        source: RegisterCode,
        addr: Address,
    },
    Storer {
        source: RegisterCode,
        dest: RegisterCode,
    },

    Move {
        dest: RegisterCode,
        source: RegisterCode,
    },

    Cmovz {
        dest: RegisterCode,
        source: RegisterCode,
    },
    Cmovn {
        dest: RegisterCode,
        source: RegisterCode,
    },
    Cmovof {
        dest: RegisterCode,
        source: RegisterCode,
    },

    In {
        port: u8,
    },
    Out {
        port: u8,
    },

    Halt,
}
