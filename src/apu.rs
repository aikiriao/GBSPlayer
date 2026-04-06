use crate::types::*;
use log::{trace, warn};

/// 波形RAM領域終端アドレス
const HWREG_CHANNEL3_WAVE_PATTERN_RAM_END: usize = HWREG_CHANNEL3_WAVE_PATTERN_RAM_START + 16;

/// パンニング
#[derive(Debug, Clone, Copy)]
enum Pan {
    /// 左
    Left,
    /// 右
    Right,
    /// 中央
    Center,
    /// 設定なし（無音）
    Ignore,
}

pub struct APU {
    /// オーディオON/OFFフラグ
    audio_on: bool,
    /// チャンネル単位のON/OFFフラグ
    ch_on: [bool; 4],
    /// 各チャンネルのパン
    ch_pan: [Pan; 4],
    /// 波形RAM 4bit深度 x 32サンプル
    wave_ram: [u8; 32],
}

impl APU {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            audio_on: false,
            ch_on: [false; 4],
            ch_pan: [Pan::Center; 4],
            wave_ram: [0u8; 32],
        }
    }

    /// レジスタへの書き込み
    pub fn write_register(&mut self, address: usize, value: u8) {
        match address {
            HWREG_NR10_CHANNEL1_SWEEP => {}
            HWREG_NR11_CHANNEL1_LENGTH_TIMER_DURY_CYCLE => {}
            HWREG_NR12_CHANNEL1_VOLUME_ENVELOPE => {}
            HWREG_NR13_CHANNEL1_PERIOD_LOW => {}
            HWREG_NR14_CHANNEL1_PERIOD_HIGH_CONTROL => {}
            HWREG_NR21_CHANNEL2_LENGTH_TIMER_DURY_CYCLE => {}
            HWREG_NR22_CHANNEL2_VOLUME_ENVELOPE => {}
            HWREG_NR23_CHANNEL2_PERIOD_LOW => {}
            HWREG_NR24_CHANNEL2_PERIOD_HIGH_CONTROL => {}
            HWREG_NR30_CHANNEL3_DAC_ENABLE => {}
            HWREG_NR31_CHANNEL3_LENGTH_TIMER => {}
            HWREG_NR32_CHANNEL3_OUTPUT_LEVEL => {}
            HWREG_NR33_CHANNEL3_PERIOD_LOW => {}
            HWREG_NR33_CHANNEL3_PERIOD_HIGH_CONTROL => {}
            HWREG_NR41_CHANNEL4_LENGTH_TIMER => {}
            HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE => {}
            HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS => {}
            HWREG_NR44_CHANNEL4_CONTROL => {}
            HWREG_NR50_MASTER_VOLUME_VIN_PANNING => {}
            HWREG_NR51_SOUND_PANNING => {
                for ch in 0..4 {
                    let left = ((value >> ch) & 0x01) != 0;
                    let right = ((value >> ch) & 0x10) != 0;
                    self.ch_pan[ch] = if left && right {
                        Pan::Center
                    } else if left && !right {
                        Pan::Left
                    } else if !left && right {
                        Pan::Right
                    } else {
                        Pan::Ignore
                    };
                }
            }
            HWREG_NR52_AUDIO_MASTER_CONTROL => {
                self.audio_on = (value & 0x80) != 0;
                for ch in 0..4 {
                    self.ch_on[ch] = ((value >> ch) & 0x1) != 0;
                }
            }
            HWREG_CHANNEL3_WAVE_PATTERN_RAM_START..HWREG_CHANNEL3_WAVE_PATTERN_RAM_END => {
                // TODO: 再生中にセットすると書き込みは無視される
                let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
                self.wave_ram[smpl + 0] = (value >> 4) & 0xF;
                self.wave_ram[smpl + 1] = (value >> 0) & 0xF;
            }
            _ => {
                // それ以外は無視
            }
        }
    }

    /// レジスタからの読み出し
    pub fn read_register(&mut self, address: usize) -> u8 {
        match address {
            HWREG_NR10_CHANNEL1_SWEEP => 0,
            HWREG_NR11_CHANNEL1_LENGTH_TIMER_DURY_CYCLE => 0,
            HWREG_NR12_CHANNEL1_VOLUME_ENVELOPE => 0,
            HWREG_NR13_CHANNEL1_PERIOD_LOW => 0,
            HWREG_NR14_CHANNEL1_PERIOD_HIGH_CONTROL => 0,
            HWREG_NR21_CHANNEL2_LENGTH_TIMER_DURY_CYCLE => 0,
            HWREG_NR22_CHANNEL2_VOLUME_ENVELOPE => 0,
            HWREG_NR23_CHANNEL2_PERIOD_LOW => 0,
            HWREG_NR24_CHANNEL2_PERIOD_HIGH_CONTROL => 0,
            HWREG_NR30_CHANNEL3_DAC_ENABLE => 0,
            HWREG_NR31_CHANNEL3_LENGTH_TIMER => 0,
            HWREG_NR32_CHANNEL3_OUTPUT_LEVEL => 0,
            HWREG_NR33_CHANNEL3_PERIOD_LOW => 0,
            HWREG_NR33_CHANNEL3_PERIOD_HIGH_CONTROL => 0,
            HWREG_NR41_CHANNEL4_LENGTH_TIMER => 0,
            HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE => 0,
            HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS => 0,
            HWREG_NR44_CHANNEL4_CONTROL => 0,
            HWREG_NR50_MASTER_VOLUME_VIN_PANNING => 0,
            HWREG_NR51_SOUND_PANNING => {
                let mut ret = 0;
                for ch in 0..4 {
                    let left = 0x01 << ch;
                    let right = 0x10 << ch;
                    match self.ch_pan[ch] {
                        Pan::Center => {
                            ret |= left;
                            ret |= right;
                        }
                        Pan::Left => {
                            ret |= left;
                        }
                        Pan::Right => {
                            ret |= right;
                        }
                        Pan::Ignore => {}
                    }
                }
                ret
            }
            HWREG_NR52_AUDIO_MASTER_CONTROL => {
                let mut ret = 0;
                ret |= if self.audio_on { 0x80 } else { 0 };
                for ch in 0..4 {
                    ret |= if self.ch_on[ch] { 1 << ch } else { 0 };
                }
                ret
            }
            HWREG_CHANNEL3_WAVE_PATTERN_RAM_START..HWREG_CHANNEL3_WAVE_PATTERN_RAM_END => {
                // TODO: 再生中に読み出すと0xFFを返す
                let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
                (self.wave_ram[smpl + 0] << 4) | (self.wave_ram[smpl + 1])
            }
            _ => {
                // それ以外は0を返す
                0
            }
        }
    }
}
