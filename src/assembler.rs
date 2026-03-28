use crate::types::*;

/// オペコード長チェック付き命令生成マクロ
macro_rules! create_opcode_with_length_check {
    ($rom:expr, $opcode:expr, $length:expr) => {{
        debug_assert!(
            $rom.len() >= $length,
            "Insufficient instruction length: {}",
            $rom[0]
        );
        ($opcode, $length)
    }};
}

/// ROMからオペコードを解釈
pub fn parse_opcode(rom: &[u8]) -> (SM83Opcode, u16) {
    match rom[0] {
        0x00 => create_opcode_with_length_check!(rom, SM83Opcode::NOP, 1),
        0x01 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N16ToR16 {
                        dst: SM83Register16::BC,
                        constant: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0x02 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::AToR16Indirect {
                        dst: SM83Register16::BC
                    }
                },
                1
            )
        }
        0x03 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R16 {
                        register: SM83Register16::BC
                    }
                },
                1
            )
        }
        0x04 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::B
                    }
                },
                1
            )
        }
        0x05 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::B
                    }
                },
                1
            )
        }
        0x07 => create_opcode_with_length_check!(rom, SM83Opcode::RLCA, 1),
        0x06 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR8 {
                        dst: SM83Register8::B,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        0x08 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16ToA16 {
                        address: make_u16_from_u8(&rom[1..3]),
                        src: SM83Register16::SP,
                    }
                },
                3
            )
        }
        0x09 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R16ToR16 {
                        dst: SM83Register16::HL,
                        src: SM83Register16::BC,
                    }
                },
                1
            )
        }
        0x0A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register16::BC,
                    }
                },
                1
            )
        }
        0x0B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R16 {
                        register: SM83Register16::BC,
                    }
                },
                1
            )
        }
        0x0C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::C
                    }
                },
                1
            )
        }
        0x0D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::C
                    }
                },
                1
            )
        }
        0x0E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR8 {
                        dst: SM83Register8::B,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        0x0F => create_opcode_with_length_check!(rom, SM83Opcode::RRCA, 1),
        0x10 => create_opcode_with_length_check!(rom, SM83Opcode::STOP, 1),
        0x11 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N16ToR16 {
                        dst: SM83Register16::DE,
                        constant: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0x12 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::AToR16Indirect {
                        dst: SM83Register16::DE
                    }
                },
                1
            )
        }
        0x13 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R16 {
                        register: SM83Register16::DE
                    }
                },
                1
            )
        }
        0x14 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::D
                    }
                },
                1
            )
        }
        0x15 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::D
                    }
                },
                1
            )
        }
        0x16 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR8 {
                        dst: SM83Register8::D,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        0x17 => create_opcode_with_length_check!(rom, SM83Opcode::RLA, 1),
        0x18 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JR {
                    oprand: SM83Oprand::E8 { e8: rom[1] as i8 }
                },
                2
            )
        }
        0x19 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R16ToR16 {
                        dst: SM83Register16::HL,
                        src: SM83Register16::DE,
                    }
                },
                1
            )
        }
        0x1A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register16::DE,
                    }
                },
                1
            )
        }
        0x1B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R16 {
                        register: SM83Register16::DE,
                    }
                },
                1
            )
        }
        0x1C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::E
                    }
                },
                1
            )
        }
        0x1D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::E
                    }
                },
                1
            )
        }
        0x1E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR8 {
                        dst: SM83Register8::E,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        0x1F => create_opcode_with_length_check!(rom, SM83Opcode::RRA, 1),
        0x20 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JR {
                    oprand: SM83Oprand::CCAndE8 {
                        cc: SM83ConditionCode::NZ,
                        e8: rom[2] as i8,
                    }
                },
                2
            )
        }
        0x21 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N16ToR16 {
                        dst: SM83Register16::HL,
                        constant: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0x22 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::AToR16Indirect {
                        dst: SM83Register16::HLincrement
                    }
                },
                1
            )
        }
        0x23 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R16 {
                        register: SM83Register16::HL
                    }
                },
                1
            )
        }
        0x24 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::H
                    }
                },
                1
            )
        }
        0x25 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::H
                    }
                },
                1
            )
        }
        0x26 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR8 {
                        dst: SM83Register8::H,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        0x27 => create_opcode_with_length_check!(rom, SM83Opcode::DAA, 1),
        0x28 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JR {
                    oprand: SM83Oprand::CCAndE8 {
                        cc: SM83ConditionCode::Z,
                        e8: rom[1] as i8,
                    }
                },
                2
            )
        }
        0x29 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R16ToR16 {
                        dst: SM83Register16::HL,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x2A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register16::HLincrement,
                    }
                },
                1
            )
        }
        0x2B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R16 {
                        register: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x2C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::L
                    }
                },
                1
            )
        }
        0x2D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::L
                    }
                },
                1
            )
        }
        0x2E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR8 {
                        dst: SM83Register8::L,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        0x2F => create_opcode_with_length_check!(rom, SM83Opcode::CPL, 1),
        0x30 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JR {
                    oprand: SM83Oprand::CCAndE8 {
                        cc: SM83ConditionCode::NC,
                        e8: rom[1] as i8,
                    }
                },
                2
            )
        }
        0x31 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N16ToR16 {
                        dst: SM83Register16::SP,
                        constant: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0x32 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::AToR16Indirect {
                        dst: SM83Register16::HLdecrement
                    }
                },
                1
            )
        }
        0x33 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R16 {
                        register: SM83Register16::SP
                    }
                },
                1
            )
        }
        0x34 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R16Indirect {
                        r16: SM83Register16::HL
                    }
                },
                1
            )
        }
        0x35 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R16Indirect {
                        r16: SM83Register16::HL
                    }
                },
                1
            )
        }
        0x36 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR16Indirect {
                        dst: SM83Register16::HL,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        0x37 => create_opcode_with_length_check!(rom, SM83Opcode::SCF, 1),
        0x38 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JR {
                    oprand: SM83Oprand::CCAndE8 {
                        cc: SM83ConditionCode::C,
                        e8: rom[1] as i8,
                    }
                },
                2
            )
        }
        0x39 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R16ToR16 {
                        dst: SM83Register16::HL,
                        src: SM83Register16::SP,
                    }
                },
                1
            )
        }
        0x3A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register16::HLdecrement,
                    }
                },
                1
            )
        }
        0x3B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R16 {
                        register: SM83Register16::SP,
                    }
                },
                1
            )
        }
        0x3C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::INC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::A
                    }
                },
                1
            )
        }
        0x3D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::DEC {
                    oprand: SM83Oprand::R8 {
                        register: SM83Register8::A
                    }
                },
                1
            )
        }
        0x3E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR8 {
                        dst: SM83Register8::A,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        0x3F => create_opcode_with_length_check!(rom, SM83Opcode::CCF, 1),
        0x40 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::N8ToR8 {
                        dst: SM83Register8::A,
                        constant: rom[2],
                    }
                },
                2
            )
        }
        _ => panic!("Unsupported Instruction!: {:#X}", rom[0]),
    }
}
