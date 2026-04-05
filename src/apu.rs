use crate::types::*;
use log::{trace, warn};

pub struct APU {
    wave_ram: [u8; 32],
}

impl APU {
    /// コンストラクタ 
    pub fn new() -> Self {
        Self {
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
            HWREG_NR51_MASTER_VOLUME_VIN_PANNING => {}
            HWREG_NR51_SOUND_PANNING => {}
            HWREG_NR52_AUDIO_MASTER_CONTROL => {}
            HWREG_CHANNEL3_WAVE_PATTERN_RAM_START..(HWREG_CHANNEL3_WAVE_PATTERN_RAM_START + 0x10) => {
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
        0
    }
}
