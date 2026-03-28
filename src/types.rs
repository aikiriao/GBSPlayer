/// SM83レジスタ群
#[derive(Debug, Clone)]
pub struct SM83Registers {
    /// AF（アキュムレータとフラグ）
    pub af: u16,
    /// BC（汎用レジスタ）
    pub bc: u16,
    /// DE（汎用レジスタ）
    pub de: u16,
    /// HL（汎用+メモリアクセスレジスタ）
    pub hl: u16,
    /// SP（スタックポインタ）
    pub sp: u16,
    /// PC（プログラムカウンタ）
    pub pc: u16,
}

/// SM83のレジスタ種別
#[derive(Debug)]
pub enum SM83Register {
    /// AF
    AF,
    /// BC
    BC,
    /// DE
    DE,
    /// HL
    HL,
    /// SP
    SP,
    /// A
    A,
    /// B
    B,
    /// C
    C,
    /// D
    D,
    /// E
    E,
    /// H
    H,
    /// L
    L,
    /// HL+
    HLincrement,
    /// HL-
    HLdecrement,
}

/// SM83の条件コード
#[derive(Debug)]
pub enum SM83ConditionCode {
    /// Z is set
    Z,
    /// Z is not set
    NZ,
    /// C is set
    C,
    /// C is not set
    NC
}

/// SM83オペランド
#[derive(Debug)]
pub enum SM83Oprand {
    N16ToR16 { constant: u16, dst: SM83Register },
    R16ToA16 { src: SM83Register, address: u16 },
    AToR16Indirect { dst: SM83Register },
    Register { register: SM83Register },
    R16Indirect { r16: SM83Register },
    N8ToR8 { constant: u8, dst: SM83Register },
    N8ToR8Indirect { constant: u8, dst: SM83Register },
    R16ToR16 { src: SM83Register, dst: SM83Register },
    R16IndirectToR8 { src: SM83Register, dst: SM83Register },
}

/// SM83オペコード
#[derive(Debug)]
pub enum SM83Opcode {
    /// NOP
    NOP,
    /// LD (Load)
    LD { oprand: SM83Oprand },
    /// INC (Increment)
    INC { oprand: SM83Oprand },
    /// DEC (Decrement)
    DEC { oprand: SM83Oprand },
    /// RLCA (Rotate Register A Left)
    RLCA,
    /// ADD (Add)
    ADD { oprand: SM83Oprand },
    /// RRCA (Rotate Register A Right)
    RRCA,
    /// STOP (Stop)
    STOP,
    /// JR (Relative Jump)
    JR { oprand: SM83Oprand },
    /// RLA (Rotate Accumulator Left, Through the Carry Flag)
    RLA,
    /// RLA (Rotate Accumulator Right, Through the Carry Flag)
    RRA,
    /// DAA (Decimal Adjust Accumulator)
    DAA,
    /// CPL (ComPLement Accumulator)
    CPL,
    /// SCF (Set Carry Flag)
    SCF,
    /// CCF (Complement Carry Flag)
    CCF,
    /// HALT (Halt)
    HALT,
    /// ADC (Add with Carry Flag)
    ADC { oprand: SM83Oprand },
    /// SUB (Sub)
    SUB { oprand: SM83Oprand },
    /// SBC (Sub with Carry Flag)
    SBC { oprand: SM83Oprand },
    /// AND (Bitwise And)
    AND { oprand: SM83Oprand },
    /// XOR (Bitwise Xor)
    XOR { oprand: SM83Oprand },
    /// OR (Bitwise Or)
    OR { oprand: SM83Oprand },
    /// CP (ComPare)
    CP { oprand: SM83Oprand },
    /// RET (Return from Subroutine)
    RET { oprand: SM83Oprand },
    /// POP (Pop from the Stack)
    POP { oprand: SM83Oprand },
    /// JP (Jump to the Address)
    JP { oprand: SM83Oprand },
    /// CALL (Call Address)
    CALL { oprand: SM83Oprand },
    /// PUSH (Push into the Stack)
    PUSH { oprand: SM83Oprand },
    /// REST (Call Vector Address)
    RST { vec: u8 },
    /// RETI (Return from Subroutine and Enable Interrupts)
    RETI,
    /// LDH (Copy the Register A into the Address)
    LDH { oprand: SM83Oprand },
    /// DI (Disable Interrupts)
    DI,
    /// EI (Enable Interrupts)
    EI,
    // --- Prefixed Opcodes ---
    /// RLC (Rotate Register Left)
    RLC { oprand: SM83Oprand },
    /// RRC (Rotate Register Right)
    RRC { oprand: SM83Oprand },
    /// RR (Rotate Register Right, Through the Carry Flag)
    RR { oprand: SM83Oprand },
    /// SLA (Shift Left Arithmetically)
    SLA { oprand: SM83Oprand },
    /// SRA (Shift Right Arithmetically)
    SRA { oprand: SM83Oprand },
    /// SWAP (Swap the Upper 4 Bits and the Lower 4 Bits)
    SWAP { oprand: SM83Oprand },
    /// SRL (Shift Right Logically)
    SRL { oprand: SM83Oprand },
    /// BIT (Test Bit u3 in Register)
    BIT { u3: u8, oprand: SM83Oprand },
    /// RES (Reset Bit u3 in Register)
    RES { u3: u8, oprand: SM83Oprand },
    /// SET (Set Bit u3 in Register)
    SET { u3: u8, oprand: SM83Oprand },
}

/// メモリ上にあるデータから16bitデータを読みだす
pub fn make_u16_from_u8(data: &[u8]) -> u16 {
    assert_eq!(data.len(), 2);
    ((data[1] as u16) << 8) | data[0] as u16
}
