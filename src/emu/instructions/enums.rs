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
    HlAsPointer,
    BcAsPointer,
    DeAsPointer,
    HlPlus,
    HlMinus,
}
