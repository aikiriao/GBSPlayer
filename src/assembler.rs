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
                        n16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0x02 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::BC,
                        src: SM83Register8::A,
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
                        r16: SM83Register16::BC
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
                        r8: SM83Register8::B
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
                        r8: SM83Register8::B
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
                        n8: rom[2],
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
                        a16: make_u16_from_u8(&rom[1..3]),
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
                        r16: SM83Register16::BC,
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
                        r8: SM83Register8::C
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
                        r8: SM83Register8::C
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
                        n8: rom[2],
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
                        n16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0x12 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::DE,
                        src: SM83Register8::A,
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
                        r16: SM83Register16::DE
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
                        r8: SM83Register8::D
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
                        r8: SM83Register8::D
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
                        n8: rom[2],
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
                        r16: SM83Register16::DE,
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
                        r8: SM83Register8::E
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
                        r8: SM83Register8::E
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
                        n8: rom[2],
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
                        n16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0x22 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HLincrement,
                        src: SM83Register8::A,
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
                        r16: SM83Register16::HL
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
                        r8: SM83Register8::H
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
                        r8: SM83Register8::H
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
                        n8: rom[2],
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
                        r16: SM83Register16::HL,
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
                        r8: SM83Register8::L
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
                        r8: SM83Register8::L
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
                        n8: rom[2],
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
                        n16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0x32 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HLdecrement,
                        src: SM83Register8::A,
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
                        r16: SM83Register16::SP
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
                        n8: rom[2],
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
                        r16: SM83Register16::SP,
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
                        r8: SM83Register8::A
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
                        r8: SM83Register8::A
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
                        n8: rom[2],
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
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::B,
                        src: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x41 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::B,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x42 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::B,
                        src: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x43 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::B,
                        src: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x44 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::B,
                        src: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x45 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::B,
                        src: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x46 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::B,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x47 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::B,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x48 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::C,
                        src: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x49 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::C,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x4A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::C,
                        src: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x4B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::C,
                        src: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x4C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::C,
                        src: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x4D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::C,
                        src: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x4E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::C,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x4F => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::C,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x50 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::D,
                        src: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x51 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::D,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x52 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::D,
                        src: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x53 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::D,
                        src: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x54 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::D,
                        src: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x55 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::D,
                        src: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x56 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::D,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x57 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::D,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x58 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::E,
                        src: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x59 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::E,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x5A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::E,
                        src: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x5B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::E,
                        src: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x5C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::E,
                        src: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x5D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::E,
                        src: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x5E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::E,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x5F => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::E,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x60 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::H,
                        src: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x61 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::H,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x62 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::H,
                        src: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x63 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::H,
                        src: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x64 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::H,
                        src: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x65 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::H,
                        src: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x66 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::H,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x67 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::H,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x68 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::L,
                        src: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x69 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::L,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x6A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::L,
                        src: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x6B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::L,
                        src: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x6C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::L,
                        src: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x6D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::L,
                        src: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x6E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::L,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x6F => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::L,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x70 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HL,
                        src: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x71 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HL,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x72 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HL,
                        src: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x73 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HL,
                        src: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x74 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HL,
                        src: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x75 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HL,
                        src: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x76 => create_opcode_with_length_check!(rom, SM83Opcode::HALT, 1),
        0x77 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR16Indirect {
                        dst: SM83Register16::HL,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x78 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x79 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x7A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x7B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x7C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x7D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x7E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16IndirectToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x7F => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x80 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x81 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x82 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x83 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x84 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x85 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x86 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndR16Indirect {
                        r8: SM83Register8::A,
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x87 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x88 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x89 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x8A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x8B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x8C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x8D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x8E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndR16Indirect {
                        r8: SM83Register8::A,
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x8F => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x90 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x91 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x92 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x93 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x94 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x95 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x96 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndR16Indirect {
                        r8: SM83Register8::A,
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x97 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::A,
                    }
                },
                1
            )
        }
        0x98 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::B,
                    }
                },
                1
            )
        }
        0x99 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::C,
                    }
                },
                1
            )
        }
        0x9A => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::D,
                    }
                },
                1
            )
        }
        0x9B => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::E,
                    }
                },
                1
            )
        }
        0x9C => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::H,
                    }
                },
                1
            )
        }
        0x9D => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::L,
                    }
                },
                1
            )
        }
        0x9E => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndR16Indirect {
                        r8: SM83Register8::A,
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0x9F => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::A,
                    }
                },
                1
            )
        }
        0xA0 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::B,
                    }
                },
                1
            )
        }
        0xA1 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::C,
                    }
                },
                1
            )
        }
        0xA2 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::D,
                    }
                },
                1
            )
        }
        0xA3 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::E,
                    }
                },
                1
            )
        }
        0xA4 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::H,
                    }
                },
                1
            )
        }
        0xA5 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::L,
                    }
                },
                1
            )
        }
        0xA6 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndR16Indirect {
                        r8: SM83Register8::A,
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0xA7 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::A,
                    }
                },
                1
            )
        }
        0xA8 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::B,
                    }
                },
                1
            )
        }
        0xA9 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::C,
                    }
                },
                1
            )
        }
        0xAA => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::D,
                    }
                },
                1
            )
        }
        0xAB => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::E,
                    }
                },
                1
            )
        }
        0xAC => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::H,
                    }
                },
                1
            )
        }
        0xAD => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::L,
                    }
                },
                1
            )
        }
        0xAE => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndR16Indirect {
                        r8: SM83Register8::A,
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0xAF => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::A,
                    }
                },
                1
            )
        }
        0xB0 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::B,
                    }
                },
                1
            )
        }
        0xB1 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::C,
                    }
                },
                1
            )
        }
        0xB2 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::D,
                    }
                },
                1
            )
        }
        0xB3 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::E,
                    }
                },
                1
            )
        }
        0xB4 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::H,
                    }
                },
                1
            )
        }
        0xB5 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::L,
                    }
                },
                1
            )
        }
        0xB6 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndR16Indirect {
                        r8: SM83Register8::A,
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0xB7 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::A,
                    }
                },
                1
            )
        }
        0xB8 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::B,
                    }
                },
                1
            )
        }
        0xB9 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::C,
                    }
                },
                1
            )
        }
        0xBA => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::D,
                    }
                },
                1
            )
        }
        0xBB => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::E,
                    }
                },
                1
            )
        }
        0xBC => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::H,
                    }
                },
                1
            )
        }
        0xBD => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::L,
                    }
                },
                1
            )
        }
        0xBE => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndR16Indirect {
                        r8: SM83Register8::A,
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0xBF => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndR8 {
                        r1: SM83Register8::A,
                        r2: SM83Register8::A,
                    }
                },
                1
            )
        }
        0xC0 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::RET {
                    oprand: SM83Oprand::CC {
                        cc: SM83ConditionCode::NZ,
                    }
                },
                1
            )
        }
        0xC1 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::POP {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::BC,
                    }
                },
                1
            )
        }
        0xC2 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JP {
                    oprand: SM83Oprand::CCAndA16 {
                        cc: SM83ConditionCode::NZ,
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0xC3 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JP {
                    oprand: SM83Oprand::A16 {
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0xC4 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CALL {
                    oprand: SM83Oprand::CCAndA16 {
                        cc: SM83ConditionCode::NZ,
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0xC5 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::PUSH {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::BC,
                    }
                },
                1
            )
        }
        0xC6 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R8AndN8 {
                        r8: SM83Register8::A,
                        n8: rom[1],
                    }
                },
                2
            )
        }
        0xC7 => create_opcode_with_length_check!(rom, SM83Opcode::RST { vec: 0x00 }, 1),
        0xC8 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::RET {
                    oprand: SM83Oprand::CC {
                        cc: SM83ConditionCode::Z,
                    }
                },
                1
            )
        }
        0xC9 => create_opcode_with_length_check!(rom, SM83Opcode::RETNooprand, 1),
        0xCA => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JP {
                    oprand: SM83Oprand::CCAndA16 {
                        cc: SM83ConditionCode::Z,
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        // Prefixedのパース
        0xCB => parse_prefixed_opcode(rom),
        0xCC => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CALL {
                    oprand: SM83Oprand::CCAndA16 {
                        cc: SM83ConditionCode::Z,
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0xCD => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CALL {
                    oprand: SM83Oprand::A16 {
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0xCE => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADC {
                    oprand: SM83Oprand::R8AndN8 {
                        r8: SM83Register8::A,
                        n8: rom[1],
                    }
                },
                2
            )
        }
        0xCF => create_opcode_with_length_check!(rom, SM83Opcode::RST { vec: 0x08 }, 1),
        0xD0 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::RET {
                    oprand: SM83Oprand::CC {
                        cc: SM83ConditionCode::NC,
                    }
                },
                1
            )
        }
        0xD1 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::POP {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::DE,
                    }
                },
                1
            )
        }
        0xD2 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JP {
                    oprand: SM83Oprand::CCAndA16 {
                        cc: SM83ConditionCode::NC,
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        // 0xD3はない
        0xD4 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CALL {
                    oprand: SM83Oprand::CCAndA16 {
                        cc: SM83ConditionCode::NC,
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0xD5 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::PUSH {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::DE,
                    }
                },
                1
            )
        }
        0xD6 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SUB {
                    oprand: SM83Oprand::R8AndN8 {
                        r8: SM83Register8::A,
                        n8: rom[1],
                    }
                },
                2
            )
        }
        0xD7 => create_opcode_with_length_check!(rom, SM83Opcode::RST { vec: 0x10 }, 1),
        0xD8 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::RET {
                    oprand: SM83Oprand::CC {
                        cc: SM83ConditionCode::C,
                    }
                },
                1
            )
        }
        0xD9 => create_opcode_with_length_check!(rom, SM83Opcode::RETI, 1),
        0xDA => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JP {
                    oprand: SM83Oprand::CCAndA16 {
                        cc: SM83ConditionCode::C,
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        // 0xDBはない
        0xDC => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CALL {
                    oprand: SM83Oprand::CCAndA16 {
                        cc: SM83ConditionCode::C,
                        a16: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        // 0xDDはない
        0xDE => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::SBC {
                    oprand: SM83Oprand::R8AndN8 {
                        r8: SM83Register8::A,
                        n8: rom[1],
                    }
                },
                2
            )
        }
        0xDF => create_opcode_with_length_check!(rom, SM83Opcode::RST { vec: 0x18 }, 1),
        0xE0 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LDH {
                    oprand: SM83Oprand::R8ToA8 {
                        dst: rom[1],
                        src: SM83Register8::A,
                    }
                },
                2
            )
        }
        0xE1 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::POP {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0xE2 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LDH {
                    oprand: SM83Oprand::R8ToR8Indirect {
                        dst: SM83Register8::C,
                        src: SM83Register8::A,
                    }
                },
                1
            )
        }
        // 0xE3, 0xE4はない
        0xE5 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::PUSH {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0xE6 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::AND {
                    oprand: SM83Oprand::R8AndN8 {
                        r8: SM83Register8::A,
                        n8: rom[1],
                    }
                },
                2
            )
        }
        0xE7 => create_opcode_with_length_check!(rom, SM83Opcode::RST { vec: 0x20 }, 1),
        0xE8 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::ADD {
                    oprand: SM83Oprand::R16AndE8 {
                        r16: SM83Register16::SP,
                        e8: rom[1] as i8,
                    }
                },
                2
            )
        }
        0xE9 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::JP {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0xEA => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R8ToA16 {
                        dst: make_u16_from_u8(&rom[1..3]),
                        src: SM83Register8::A,
                    }
                },
                3
            )
        }
        // 0xEB, 0xEC, 0xEDはない
        0xEE => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::XOR {
                    oprand: SM83Oprand::R8AndN8 {
                        r8: SM83Register8::A,
                        n8: rom[1],
                    }
                },
                2
            )
        }
        0xEF => create_opcode_with_length_check!(rom, SM83Opcode::RST { vec: 0x28 }, 1),
        0xF0 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LDH {
                    oprand: SM83Oprand::A8ToR8 {
                        dst: SM83Register8::A,
                        src: rom[1],
                    }
                },
                2
            )
        }
        0xF1 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::POP {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::AF,
                    }
                },
                1
            )
        }
        0xF2 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LDH {
                    oprand: SM83Oprand::R8IndirectToR8 {
                        dst: SM83Register8::A,
                        src: SM83Register8::C,
                    }
                },
                1
            )
        }
        0xF3 => create_opcode_with_length_check!(rom, SM83Opcode::DI, 1),
        // 0xE4はない
        0xF5 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::PUSH {
                    oprand: SM83Oprand::R16 {
                        r16: SM83Register16::AF,
                    }
                },
                1
            )
        }
        0xF6 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::OR {
                    oprand: SM83Oprand::R8AndN8 {
                        r8: SM83Register8::A,
                        n8: rom[1],
                    }
                },
                2
            )
        }
        0xF7 => create_opcode_with_length_check!(rom, SM83Opcode::RST { vec: 0x30 }, 1),
        0xF8 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16E8IndirectToR16 {
                        dst: SM83Register16::HL,
                        src_r16: SM83Register16::SP,
                        src_e8: rom[1] as i8,
                    }
                },
                2
            )
        }
        0xF9 => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::R16ToR16 {
                        dst: SM83Register16::SP,
                        src: SM83Register16::HL,
                    }
                },
                1
            )
        }
        0xFA => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::LD {
                    oprand: SM83Oprand::A16ToR8 {
                        dst: SM83Register8::A,
                        src: make_u16_from_u8(&rom[1..3]),
                    }
                },
                3
            )
        }
        0xFB => create_opcode_with_length_check!(rom, SM83Opcode::EI, 1),
        // 0xFC, 0xFDはない
        0xFE => {
            create_opcode_with_length_check!(
                rom,
                SM83Opcode::CP {
                    oprand: SM83Oprand::R8AndN8 {
                        r8: SM83Register8::A,
                        n8: rom[1],
                    }
                },
                2
            )
        }
        0xFF => create_opcode_with_length_check!(rom, SM83Opcode::RST { vec: 0x38 }, 1),
        _ => panic!("Unsupported Instruction!: {:#X}", rom[0]),
    }
}

/// Prefixed（0xCBで始まる）オペコードを解釈
fn parse_prefixed_opcode(rom: &[u8]) -> (SM83Opcode, u16) {
    debug_assert_eq!(
        rom[0], 0xCB,
        "Prefixed opcode is not started by 0xCB but by {}",
        rom[0]
    );

    match rom[1] {
        0x00 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RLC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x01 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RLC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x02 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RLC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x03 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RLC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x04 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RLC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x05 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RLC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x06 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RLC {
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x07 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RLC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x08 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RRC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x09 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RRC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x0A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RRC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x0B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RRC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x0C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RRC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x0D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RRC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x0E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RRC {
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x0F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RRC {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x10 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x11 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x12 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x13 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x14 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x15 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x16 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RL {
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x17 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x18 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RR {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x19 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RR {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x1A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RR {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x1B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RR {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x1C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RR {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x1D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RR {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x1E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RR {
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x1F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RR {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x20 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SLA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x21 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SLA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x22 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SLA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x23 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SLA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x24 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SLA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x25 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SLA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x26 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SLA {
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x27 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SLA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x28 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x29 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x2A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x2B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x2C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x2D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x2E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRA {
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x2F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRA {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x30 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SWAP {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x31 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SWAP {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x32 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SWAP {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x33 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SWAP {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x34 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SWAP {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x35 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SWAP {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x36 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SWAP {
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x37 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SWAP {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x38 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x39 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x3A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x3B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x3C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x3D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x3E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRL {
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x3F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SRL {
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x40 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x41 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x42 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x43 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x44 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x45 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x46 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 0,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x47 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x48 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x49 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x4A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x4B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x4C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x4D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x4E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 1,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x4F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x50 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x51 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x52 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x53 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x54 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x55 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x56 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 2,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x57 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x58 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x59 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x5A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x5B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x5C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x5D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x5E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 3,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x5F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x60 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x61 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x62 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x63 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x64 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x65 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x66 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 4,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x67 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x68 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x69 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x6A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x6B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x6C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x6D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x6E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 5,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x6F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x70 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x71 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x72 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x73 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x74 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x75 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x76 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 6,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x77 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x78 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x79 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x7A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x7B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x7C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x7D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x7E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 7,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x7F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::BIT {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x80 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x81 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x82 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x83 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x84 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x85 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x86 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 0,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x87 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x88 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x89 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x8A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x8B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x8C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x8D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x8E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 1,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x8F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x90 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x91 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x92 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x93 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x94 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x95 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x96 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 2,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x97 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0x98 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0x99 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0x9A => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0x9B => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0x9C => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0x9D => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0x9E => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 3,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0x9F => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xA0 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xA1 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xA2 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xA3 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xA4 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xA5 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xA6 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 4,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xA7 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xA8 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xA9 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xAA => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xAB => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xAC => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xAD => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xAE => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 5,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xAF => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xb0 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xb1 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xb2 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xb3 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xb4 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xb5 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xb6 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 6,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xb7 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xb8 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xb9 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xBA => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xBB => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xBC => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xBD => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xBE => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 7,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xBF => create_opcode_with_length_check!(
            rom,
            SM83Opcode::RES {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xC0 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xC1 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xC2 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xC3 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xC4 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xC5 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xC6 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 0,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xC7 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 0,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xC8 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xC9 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xCA => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xCB => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xCC => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xCD => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xCE => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 1,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xCF => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 1,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xd0 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xd1 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xd2 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xd3 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xd4 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xd5 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xd6 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 2,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xd7 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 2,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xd8 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xd9 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xDA => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xDB => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xDC => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xDD => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xDE => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 3,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xDF => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 3,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xE0 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xE1 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xE2 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xE3 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xE4 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xE5 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xE6 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 4,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xE7 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 4,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xE8 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xE9 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xEA => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xEB => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xEC => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xED => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xEE => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 5,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xEF => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 5,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xF0 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xF1 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xF2 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xF3 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xF4 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xF5 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xF6 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 6,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xF7 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 6,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
        0xF8 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::B,
                }
            },
            2
        ),
        0xF9 => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::C,
                }
            },
            2
        ),
        0xFA => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::D,
                }
            },
            2
        ),
        0xFB => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::E,
                }
            },
            2
        ),
        0xFC => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::H,
                }
            },
            2
        ),
        0xFD => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::L,
                }
            },
            2
        ),
        0xFE => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 7,
                oprand: SM83Oprand::R16Indirect {
                    r16: SM83Register16::HL,
                }
            },
            2
        ),
        0xFF => create_opcode_with_length_check!(
            rom,
            SM83Opcode::SET {
                u3: 7,
                oprand: SM83Oprand::R8 {
                    r8: SM83Register8::A,
                }
            },
            2
        ),
    }
}
