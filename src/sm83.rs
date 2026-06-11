use crate::apu::*;
use crate::assembler::*;
use crate::types::*;
use log::{trace, warn};

/// VBlankあたりのシステムクロック数
const SYSTEM_CLOCKS_PER_VBLANK: u32 = MASTER_CLOCKS_PER_VBLANK / 4;
/// DIVレジスタがカウントアップするシステムクロック数
const DIVIDER_RATE_STSTEM_CLOCKS: u32 = 64;

/// ゼロフラグ
const FLAG_Z: u8 = 1 << 7;
/// ネガティブ(BCD)フラグ
const FLAG_N: u8 = 1 << 6;
/// ハーフキャリーフラグ
const FLAG_H: u8 = 1 << 5;
/// キャリーフラグ
const FLAG_C: u8 = 1 << 4;

// 割り込み要求・有効フラグ
/// ジョイパッド割り込み
pub const SM83_INTERRUPT_FLAG_JOYPAD: u8 = 1 << 4;
/// シリアル割り込み
pub const SM83_INTERRUPT_FLAG_SERIAL: u8 = 1 << 3;
/// タイマー割り込み
pub const SM83_INTERRUPT_FLAG_TIMER: u8 = 1 << 2;
/// LCD割り込み
pub const SM83_INTERRUPT_FLAG_LCD: u8 = 1 << 1;
/// VBlank割り込み
pub const SM83_INTERRUPT_FLAG_VBLANK: u8 = 1 << 0;

// MBC(Memory Bank Controller)レジスタアドレスの範囲
/// MBC1 RAM gate register
pub const RAMG_START_ADDRESS: usize = 0x0000;
/// BANK1 MBC1 bank register 1
pub const BANK1_START_ADDRESS: usize = 0x2000;
/// BANK2 MBC1 bank register 2
pub const BANK2_START_ADDRESS: usize = 0x4000;
/// MBC1 mode register
pub const MODE_START_ADDRESS: usize = 0x6000;

/// SM83エミュレータ
pub struct SM83<R>
where
    R: AsRef<[u8]>,
{
    /// レジスタ
    pub regs: SM83Registers,
    /// 64KBメモリ領域
    pub mem: [u8; 65536],
    /// ROM
    pub rom: R,
    /// APU
    apu: APU,
    /// RAMゲートレジスタ
    ramg: u8,
    /// MBC1バンクレジスタ1
    mbc1_bank1: u8,
    /// MBC1バンクレジスタ2
    mbc1_bank2: u8,
    /// MBC1モードレジスタ
    mbc1_mode: u8,
    /// MBC1のバンクマスク
    mbc1_bank_mask: u8,
    /// IME（割り込み有効）フラグ
    ime_flag: bool,
    /// タイマー(TIMA)有効か？
    timer_enable: bool,
    /// タイマーが増加するサイクル数 (M-cycle)
    timer_increment_mcycle: u32,
    /// タイマーティック用カウント (M-cycle)
    timer_tick_mcycle_count: u32,
    /// DIVレジスタ用カウント (M-cycle)
    div_mcycle_count: u32,
    /// VBlank用カウント (M-cycle)
    vblank_mcycle_count: u32,
}

impl<R> SM83<R>
where
    R: AsRef<[u8]>,
{
    /// コンストラクタ
    pub fn new(rom: R) -> Self {
        // ROMはBANK0のサイズ(0x4000)以上かつバンクのサイズの倍数であることを要求
        // ROMの先頭バイトはBANK0に置かれる
        // ROMを配置する側が適切にサイズ調整し、残った領域は0埋めする
        debug_assert!(rom.as_ref().len() >= DMG_ROM_BANK_SIZE);
        debug_assert!((rom.as_ref().len() % DMG_ROM_BANK_SIZE) == 0);
        // バンク数のマスクを作成
        let num_rom_banks = rom.as_ref().len() / DMG_ROM_BANK_SIZE;
        let bank_mask = (1 << ((num_rom_banks - 1).ilog2() + 1)) - 1;
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
            apu: APU::new(),
            ramg: 0xA,
            mbc1_bank1: 1,
            mbc1_bank2: 0,
            mbc1_mode: 0,
            mbc1_bank_mask: bank_mask,
            rom: rom,
            ime_flag: false,
            timer_enable: false,
            timer_increment_mcycle: 256, // Clock select = 00 に相当
            timer_tick_mcycle_count: 0,
            div_mcycle_count: 0,
            vblank_mcycle_count: 0,
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
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        self.write_mem_u8(self.regs.sp as usize, value);
    }

    /// スタックからデータをPOP
    fn pop_stack(&mut self) -> u8 {
        let ret = self.read_mem_u8(self.regs.sp as usize);
        self.regs.sp = self.regs.sp.wrapping_add(1);
        ret
    }

    /// フラグリセット
    pub fn reset_flags(&mut self) {
        self.set_flag(FLAG_Z, true);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_H, true);
        self.set_flag(FLAG_C, true);
        self.ime_flag = false;
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

    /// タイマー関係の状態をリセット
    pub fn reset_timers(&mut self) {
        self.timer_tick_mcycle_count = 0;
        self.div_mcycle_count = 0;
        self.vblank_mcycle_count = 0;
        self.mem[HWREG_DIV_REGISTER] = 0;
        self.mem[HWREG_IF_INTERRUPT_FLAG] &=
            !(SM83_INTERRUPT_FLAG_TIMER | SM83_INTERRUPT_FLAG_VBLANK);
    }

    /// ROMバンクの切り替え(MBC1)
    fn switch_rom_bank_mbc1(&mut self) {
        let bank_number = ((self.mbc1_bank2 << 5) | (self.mbc1_bank1)) & self.mbc1_bank_mask;
        let offset = (bank_number as usize) * DMG_ROM_BANK_SIZE;
        self.mem[ROM_BANK1_START_ADDRESS..(ROM_BANK1_START_ADDRESS + DMG_ROM_BANK_SIZE)]
            .copy_from_slice(&self.rom.as_ref()[offset..(offset + DMG_ROM_BANK_SIZE)]);
    }

    /// ステップ実行
    pub fn execute_step(&mut self) -> (SM83Opcode, u8) {
        // 割り込み処理
        self.handle_interrupt();
        // オペコードをパース
        let (opcode, len) = parse_opcode(&self.mem[(self.regs.pc as usize)..]);
        trace!(
            "{:#06X}: {:02X?} {:X?} {:X?}",
            self.regs.pc,
            &self.mem[(self.regs.pc as usize)..((self.regs.pc + len) as usize)],
            opcode,
            self.regs
        );
        self.regs.pc += len;
        // 命令実行
        let cycle = self.execute_opcode(&opcode);
        // システムクロックティック
        for _ in 0..cycle {
            self.system_clock_tick();
        }
        (opcode, cycle)
    }

    /// 割り込み処理
    fn handle_interrupt(&mut self) {
        // 割り込み無効ならば何もしない
        if !self.ime_flag {
            return;
        }

        // 割り込み優先順(IFのbit0から順)に処理
        let iflags = self.mem[HWREG_IF_INTERRUPT_FLAG] & self.mem[HWREG_IE_INTERRUPT_ENABLE];
        for i in 0..=4 {
            if (iflags & (1 << i)) != 0 {
                // 割り込みフラグをクリア
                self.ime_flag = false;
                self.mem[HWREG_IF_INTERRUPT_FLAG] &= !(1 << i);
                // 現在のPCをスタックにプッシュ
                self.push_stack(((self.regs.pc >> 8) & 0xFF) as u8);
                self.push_stack(((self.regs.pc >> 0) & 0xFF) as u8);
                // 割り込み先にジャンプ
                self.regs.pc = 0x0040 + 8 * i;
                // RETI命令があるまで実行
                loop {
                    let (opcode, _) = self.execute_step();
                    match opcode {
                        SM83Opcode::RETI => break,
                        _ => {}
                    }
                }
            }
        }
    }

    /// 8bitメモリ書き込み
    pub fn write_mem_u8(&mut self, address: usize, value: u8) {
        match address {
            RAMG_START_ADDRESS..BANK1_START_ADDRESS => {
                self.ramg = value & 0xF;
            }
            BANK1_START_ADDRESS..BANK2_START_ADDRESS => {
                self.mbc1_bank1 = value & 0x1F;
                // 0は強制的に1として扱われる
                if self.mbc1_bank1 == 0 {
                    self.mbc1_bank1 = 1;
                }
                self.switch_rom_bank_mbc1();
            }
            BANK2_START_ADDRESS..MODE_START_ADDRESS => {
                self.mbc1_bank2 = value & 0x3;
                self.switch_rom_bank_mbc1();
            }
            MODE_START_ADDRESS..VRAM_START_ADDRESS => {
                self.mbc1_mode = value & 0x1;
                self.switch_rom_bank_mbc1();
                // TODO: RAMバンクスイッチ
            }
            EXTERNAL_RAM_START_ADDRESS..WRAM_BANK0_START_ADDRESS => {
                // 外部RAM
                // RAMGが特定の値のみ有効
                if self.ramg == 0xA {
                    self.mem[address] = value;
                }
            }
            WRAM_BANK0_START_ADDRESS..ECHO_RAM_START_ADDRESS => {
                // RAM
                self.mem[address] = value;
            }
            HWREG_DIV_REGISTER => {
                // どの値の書き込みでも0にリセット
                self.mem[HWREG_DIV_REGISTER] = 0;
                return;
            }
            HWREG_TIMA_TIMER_COUNTER | HWREG_TMA_TIMER_MODULO => {
                // そのまま書き込む
                self.mem[address] = value;
            }
            HWREG_TAC_TIMER_CONTROL => {
                self.timer_enable = (value & 0x4) != 0;
                self.timer_increment_mcycle = match value & 0x3 {
                    0 => 256,
                    1 => 4,
                    2 => 16,
                    3 => 64,
                    _ => unreachable!(),
                };
                self.mem[address] = value;
            }
            HWREG_NR10_CHANNEL1_SWEEP..HWREG_LCDC_LCD_CONTROL
            | HWREG_PCM12_AUDIO_DIGITAL_OUTPUTS_12
            | HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34 => {
                self.apu.write_register(address, value);
                self.mem[address] = value;
            }
            _ => {
                self.mem[address] = value;
            }
        }
        trace!("W: 0x{:04X} <- {:02X}", address, value);
    }

    /// 8bitメモリ読み込み
    pub fn read_mem_u8(&mut self, address: usize) -> u8 {
        let data = match address {
            HWREG_NR10_CHANNEL1_SWEEP..HWREG_LCDC_LCD_CONTROL
            | HWREG_PCM12_AUDIO_DIGITAL_OUTPUTS_12
            | HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34 => self.apu.read_register(address),
            _ => self.mem[address],
        };

        trace!("R: 0x{:04X} -> {:02X}", address, data);
        data
    }

    /// システムクロック(M-Cycle)ティック
    fn system_clock_tick(&mut self) {
        // タイマーティック
        self.timer_tick_mcycle_count += 1;
        if self.timer_tick_mcycle_count >= self.timer_increment_mcycle {
            if self.timer_enable {
                let (tima, overflow) = self.mem[HWREG_TIMA_TIMER_COUNTER].overflowing_add(1);
                self.mem[HWREG_TIMA_TIMER_COUNTER] = if overflow {
                    // タイマー割り込み要求フラグを立てる
                    self.mem[HWREG_IF_INTERRUPT_FLAG] |= SM83_INTERRUPT_FLAG_TIMER;
                    // TIMER MODULOの値にリセット
                    self.mem[HWREG_TMA_TIMER_MODULO]
                } else {
                    tima
                };
            }
            self.timer_tick_mcycle_count -= self.timer_increment_mcycle;
        }

        // DIVレジスタカウントアップ
        self.div_mcycle_count += 1;
        if self.div_mcycle_count >= DIVIDER_RATE_STSTEM_CLOCKS {
            self.mem[HWREG_DIV_REGISTER] = self.mem[HWREG_DIV_REGISTER].wrapping_add(1);
            self.div_mcycle_count -= DIVIDER_RATE_STSTEM_CLOCKS;
        }

        // VBLANK
        self.vblank_mcycle_count += 1;
        if self.vblank_mcycle_count >= SYSTEM_CLOCKS_PER_VBLANK {
            // VBLANK割り込み要求フラグを立てる
            self.mem[HWREG_IF_INTERRUPT_FLAG] |= SM83_INTERRUPT_FLAG_VBLANK;
            self.vblank_mcycle_count -= SYSTEM_CLOCKS_PER_VBLANK;
        }

        // オーディオ信号処理
        for _ in 0..2 {
            self.apu.clock_tick_2mhz();
        }
    }

    /// 音声出力サンプリングレートの設定
    pub fn set_audio_sampling_rate(&mut self, sampling_rate: u32) {
        self.apu.set_sampling_rate(sampling_rate);
    }

    /// 1ステレオサンプル出力
    /// 現在の出力サンプルを元に出力を計算します。サンプリングレート間隔で実行してください
    pub fn output_audio_sample(&mut self) -> [f32; 2] {
        self.apu.compute_output()
    }

    /// 16bitメモリ書き込み
    fn write_mem_u16(&mut self, address: usize, value: u16) {
        trace!("W16: 0x{:04X} <- {:04X}", address, value,);
        self.mem[address + 0] = ((value >> 0) & 0xFF) as u8;
        self.mem[address + 1] = ((value >> 8) & 0xFF) as u8;
    }

    /// オペコード実行
    fn execute_opcode(&mut self, opcode: &SM83Opcode) -> u8 {
        match opcode {
            SM83Opcode::NOP => {
                // 何もしない
                1
            }
            // 算術演算命令
            SM83Opcode::ADD { oprand } => {
                fn add_u8(v1: u8, v2: u8) -> (u8, bool, bool) {
                    let (ret, overflow) = v1.overflowing_add(v2);
                    let half_overflow = ((v1 & 0xF) + (v2 & 0xF)) > 0xF;
                    (ret, overflow, half_overflow)
                }
                fn add_u16(v1: u16, v2: u16) -> (u16, bool, bool) {
                    let (ret, overflow) = v1.overflowing_add(v2);
                    let half_overflow = ((v1 & 0xFFF) + (v2 & 0xFFF)) > 0xFFF;
                    (ret, overflow, half_overflow)
                }
                let cycle;
                let overflow;
                let half_overflow;
                match oprand {
                    SM83Oprand::R8AndR8 { r1, r2 } => {
                        let ret;
                        (ret, overflow, half_overflow) = add_u8(self.get_r8(r1), self.get_r8(r2));
                        self.regs.a = ret;
                        self.set_flag(FLAG_Z, ret == 0);
                        cycle = 1;
                    }
                    SM83Oprand::R8AndR16Indirect { r8, r16 } => {
                        let ret;
                        let address = self.get_r16(r16);
                        (ret, overflow, half_overflow) =
                            add_u8(self.get_r8(r8), self.read_mem_u8(address as usize));
                        self.regs.a = ret;
                        self.set_flag(FLAG_Z, ret == 0);
                        cycle = 2;
                    }
                    SM83Oprand::R8AndN8 { r8, n8 } => {
                        let ret;
                        (ret, overflow, half_overflow) = add_u8(self.get_r8(r8), *n8);
                        self.regs.a = ret;
                        self.set_flag(FLAG_Z, ret == 0);
                        cycle = 2;
                    }
                    SM83Oprand::R16ToR16 { dst, src } => {
                        let ret;
                        (ret, overflow, half_overflow) =
                            add_u16(self.get_r16(dst), self.get_r16(src));
                        self.set_r16(dst, ret);
                        // ゼロフラグは不変
                        cycle = 2;
                    }
                    SM83Oprand::R16AndE8 { r16, e8 } => {
                        let reg = self.get_r16(r16);
                        let r16ret = (reg as i32 + *e8 as i32) as u16;
                        // オーバーフロー判定は8bitの範囲で行う
                        half_overflow = ((reg ^ (*e8 as u16) ^ r16ret) & 0x0010) == 0x0010;
                        overflow = ((reg ^ (*e8 as u16) ^ r16ret) & 0x0100) == 0x0100;
                        self.set_r16(r16, r16ret);
                        self.set_flag(FLAG_Z, false);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, half_overflow);
                self.set_flag(FLAG_C, overflow);
                cycle
            }
            SM83Opcode::SUB { oprand } => {
                fn sub(a: u8, b: u8, _: bool) -> (u8, bool, bool) {
                    let (ret, overflow) = a.overflowing_sub(b);
                    let half_overflow = ((a & 0xF) as i16 - (b & 0xF) as i16) < 0;
                    (ret, overflow, half_overflow)
                }
                let cycle = self.execute_sub_adc_sbc(oprand, sub);
                self.set_flag(FLAG_N, true);
                cycle
            }
            SM83Opcode::ADC { oprand } => {
                fn adc(a: u8, b: u8, carry: bool) -> (u8, bool, bool) {
                    let a16 = a as u16;
                    let b16 = b as u16;
                    let c = if carry { 1 } else { 0 };
                    let ret = a16 + b16 + c;
                    (
                        (ret & 0xFF) as u8,
                        ret >= 0x100,
                        (a16 ^ b16 ^ ret) & 0x10 != 0,
                    )
                }
                let cycle = self.execute_sub_adc_sbc(oprand, adc);
                self.set_flag(FLAG_N, false);
                cycle
            }
            SM83Opcode::SBC { oprand } => {
                fn sbc(a: u8, b: u8, carry: bool) -> (u8, bool, bool) {
                    let a16 = a as u16;
                    let b16 = b as u16;
                    let c = if carry { 1 } else { 0 };
                    let ret = a16.wrapping_sub(b16).wrapping_sub(c);
                    (
                        (ret & 0xFF) as u8,
                        (b16 + c) > a16,
                        ((b16 & 0x0F) + c) > (a16 & 0x0F),
                    )
                }
                let cycle = self.execute_sub_adc_sbc(oprand, sbc);
                self.set_flag(FLAG_N, true);
                cycle
            }
            SM83Opcode::CP { oprand } => {
                // Aを更新しない以外はSUBと同等
                fn sub(a: u8, b: u8, _: bool) -> (u8, bool, bool) {
                    let (ret, overflow) = a.overflowing_sub(b);
                    let half_overflow = ((a & 0xF) as i16 - (b & 0xF) as i16) < 0;
                    (ret, overflow, half_overflow)
                }
                let a_backup = self.regs.a;
                let cycle = self.execute_sub_adc_sbc(oprand, sub);
                self.regs.a = a_backup;
                self.set_flag(FLAG_N, true);
                cycle
            }
            SM83Opcode::DEC { oprand } => {
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let value = self.get_r8(r8);
                        let ret = value.wrapping_sub(1);
                        self.set_r8(r8, ret);
                        self.set_flag(FLAG_Z, ret == 0);
                        self.set_flag(FLAG_N, true);
                        self.set_flag(FLAG_H, (ret & 0x0F) == 0x0F);
                        cycle = 1;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        let ret = value.wrapping_sub(1);
                        self.write_mem_u8(address, ret);
                        self.set_flag(FLAG_Z, ret == 0);
                        self.set_flag(FLAG_N, true);
                        self.set_flag(FLAG_H, (ret & 0x0F) == 0x0F);
                        cycle = 3;
                    }
                    SM83Oprand::R16 { r16 } => {
                        let value = self.get_r16(r16);
                        self.set_r16(r16, value.wrapping_sub(1));
                        cycle = 2;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                cycle
            }
            SM83Opcode::INC { oprand } => {
                let cycle;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let value = self.get_r8(r8);
                        let ret = value.wrapping_add(1);
                        self.set_r8(r8, ret);
                        self.set_flag(FLAG_Z, ret == 0);
                        self.set_flag(FLAG_N, false);
                        self.set_flag(FLAG_H, (ret & 0x0F) == 0);
                        cycle = 1;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        let ret = value.wrapping_add(1);
                        self.write_mem_u8(address, ret);
                        self.set_flag(FLAG_Z, ret == 0);
                        self.set_flag(FLAG_N, false);
                        self.set_flag(FLAG_H, (ret & 0x0F) == 0);
                        cycle = 3;
                    }
                    SM83Oprand::R16 { r16 } => {
                        let value = self.get_r16(r16);
                        self.set_r16(r16, value.wrapping_add(1));
                        cycle = 2;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
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
            // bit操作命令
            SM83Opcode::BIT { u3, oprand } => {
                let cycle;
                let ret;
                let test_bit = 1 << u3;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        ret = reg & test_bit;
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16);
                        let value = self.read_mem_u8(address as usize);
                        ret = value & test_bit;
                        cycle = 3;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, true);
                cycle
            }
            SM83Opcode::RES { u3, oprand } => {
                let cycle;
                let mask = !(1 << u3);
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        self.set_r8(r8, reg & mask);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        self.write_mem_u8(address, value & mask);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
                cycle
            }
            SM83Opcode::SET { u3, oprand } => {
                let cycle;
                let bit = 1 << u3;
                match oprand {
                    SM83Oprand::R8 { r8 } => {
                        let reg = self.get_r8(r8);
                        self.set_r8(r8, reg | bit);
                        cycle = 2;
                    }
                    SM83Oprand::R16Indirect { r16 } => {
                        let address = self.get_r16(r16) as usize;
                        let value = self.read_mem_u8(address);
                        self.write_mem_u8(address, value | bit);
                        cycle = 4;
                    }
                    _ => unreachable!("Invalid oprand!"),
                }
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
            // bitシフト命令
            SM83Opcode::RLCA => {
                let msb = self.regs.a & 0x80;
                self.regs.a = (self.regs.a << 1) | (msb >> 7);
                self.set_flag(FLAG_Z, false);
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, msb != 0);
                1
            }
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
            SM83Opcode::RRA => {
                let lsb = self.regs.a & 0x01;
                let msb = if self.test_flag(FLAG_C) { 0x80 } else { 0x00 };
                self.regs.a = (self.regs.a >> 1) | msb;
                self.set_flag(FLAG_C, lsb != 0);
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
            // ロード命令
            SM83Opcode::LD { oprand } => self.execute_ld(oprand),
            SM83Opcode::LDH { oprand } => match oprand {
                SM83Oprand::R8ToA8 { dst, src } => {
                    let value = self.get_r8(src);
                    let address = HWREG_START_ADDRESS as usize + *dst as usize;
                    self.write_mem_u8(address, value);
                    3
                }
                SM83Oprand::A8ToR8 { dst, src } => {
                    let address = HWREG_START_ADDRESS as usize + *src as usize;
                    let value = self.read_mem_u8(address);
                    self.set_r8(dst, value);
                    3
                }
                SM83Oprand::R8IndirectToR8 { dst, src } => {
                    let address = HWREG_START_ADDRESS as usize + self.get_r8(src) as usize;
                    let value = self.read_mem_u8(address);
                    self.set_r8(dst, value);
                    2
                }
                SM83Oprand::R8ToR8Indirect { dst, src } => {
                    let value = self.get_r8(src);
                    let address = HWREG_START_ADDRESS as usize + self.get_r8(dst) as usize;
                    self.write_mem_u8(address, value);
                    2
                }
                _ => unreachable!("Invalid oprand!"),
            },
            // ジャンプ・コール命令
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
                SM83Oprand::R16 { r16 } => {
                    match r16 {
                        &SM83Register16::HL => self.regs.pc = self.get_r16(r16),
                        _ => unreachable!("Invalid register!"),
                    }
                    1
                }
                _ => unreachable!("Invalid oprand!"),
            },
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
                    self.regs.pc = *a16;
                    6
                }
                _ => unreachable!("Invalid oprand!"),
            },
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
            SM83Opcode::RETI => {
                let low = self.pop_stack();
                let high = self.pop_stack();
                self.regs.pc = ((high as u16) << 8) | (low as u16);
                self.ime_flag = true;
                4
            }
            SM83Opcode::RST { vec } => {
                self.push_stack(((self.regs.pc >> 8) & 0xFF) as u8);
                self.push_stack(((self.regs.pc >> 0) & 0xFF) as u8);
                self.regs.pc = *vec as u16;
                4
            }
            // スタック操作命令
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
            // その他
            SM83Opcode::CCF => {
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, !self.test_flag(FLAG_C));
                1
            }
            SM83Opcode::CPL => {
                self.regs.a = !self.regs.a;
                self.set_flag(FLAG_N, true);
                self.set_flag(FLAG_H, true);
                1
            }
            SM83Opcode::DAA => {
                let mut ret = self.regs.a;
                let mut carry = self.test_flag(FLAG_C);
                if self.test_flag(FLAG_N) {
                    // ハーフキャリーフラグが設定されている or 下位ニブルが0xA以上ならば0x6を足す
                    if self.test_flag(FLAG_H) || (ret & 0x0F) >= 0xA {
                        (ret, carry) = ret.overflowing_sub(0x06);
                    }
                    // キャリーフラグがクリアされている or 上位ニブルが0xA以上ならば0x60を足す
                    if !self.test_flag(FLAG_C) || ((ret & 0xF0) >> 4) >= 0xA {
                        (ret, carry) = ret.overflowing_sub(0x60);
                    }
                } else {
                    // ハーフキャリーフラグが設定されている or 下位ニブルが0xA以上ならば0x6を足す
                    if self.test_flag(FLAG_H) || (ret & 0x0F) >= 0xA {
                        (ret, carry) = ret.overflowing_add(0x06);
                    }
                    // キャリーフラグがクリアされている or 上位ニブルが0xA以上ならば0x60を足す
                    if !self.test_flag(FLAG_C) || ((ret & 0xF0) >> 4) >= 0xA {
                        (ret, carry) = ret.overflowing_add(0x60);
                    }
                }
                // 最上位ビットにキャリーフラグをセットする
                ret = if self.test_flag(FLAG_C) {
                    ret | 0x80
                } else {
                    ret & 0x7F
                };
                self.regs.a = ret;
                self.set_flag(FLAG_Z, ret == 0);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, carry);
                1
            }
            SM83Opcode::DI => {
                self.ime_flag = false;
                1
            }
            SM83Opcode::EI => {
                // NOTE: この命令実行後に有効になる
                self.ime_flag = true;
                1
            }
            SM83Opcode::HALT => {
                // 低電力モードに移行？
                warn!("execute HALT instruction");
                1
            }
            SM83Opcode::SCF => {
                self.set_flag(FLAG_N, false);
                self.set_flag(FLAG_H, false);
                self.set_flag(FLAG_C, true);
                1
            }
            SM83Opcode::STOP => {
                warn!("execute STOP instruction");
                // DIVレジスタを0クリア
                self.mem[HWREG_DIV_REGISTER] = 0;
                1
            }
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

    /// SUB/ADC/SBC/命令の実行
    fn execute_sub_adc_sbc(
        &mut self,
        oprand: &SM83Oprand,
        op: fn(u8, u8, bool) -> (u8, bool, bool),
    ) -> u8 {
        let ret;
        let cycle;
        let overflow;
        let half_overflow;
        let carry = self.test_flag(FLAG_C);
        match oprand {
            SM83Oprand::R8AndR8 { r1, r2 } => {
                (ret, overflow, half_overflow) = op(self.get_r8(r1), self.get_r8(r2), carry);
                cycle = 1;
            }
            SM83Oprand::R8AndR16Indirect { r8, r16 } => {
                let address = self.get_r16(r16);
                (ret, overflow, half_overflow) =
                    op(self.get_r8(r8), self.read_mem_u8(address as usize), carry);
                cycle = 2;
            }
            SM83Oprand::R8AndN8 { r8, n8 } => {
                (ret, overflow, half_overflow) = op(self.get_r8(r8), *n8, carry);
                cycle = 2;
            }
            _ => unreachable!("Invalid oprand!"),
        }
        self.regs.a = ret;
        self.set_flag(FLAG_Z, ret == 0);
        self.set_flag(FLAG_H, half_overflow);
        self.set_flag(FLAG_C, overflow);
        cycle
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
                cycle = 4;
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
            SM83Oprand::R16ToR16 { dst, src } => {
                let value = self.get_r16(src);
                self.set_r16(dst, value);
                cycle = 2;
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
