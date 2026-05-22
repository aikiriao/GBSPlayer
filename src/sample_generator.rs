use crate::length_timer::*;
use crate::types::*;

/// サンプルジェネレータの動作クロック
const SAMPLE_GENERATOR_CLOCK_HZ: u32 = 2097152;

/// CH3: サンプルジェネレータ
#[derive(Debug)]
pub struct SampleGenerator {
    /// 有効か？
    pub enable: bool,
    /// DAC有効か
    dac_enable: bool,
    /// 出力レベル右シフト量
    output_level_shift: u8,
    /// 周期
    period: u16,
    /// 周期の変更があったか？
    period_changed: bool,
    /// サンプル更新間隔
    sample_update_period: u16,
    /// サンプル更新のためのシステムクロックカウンタ
    sample_update_counter: u16,
    /// 持続時間有効か
    length_enable: bool,
    /// 再生要求フラグ
    trigger: bool,
    /// 波形RAM 4bit深度 x 32サンプル
    wave_ram: [u8; 32],
    /// 波形RAMの参照インデックス
    wave_ram_index: usize,
    /// 長さタイマー
    length_timer: LengthTimer,
}

impl SampleGenerator {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            enable: false,
            dac_enable: false,
            output_level_shift: 0,
            length_enable: false,
            period: 0,
            period_changed: false,
            sample_update_period: 0,
            sample_update_counter: 0,
            trigger: false,
            wave_ram: [0u8; 32],
            wave_ram_index: 0,
            length_timer: LengthTimer::new(SAMPLE_GENERATOR_CLOCK_HZ),
        }
    }

    /// DACのON/OFF
    pub fn set_dac_enable(&mut self, value: u8) {
        self.dac_enable = (value & 0x80) != 0;
        if !self.dac_enable {
            self.enable = false;
        }
    }

    /// 長さタイマーの設定
    pub fn set_length_timer(&mut self, value: u8) {
        self.length_timer.set_length_timer(value, 256);
    }

    /// 出力レベルの設定
    pub fn set_output_level(&mut self, value: u8) {
        self.output_level_shift = match (value >> 5) & 0x3 {
            0 => 4,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => unreachable!(),
        }
    }

    /// 周期下位ビット設定
    pub fn set_period_low(&mut self, value: u8) {
        self.period = (self.period & 0xFF00) | (value as u16);
        self.period_changed = true;
    }

    /// 周期上位ビット・制御フラグ設定
    pub fn set_period_high_control(&mut self, value: u8) {
        self.period = (((value & 0x7) as u16) << 8) | (self.period & 0x00FF);
        self.period_changed = true;
        self.length_enable = (value & 0x40) != 0;
        self.trigger = (value & 0x80) != 0;
        if self.trigger {
            self.process_trigger();
        }
    }

    /// 波形RAMの設定
    pub fn set_wave_ram(&mut self, address: usize, value: u8) {
        // 再生中は書き込みは無視される
        if !self.enable {
            let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
            self.wave_ram[smpl + 0] = (value >> 4) & 0xF;
            self.wave_ram[smpl + 1] = (value >> 0) & 0xF;
        }
    }

    /// DACのON/OFF
    pub fn get_dac_enable(&self) -> u8 {
        if self.dac_enable {
            0x80
        } else {
            0x00
        }
    }

    /// 長さタイマーの取得
    pub fn get_length_timer(&self) -> u8 {
        self.length_timer.get_initial_length_timer()
    }

    /// 出力レベルの設定
    pub fn get_output_level(&self) -> u8 {
        match self.output_level_shift {
            4 => 0 << 5,
            0 => 1 << 5,
            1 => 2 << 5,
            2 => 3 << 5,
            _ => unreachable!(),
        }
    }

    /// 周期下位ビット設定値の取得
    pub fn get_period_low(&self) -> u8 {
        (self.period & 0x00FF) as u8
    }

    /// 周期上位ビット・制御フラグ設定値の取得
    pub fn get_period_high_control(&self) -> u8 {
        let mut ret = 0;
        ret |= ((self.period >> 8) & 0x7) as u8;
        ret |= if self.length_enable { 0x40 } else { 0 };
        ret |= if self.trigger { 0x80 } else { 0 };
        ret
    }

    /// 波形RAMの取得
    pub fn get_wave_ram(&self, address: usize) -> u8 {
        if self.enable {
            // 再生中に読み出すと0xFFを返す
            0xFF
        } else {
            let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
            (self.wave_ram[smpl + 0] << 4) | (self.wave_ram[smpl + 1])
        }
    }

    /// トリガーON時の処理
    fn process_trigger(&mut self) {
        // チャンネルを有効に
        self.enable = true;
        // 長さタイマーが切れていたら再度トリガー処理
        if self.length_timer.expired {
            self.length_timer.process_trigger();
        }
        // 更新周期の設定
        self.sample_update_period = 2048 - self.period;
        // サンプル参照位置のリセット
        self.wave_ram_index = 1;
    }

    /// 1システムクロック単位処理
    pub fn clock_tick_2mhz(&mut self) -> Option<u8> {
        let mut out = None;

        if self.enable {
            // カウンタ増加
            self.sample_update_counter += 1;

            // サンプル更新
            if self.sample_update_counter >= self.sample_update_period {
                out = Some(self.wave_ram[self.wave_ram_index] >> self.output_level_shift);
                self.wave_ram_index = (self.wave_ram_index + 1) & 0x1F;
                self.sample_update_counter -= self.sample_update_period;
                // 周期の反映はサンプル出力後
                if self.period_changed {
                    self.sample_update_period = 2048 - self.period;
                    self.period_changed = false;
                }
            }

            // 長さタイマーの更新
            self.length_timer.clock_tick();

            // 長さタイマーが時間切れしていたら無効に
            if self.length_timer.expired {
                self.enable = false;
            }
        }

        out
    }
}
