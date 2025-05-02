pub enum JpOperands {
    NZ,
    Z,
    NC,
    C,
}

pub enum InstructionSourceTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    SP,
    HL,
    AF,
    HlAsPointer,
    BcAsPointer,
    DeAsPointer,
    HlPlus,
    HlMinus,
}
