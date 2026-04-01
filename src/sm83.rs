use crate::assembler::*;
use crate::types::*;
use log::{trace, warn};

/// ゼロフラグ
const FLAG_Z: u8 = 1 << 7;
/// ネガティブ(BCD)フラグ
const FLAG_N: u8 = 1 << 6;
/// ハーフキャリーフラグ
const FLAG_H: u8 = 1 << 5;
/// キャリーフラグ
const FLAG_C: u8 = 1 << 4;

const ROM_BANK0_START_ADDRESS: usize = 0x0000;
const ROM_BANK1_START_ADDRESS: usize = 0x4000;
const VRAM_START_ADDRESS: usize = 0x8000;
const EXTERNAL_RAM_START_ADDRESS: usize = 0xA000;
/// Work RAM
const WRAM_BANK0_START_ADDRESS: usize = 0xC000;
/// Work RAM Bank 1-7
const WRAM_BANK1_START_ADDRESS: usize = 0xD000;
///
const ECHO_RAM_START_ADDRESS: usize = 0xE000;
/// Object Attribute Memory (OAM)
const OAM_START_ADDRESS: usize = 0xFE00;
const NOT_USABLE_START_ADDRESS: usize = 0xFEA0;
/// ハードウェアレジスタの開始アドレス
const HWREG_START_ADDRESS: usize = 0xFF00;
/// High RAM (HRAM)
const HRAM_START_ADDRESS: usize = 0xFF80;
/// Intterupt Enable Register
const IE_START_ADDRESS: usize = 0xFFFF;

// ハードウェアレジスタアドレス
/// ジョイパッド
pub const HWREG_P1_JOYPAD: usize = 0xFF00;
/// シリアル通信データ
pub const HWREG_SB_SERIAL_TRANSFER_DATA: usize = 0xFF01;
/// シリアル通信制御
pub const HWREG_SC_SERIAL_TRANSFER_CONTROL: usize = 0xFF02;
/// 単調増加タイマー
pub const HWREG_DIV_REGISTER: usize = 0xFF04;
/// タイマーカウンタ
pub const HWREG_TIMA_TIMER_COUNTER: usize = 0xFF05;
/// タイマー剰余
pub const HWREG_TMA_TIMER_MODULO: usize = 0xFF06;
/// タイマー制御
pub const HWREG_TAC_TIMER_CONTROL: usize = 0xFF07;
/// 割り込みフラグ
pub const HWREG_IF_INTERRUPT_FLAG: usize = 0xFF0F;
/// チャンネル1スイープ
pub const HWREG_NR10_CHANNEL1_SWEEP: usize = 0xFF10;
/// チャンネル1長さタイマー・デューティ比
pub const HWREG_NR11_CHANNEL1_LENGTH_TIMER_DURY_CYCLE: usize = 0xFF11;
/// チャンネル1ボリューム・エンベロープ
pub const HWREG_NR12_CHANNEL1_VOLUME_ENVELOPE: usize = 0xFF12;
/// チャンネル1周期下位ビット
pub const HWREG_NR13_CHANNEL1_PERIOD_LOW: usize = 0xFF13;
/// チャンネル1周期上位ビット・制御ビット
pub const HWREG_NR14_CHANNEL1_PERIOD_HIGH_CONTROL: usize = 0xFF14;
/// チャンネル2長さタイマー・デューティ比
pub const HWREG_NR21_CHANNEL2_LENGTH_TIMER_DURY_CYCLE: usize = 0xFF16;
/// チャンネル2ボリューム・エンベロープ
pub const HWREG_NR22_CHANNEL2_VOLUME_ENVELOPE: usize = 0xFF17;
/// チャンネル2周期下位ビット
pub const HWREG_NR23_CHANNEL2_PERIOD_LOW: usize = 0xFF18;
/// チャンネル2周期上位ビット・制御ビット
pub const HWREG_NR24_CHANNEL2_PERIOD_HIGH_CONTROL: usize = 0xFF19;
/// チャンネル3DAC有効フラグ
pub const HWREG_NR30_CHANNEL3_DAC_ENABLE: usize = 0xFF1A;
/// チャンネル3長さタイマー
pub const HWREG_NR31_CHANNEL3_LENGTH_TIMER: usize = 0xFF1B;
/// チャンネル3出力レベル
pub const HWREG_NR32_CHANNEL3_OUTPUT_LEVEL: usize = 0xFF1C;
/// チャンネル3周期下位ビット
pub const HWREG_NR33_CHANNEL3_PERIOD_LOW: usize = 0xFF1D;
/// チャンネル3周期上位ビット・制御ビット
pub const HWREG_NR33_CHANNEL3_PERIOD_HIGH_CONTROL: usize = 0xFF1E;
/// チャンネル4長さタイマー
pub const HWREG_NR41_CHANNEL4_LENGTH_TIMER: usize = 0xFF20;
/// チャンネル4ボリューム・エンベロープ
pub const HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE: usize = 0xFF21;
/// チャンネル4周波数・ランダム度
pub const HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS: usize = 0xFF22;
/// チャンネル4制御
pub const HWREG_NR44_CHANNEL4_CONTROL: usize = 0xFF23;
/// マスターボリューム・外部音声出力のパン
pub const HWREG_NR51_MASTER_VOLUME_VIN_PANNING: usize = 0xFF24;
/// 各チャンネルのパン
pub const HWREG_NR51_SOUND_PANNING: usize = 0xFF25;
/// 全体と各チャンネルのオーディオON/OFF
pub const HWREG_NR52_AUDIO_MASTER_CONTROL: usize = 0xFF26;
/// チャンネル3の波形RAM
pub const HWREG_CHANNEL3_WAVE_PATTERN_RAM_START: usize = 0xFF30;
/// LCD制御
pub const HWREG_LCDC_LCD_CONTROL: usize = 0xFF40;
/// LCDステータス
pub const HWREG_STAT_LCD_STATUS: usize = 0xFF41;
/// ビューポートのY座標
pub const HWREG_SCY_VIEWPORT_Y: usize = 0xFF42;
/// ビューポートのX座標
pub const HWREG_SCX_VIEWPORT_X: usize = 0xFF43;
/// LCDのY座標
pub const HWREG_LY_LCD_Y_COORDINATE: usize = 0xFF44;
/// LYとの比較
pub const HWREG_LYC_LY_COMPARE: usize = 0xFF45;
/// ROM/RAMからOAM(Object Attribute Memory)へのDMA転送アドレス
pub const HWREG_DMA_SOURCE_ADDRESS_START: usize = 0xFF46;
/// BGパレットデータ
pub const HWREG_BGP_BG_PALETTE_DATA: usize = 0xFF47;
/// パレット0のカラーインデックス
pub const HWREG_OBP0_OBJ_PALETTE0: usize = 0xFF48;
/// パレット1のカラーインデックス
pub const HWREG_OBP1_OBJ_PALETTE1: usize = 0xFF49;
/// ウィンドウのY座標
pub const HWREG_WY_WINDOW_Y: usize = 0xFF4A;
/// ウィンドウのX座標+7
pub const HWREG_WX_WINDOW_X_PLUS_7: usize = 0xFF4B;
/// CPUモードセレクト
pub const HWREG_KEY0_CPU_MODE_SELECT: usize = 0xFF4C;
/// CGBの倍速モード/通常速モードに備える
pub const HWREG_KEY1_PREPARE_SPEED_SWITCH: usize = 0xFF4D;
/// VRAMバンク
pub const HWREG_VBK_VRAM_BANK: usize = 0xFF4F;
/// ブートROMのマッピング制御
pub const HWREG_BANK_BOOTROM_MAPPING_CONTROL: usize = 0xFF50;
/// VRAM DMAのソースアドレス上位
pub const HWREG_HDMA1_VRAMDMA_SOURCE_HIGH: usize = 0xFF51;
/// VRAM DMAのソースアドレス下位
pub const HWREG_HDMA2_VRAMDMA_SOURCE_LOW: usize = 0xFF52;
/// VRAM DMAのターゲットアドレス上位
pub const HWREG_HDMA3_VRAMDMA_DESTINATION_HIGH: usize = 0xFF53;
/// VRAM DMAのターゲットアドレス下位
pub const HWREG_HDMA4_VRAMDMA_DESTINATION_LOW: usize = 0xFF54;
/// VRAM DMAの長さ/モード/開始
pub const HWREG_HDMA5_LENGTH_MODE_START: usize = 0xFF55;
/// 赤外線通信ポート
pub const HWREG_RP_INFRATED_COMMUNICATIONS_PORT: usize = 0xFF56;
/// 背景カラーパレット仕様・インデックス
pub const HWREG_BCPS_BACKGROUND_COLOR_PALETTE_SPECIFICATION: usize = 0xFF68;
/// 背景カラーパレットデータ
pub const HWREG_BCPS_BACKGROUND_COLOR_PALETTE_DATA: usize = 0xFF69;
/// OBJカラーパレット仕様・インデックス
pub const HWREG_OCPS_OBJ_COLOR_PALETTE_SPECIFICATION: usize = 0xFF6A;
/// OBJカラーパレットデータ
pub const HWREG_OCPS_OBJ_COLOR_PALETTE_DATA: usize = 0xFF6B;
/// オブジェクト優先度
pub const HWREG_OPRI_OBJECT_PRIOROTY_MODE: usize = 0xFF6C;
/// WRAMバンク
pub const HWREG_SVBK_WRAM_BANK: usize = 0xFF70;
/// チャンネル1,2のオーディオデジタル出力
pub const HWREG_PCM12_AUDIO_DIGITAL_OUTPUTS_12: usize = 0xFF76;
/// チャンネル3,4のオーディオデジタル出力
pub const HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34: usize = 0xFF77;
/// 割り込み有効フラグ
pub const HWREG_IE_INTTERUPT_ENABLE: usize = 0xFFFF;

/// SM83エミュレータ
pub struct SM83 {
    /// レジスタ
    pub regs: SM83Registers,
    /// 64KBメモリ領域
    pub mem: [u8; 65536],
    /// IMEフラグ
    pub ime_flag: bool,
}

impl SM83 {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            regs: SM83Registers {
                a: 0,
                f: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
                sp: 0,
                pc: 0,
            },
            mem: [0; 65536],
            ime_flag: false,
        }
    }

    /// フラグが立っているか検査
    fn test_flag(&self, flag: u8) -> bool {
        (self.regs.f & flag) != 0
    }

    /// 条件conditionに依存し、フラグのset/resetを実行
    fn set_flag(&mut self, flag: u8, condition: bool) {
        self.regs.f = if condition {
            self.regs.f | flag
        } else {
            self.regs.f & !flag
        };
    }

    /// 条件コードの成立を判定
    fn test_condition_code(&self, cc: &SM83ConditionCode) -> bool {
        match cc {
            SM83ConditionCode::Z => self.test_flag(FLAG_Z),
            SM83ConditionCode::NZ => !self.test_flag(FLAG_Z),
            SM83ConditionCode::C => self.test_flag(FLAG_C),
            SM83ConditionCode::NC => !self.test_flag(FLAG_C),
        }
    }

    /// スタックにデータをPUSH
    fn push_stack(&mut self, value: u8) {
        self.write_mem_u8(self.regs.sp as usize, value);
        self.regs.sp = self.regs.sp.wrapping_sub(1);
    }

    /// スタックからデータをPOP
    fn pop_stack(&mut self) -> u8 {
        self.regs.sp = self.regs.sp.wrapping_add(1);
        self.read_mem_u8(self.regs.sp as usize)
    }

    /// フラグリセット
    pub fn reset_flags(&mut self) {
        self.set_flag(FLAG_Z, true);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_H, true);
        self.set_flag(FLAG_C, true);
        // TODO: 割り込みフラグを無効に
    }

    /// レジスタリセット
    pub fn reset_registers(&mut self) {
        self.regs.a = 0;
        // フラグは維持
        self.regs.b = 0;
        self.regs.c = 0;
        self.regs.d = 0;
        self.regs.e = 0;
        self.regs.h = 0;
        self.regs.l = 0;
        self.regs.sp = 0xFFFE;
        self.regs.pc = 0x0100;
    }

    /// RAM領域のクリア
    pub fn clear_ram(&mut self) {
        self.mem[EXTERNAL_RAM_START_ADDRESS..(EXTERNAL_RAM_START_ADDRESS + 0x2000)].fill(0);
        self.mem[WRAM_BANK0_START_ADDRESS..(WRAM_BANK0_START_ADDRESS + 0x1000)].fill(0);
        self.mem[WRAM_BANK1_START_ADDRESS..(WRAM_BANK1_START_ADDRESS + 0x1000)].fill(0);
    }

    /// ステップ実行
    pub fn execute_step(&mut self) -> (SM83Opcode, u8) {
        let (opcode, len) = parse_opcode(&self.mem[(self.regs.pc as usize)..]);
        trace!(
            "{:#06X}: {:02X?} {:X?} {:X?}",
            self.regs.pc,
            &self.mem[(self.regs.pc as usize)..((self.regs.pc + len) as usize)],
            opcode,
            self.regs
        );
        self.regs.pc += len;
        let cycle = self.execute_opcode(&opcode);
        (opcode, cycle)
    }

    /// 8bitメモリ書き込み
    pub fn write_mem_u8(&mut self, address: usize, value: u8) {
        // ハードウェアレジスタへの書き込み
        if (address >= HWREG_P1_JOYPAD) && (address <= HWREG_IE_INTTERUPT_ENABLE) {
            match address {
                _ => todo!("unimplemented!"),
            }
        }
        self.mem[address] = value;
        trace!("W: 0x{:04X} <- {:02X}", address, value);
    }

    /// 8bitメモリ読み込み
    pub fn read_mem_u8(&mut self, address: usize) -> u8 {
        trace!("R: 0x{:04X} -> {:02X}", address, self.mem[address]);
        // ハードウェアレジスタからの読み込み
        if (address >= HWREG_P1_JOYPAD) && (address <= HWREG_IE_INTTERUPT_ENABLE) {
            match address {
                _ => todo!("unimplemented!"),
            }
        }
        self.mem[address]
    }

    /// 16bitメモリ書き込み
    fn write_mem_u16(&mut self, address: usize, value: u16) {
        trace!("W16: 0x{:04X} <- {:04X}", address, value,);
        self.mem[address + 0] = ((value >> 0) & 0xFF) as u8;
        self.mem[address + 1] = ((value >> 8) & 0xFF) as u8;
    }

    /// 16bitメモリ読み込み
    fn read_mem_u16(&self, address: usize) -> u16 {
        trace!(
            "R16: 0x{:04X} -> {:04X}",
            address,
            ((self.mem[address + 1] as u16) << 8) | self.mem[address] as u16
        );
        ((self.mem[address + 1] as u16) << 8) | self.mem[address] as u16
    }

    /// オペコード実行
    fn execute_opcode(&mut self, opcode: &SM83Opcode) -> u8 {
        match opcode {
            SM83Opcode::NOP => {
                // 何もしない
                1
            }
            SM83Opcode::LD { oprand } => self.execute_ld(oprand),
            SM83Opcode::INC { oprand } => {
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let value = self.get_r8(r8);
                        self.set_r8(r8, value.wrapping_add(1));
                        cycle = 1;
                    }
                    SM83Oprand::R16 { r16 } => {
                        let value = self.get_r16(r16);
                        self.set_r16(r16, value.wrapping_add(1));
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        self.write_mem_u8(address, value.wrapping_add(1));
                        cycle = 3;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                cycle
            }
            SM83Opcode::DEC { oprand } => {
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let value = self.get_r8(r8);
                        self.set_r8(r8, value.wrapping_sub(1));
                        cycle = 1;
                    }
                    SM83Oprand::R16 { r16 } => {
                        let value = self.get_r16(r16);
                        self.set_r16(r16, value.wrapping_sub(1));
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        self.write_mem_u8(address, value.wrapping_sub(1));
                        cycle = 3;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                cycle
            }
            SM83Opcode::RLCA => {
                let msb = self.regs.a & 0x80;
                self.regs.a = (self.regs.a << 1) | (msb >> 7);
                self.set_flag(FLAG_Z, false);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, msb != 0);
                1
            }
            // TODO: ADD
            SM83Opcode::RRCA => {
                let lsb = self.regs.a & 0x01;
                self.regs.a = (self.regs.a >> 1) | (lsb << 7);
                self.set_flag(FLAG_Z, false);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, lsb != 0);
                1
            }
            SM83Opcode::RLA => {
                let msb = self.regs.a & 0x80;
                let lsb = if self.test_flag(FLAG_C) { 0x01 } else { 0x00 };
                self.regs.a = (self.regs.a << 1) | lsb;
                self.set_flag(FLAG_C, msb != 0);
                1
            }
            SM83Opcode::JR { oprand } => match oprand {
                SM83Oprand::E8 { e8 } => {
                    self.regs.pc = (self.regs.pc as i32 + *e8 as i32) as u16;
                    3
                }
                SM83Oprand::CCAndE8 { cc, e8 } => {
                    if self.test_condition_code(cc) {
                        self.regs.pc = (self.regs.pc as i32 + *e8 as i32) as u16;
                        3
                    } else {
                        2
                    }
                }
                _ => unreachable!("Invalid oprand!"),
            },
            SM83Opcode::RRA => {
                let lsb = self.regs.a & 0x01;
                let msb = if self.test_flag(FLAG_C) { 0x80 } else { 0x00 };
                self.regs.a = (self.regs.a >> 1) | msb;
                self.set_flag(FLAG_C, lsb != 0);
                1
            }
            // TODO: DAA
            SM83Opcode::CPL => {
                self.regs.a = !self.regs.a;
                self.set_flag(FLAG_N, true);
                self.set_flag(FLAG_H, true);
                1
            }
            SM83Opcode::SCF => {
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, true);
                1
            }
            SM83Opcode::CCF => {
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, !self.test_flag(FLAG_C));
                1
            }
            SM83Opcode::HALT => {
                warn!("execute HALT instruction");
                // TODO: 低電力モードに移行？
                1
            }
            // TODO: ADC
            // TODO: SUB
            // TODO: SBC
            SM83Opcode::AND { oprand } => {
                let cycle;
                let ret;
                match oprand {
                    SM83Oprand::R8AndR8 { r1, r2 } => {
                        ret = self.get_r8(r1) & self.get_r8(r2);
                        cycle = 1;
                    }
                    SM83Oprand::R8AndR16Indirect { r8, r16 } => {
                        let address = self.get_r16(r16);
                        ret = self.get_r8(r8) & self.read_mem_u8(address as usize);
                        cycle = 2;
                    }
                    SM83Oprand::R8AndN8 { r8, n8 } => {
                        ret = self.get_r8(r8) & *n8;
                        cycle = 2;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.regs.a = ret;
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, true);
                self.set_flag(FLAG_C, false);
                cycle
            }
            SM83Opcode::XOR { oprand } => {
                let cycle;
                let ret;
                match oprand {
                    SM83Oprand::R8AndR8 { r1, r2 } => {
                        ret = self.get_r8(r1) ^ self.get_r8(r2);
                        cycle = 1;
                    }
                    SM83Oprand::R8AndR16Indirect { r8, r16 } => {
                        let address = self.get_r16(r16);
                        ret = self.get_r8(r8) ^ self.read_mem_u8(address as usize);
                        cycle = 2;
                    }
                    SM83Oprand::R8AndN8 { r8, n8 } => {
                        ret = self.get_r8(r8) ^ *n8;
                        cycle = 2;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.regs.a = ret;
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, false);
                cycle
            }
            SM83Opcode::OR { oprand } => {
                let cycle;
                let ret;
                match oprand {
                    SM83Oprand::R8AndR8 { r1, r2 } => {
                        ret = self.get_r8(r1) | self.get_r8(r2);
                        cycle = 1;
                    }
                    SM83Oprand::R8AndR16Indirect { r8, r16 } => {
                        let address = self.get_r16(r16);
                        ret = self.get_r8(r8) | self.read_mem_u8(address as usize);
                        cycle = 2;
                    }
                    SM83Oprand::R8AndN8 { r8, n8 } => {
                        ret = self.get_r8(r8) | *n8;
                        cycle = 2;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.regs.a = ret;
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, false);
                cycle
            }
            // TODO: CP
            SM83Opcode::RETNooprand => {
                let low = self.pop_stack();
                let high = self.pop_stack();
                self.regs.pc = ((high as u16) << 8) | (low as u16);
                4
            }
            SM83Opcode::RET { oprand } => match oprand {
                SM83Oprand::CC { cc } => {
                    if self.test_condition_code(cc) {
                        let low = self.pop_stack();
                        let high = self.pop_stack();
                        self.regs.pc = ((high as u16) << 8) | (low as u16);
                        5
                    } else {
                        2
                    }
                }
                _ => unreachable!("Invalid oprand!"),
            },
            SM83Opcode::POP { oprand } => {
                match oprand {
                    SM83Oprand::R16 { r16 } => {
                        let low = self.pop_stack();
                        let high = self.pop_stack();
                        self.set_r16(r16, ((high as u16) << 8) | (low as u16));
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                3
            }
            SM83Opcode::PUSH { oprand } => {
                match oprand {
                    SM83Oprand::R16 { r16 } => {
                        let value = self.get_r16(r16);
                        self.push_stack(((value >> 8) & 0xFF) as u8);
                        self.push_stack(((value >> 0) & 0xFF) as u8);
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                4
            }
            SM83Opcode::RST { vec } => {
                self.push_stack(((self.regs.pc >> 8) & 0xFF) as u8);
                self.push_stack(((self.regs.pc >> 0) & 0xFF) as u8);
                self.regs.pc = *vec as u16;
                4
            }
            SM83Opcode::JP { oprand } => match oprand {
                SM83Oprand::CCAndA16 { cc, a16 } => {
                    if self.test_condition_code(cc) {
                        self.regs.pc = *a16;
                        4
                    } else {
                        3
                    }
                }
                SM83Oprand::A16 { a16 } => {
                    self.regs.pc = *a16;
                    4
                }
                _ => unreachable!("Invalid oprand!"),
            },
            SM83Opcode::CALL { oprand } => match oprand {
                SM83Oprand::CCAndA16 { cc, a16 } => {
                    if self.test_condition_code(cc) {
                        self.push_stack(((self.regs.pc >> 8) & 0xFF) as u8);
                        self.push_stack(((self.regs.pc >> 0) & 0xFF) as u8);
                        self.regs.pc = *a16;
                        6
                    } else {
                        3
                    }
                }
                SM83Oprand::A16 { a16 } => {
                    self.push_stack(((self.regs.pc >> 8) & 0xFF) as u8);
                    self.push_stack(((self.regs.pc >> 0) & 0xFF) as u8);
                    6
                }
                _ => unreachable!("Invalid oprand!"),
            },
            SM83Opcode::LDH { oprand } => match oprand {
                SM83Oprand::R8ToA8 { dst, src } => {
                    let value = self.get_r8(src);
                    let address = HWREG_START_ADDRESS as usize + *dst as usize;
                    self.write_mem_u8(address, value);
                    3
                }
                SM83Oprand::R8IndirectToR8 { dst, src } => {
                    let address = HWREG_START_ADDRESS as usize + self.get_r8(src) as usize;
                    let value = self.read_mem_u8(address);
                    self.set_r8(dst, value);
                    2
                }
                _ => unreachable!("Invalid oprand!"),
            },
            SM83Opcode::DI => {
                self.ime_flag = false;
                1
            }
            SM83Opcode::EI => {
                // NOTE: この命令実行後に有効になる
                self.ime_flag = true;
                1
            }
            SM83Opcode::RLC { oprand } => {
                let ret;
                let msb;
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        msb = reg & 0x80;
                        ret = (reg << 1) | (msb >> 7);
                        self.set_r8(r8, ret);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        msb = value & 0x80;
                        ret = (value << 1) | (msb >> 7);
                        self.write_mem_u8(address, ret);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, msb != 0);
                cycle
            }
            SM83Opcode::RRC { oprand } => {
                let ret;
                let lsb;
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        lsb = reg & 0x01;
                        ret = (reg >> 1) | (lsb << 7);
                        self.set_r8(r8, ret);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        lsb = value & 0x01;
                        ret = (value >> 1) | (lsb << 7);
                        self.write_mem_u8(address, ret);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, lsb != 0);
                cycle
            }
            SM83Opcode::RL { oprand } => {
                let ret;
                let msb;
                let cycle;
                let lsb = if self.test_flag(FLAG_C) { 0x01 } else { 0x00 };
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        msb = reg & 0x80;
                        ret = (reg << 1) | lsb;
                        self.set_r8(r8, ret);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        msb = value & 0x80;
                        ret = (value << 1) | lsb;
                        self.write_mem_u8(address, ret);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, msb != 0);
                cycle
            }
            SM83Opcode::RR { oprand } => {
                let ret;
                let lsb;
                let cycle;
                let msb = if self.test_flag(FLAG_C) { 0x80 } else { 0x00 };
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        lsb = reg & 0x01;
                        ret = (reg >> 1) | msb;
                        self.set_r8(r8, ret);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        lsb = value & 0x01;
                        ret = (value >> 1) | msb;
                        self.write_mem_u8(address, ret);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, lsb != 0);
                cycle
            }
            SM83Opcode::SLA { oprand } => {
                let ret;
                let msb;
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        msb = reg & 0x80;
                        ret = reg << 1;
                        self.set_r8(r8, ret);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        msb = value & 0x80;
                        ret = value << 1;
                        self.write_mem_u8(address, ret);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, msb != 0);
                cycle
            }
            SM83Opcode::SRA { oprand } => {
                let ret;
                let lsb;
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        let msb = reg & 0x80;
                        lsb = reg & 0x01;
                        ret = msb | (reg >> 1);
                        self.set_r8(r8, ret);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        let msb = value & 0x80;
                        lsb = value & 0x01;
                        ret = msb | (value >> 1);
                        self.write_mem_u8(address, ret);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, lsb != 0);
                cycle
            }
            SM83Opcode::SWAP { oprand } => {
                let ret;
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        ret = (reg >> 4) | (reg << 4);
                        self.set_r8(r8, ret);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        ret = (value >> 4) | (value << 4);
                        self.write_mem_u8(address, ret);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, false);
                cycle
            }
            SM83Opcode::SRL { oprand } => {
                let ret;
                let lsb;
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        lsb = reg & 0x01;
                        ret = reg >> 1;
                        self.set_r8(r8, ret);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        lsb = value & 0x01;
                        ret = value >> 1;
                        self.write_mem_u8(address, ret);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, lsb != 0);
                cycle
            }
            _ => panic!("Invalid opcode: {:?}", opcode),
        }
    }

    /// 8bitレジスタ値の取得
    fn get_r8(&self, r8: &SM83Register8) -> u8 {
        match r8 {
            SM83Register8::A => self.regs.a,
            SM83Register8::B => self.regs.b,
            SM83Register8::C => self.regs.c,
            SM83Register8::D => self.regs.d,
            SM83Register8::E => self.regs.e,
            SM83Register8::H => self.regs.h,
            SM83Register8::L => self.regs.l,
        }
    }

    /// 8bitレジスタ値の設定
    fn set_r8(&mut self, r8: &SM83Register8, value: u8) {
        match r8 {
            SM83Register8::A => {
                self.regs.a = value;
            }
            SM83Register8::B => {
                self.regs.b = value;
            }
            SM83Register8::C => {
                self.regs.c = value;
            }
            SM83Register8::D => {
                self.regs.d = value;
            }
            SM83Register8::E => {
                self.regs.e = value;
            }
            SM83Register8::H => {
                self.regs.h = value;
            }
            SM83Register8::L => {
                self.regs.l = value;
            }
        }
    }

    /// 16bitレジスタ値の取得
    fn get_r16(&self, r16: &SM83Register16) -> u16 {
        match r16 {
            SM83Register16::AF => ((self.regs.a as u16) << 8) | (self.regs.f as u16),
            SM83Register16::BC => ((self.regs.b as u16) << 8) | (self.regs.c as u16),
            SM83Register16::DE => ((self.regs.d as u16) << 8) | (self.regs.e as u16),
            SM83Register16::HL | SM83Register16::HLincrement | SM83Register16::HLdecrement => {
                ((self.regs.h as u16) << 8) | (self.regs.l as u16)
            }
            SM83Register16::SP => self.regs.sp,
        }
    }

    /// 16bitレジスタ値の設定
    fn set_r16(&mut self, r16: &SM83Register16, value: u16) {
        let high = ((value >> 8) & 0xFF) as u8;
        let low = ((value >> 0) & 0xFF) as u8;
        match r16 {
            SM83Register16::AF => {
                self.regs.a = high;
                self.regs.f = low;
            }
            SM83Register16::BC => {
                self.regs.b = high;
                self.regs.c = low;
            }
            SM83Register16::DE => {
                self.regs.d = high;
                self.regs.e = low;
            }
            SM83Register16::HL | SM83Register16::HLincrement | SM83Register16::HLdecrement => {
                self.regs.h = high;
                self.regs.l = low;
            }
            SM83Register16::SP => {
                self.regs.sp = value;
            }
        }
    }

    /// LD命令の実行
    fn execute_ld(&mut self, oprand: &SM83Oprand) -> u8 {
        let cycle;

        match oprand {
            SM83Oprand::N16ToR16 { dst, n16 } => {
                self.set_r16(dst, *n16);
                cycle = 3;
            }
            SM83Oprand::R8ToR16Indirect { dst, src } => {
                let address = self.get_r16(dst);
                let value = self.get_r8(src);
                self.write_mem_u8(address as usize, value);
                match dst {
                    SM83Register16::HLincrement => {
                        let address = address.wrapping_add(1);
                        self.set_r16(dst, address);
                    }
                    SM83Register16::HLdecrement => {
                        let address = address.wrapping_sub(1);
                        self.set_r16(dst, address);
                    }
                    _ => {}
                }
                cycle = 2;
            }
            SM83Oprand::N8ToR8 { dst, n8 } => {
                self.set_r8(dst, *n8);
                cycle = 2;
            }
            SM83Oprand::R16IndirectToR8 { dst, src } => {
                let address = self.get_r16(src);
                let value = self.read_mem_u8(address as usize);
                self.set_r8(dst, value);
                match src {
                    SM83Register16::HLincrement => {
                        self.set_r16(src, address.wrapping_add(1));
                    }
                    SM83Register16::HLdecrement => {
                        self.set_r16(src, address.wrapping_sub(1));
                    }
                    _ => {}
                }
                cycle = 2;
            }
            SM83Oprand::N8ToR16Indirect { dst, n8 } => {
                let address = self.get_r16(dst);
                self.write_mem_u8(address as usize, *n8);
                cycle = 3;
            }
            SM83Oprand::R8ToR8 { dst, src } => {
                let value = self.get_r8(src);
                self.set_r8(dst, value);
                cycle = 1;
            }
            SM83Oprand::R8ToA16 { dst, src } => {
                let value = self.get_r8(src);
                self.write_mem_u8(*dst as usize, value);
                cycle = 3;
            }
            SM83Oprand::A16ToR8 { dst, src } => {
                let value = self.read_mem_u8(*src as usize);
                self.set_r8(dst, value);
                cycle = 4;
            }
            SM83Oprand::R16ToA16 { a16, src } => {
                let value = self.get_r16(src);
                self.write_mem_u16(*a16 as usize, value);
                cycle = 5;
            }
            SM83Oprand::R16E8IndirectToR16 {
                dst,
                src_r16,
                src_e8,
            } => {
                let offset = self.get_r16(src_r16);
                let value = (offset as i32 + *src_e8 as i32) as u16;
                self.set_r16(dst, value);
                self.set_flag(FLAG_Z, false);
                self.set_flag(FLAG_N, false);
                // FIXME: 加算結果に依存してH,Cをセット
                // https://stackoverflow.com/questions/5159603/gbz80-how-does-ld-hl-spe-affect-h-and-c-flags
                // self.set_flag(FLAG_H, (value & 0x08) != 0);
                // self.set_flag(FLAG_C, (value & 0x80) != 0);
                cycle = 3;
            }
            _ => unreachable!("Invalid oprand!"),
        }

        cycle
    }
}
