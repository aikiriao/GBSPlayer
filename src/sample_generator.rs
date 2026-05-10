use crate::types::*;
use crate::length_timer::*;

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
    /// 持続時間有効か
    length_enable: bool,
    /// 再生要求フラグ
    trigger: bool,
    /// 波形RAM 4bit深度 x 32サンプル
    wave_ram: [u8; 32],
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
            trigger: false,
            wave_ram: [0u8; 32],
            length_timer: LengthTimer::new(),
        }
    }

    /// DACのON/OFF
    pub fn set_dac_enable(&mut self, value: u8) {
        self.dac_enable = (value & 0x80) != 0;
    }

    /// 長さタイマーの設定
    pub fn set_length_timer(&mut self, value: u8) {
        self.length_timer.set_length_timer(value, 1);
    }

    /// 出力レベルの設定
    pub fn set_output_level(&mut self, value: u8) {
        self.output_level_shift = match (value >> 4) & 0x3 {
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
    }

    /// 周期上位ビット・制御フラグ設定
    pub fn set_period_high_control(&mut self, value: u8) {
        self.period = (((value & 0x7) as u16) << 8) | (self.period & 0x00FF);
        self.length_enable = (value & 0x40) != 0;
        self.trigger = (value & 0x80) != 0;
        if self.trigger {
            self.process_trigger();
        }
    }

    /// 波形RAMの設定
    pub fn set_wave_ram(&mut self, address: usize, value: u8) {
        // TODO: 再生中にセットすると書き込みは無視される
        let smpl = 2 * (address -  HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
        self.wave_ram[smpl + 0] = (value >> 4) & 0xF;
        self.wave_ram[smpl + 1] = (value >> 4) & 0xF;
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
            4 => 0 << 4,
            0 => 1 << 4,
            1 => 2 << 4,
            2 => 3 << 4,
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
        ret |= (self.period & 0x7) as u8;
        ret |= if self.length_enable { 0x40 } else { 0 };
        ret |= if self.trigger { 0x80 } else { 0 };
        ret
    }

    /// 波形RAMの取得
    pub fn get_wave_ram(&self, address: usize) -> u8 {
        // TODO: 再生中に読み出すと0xFFを返す
        let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
        (self.wave_ram[smpl + 0] << 4) | (self.wave_ram[smpl + 1])
    }

    /// トリガーON時の処理
    fn process_trigger(&mut self) {
        // チャンネルを有効に
        self.enable = true;
        // 長さタイマーが切れていたらリセット
        if self.length_timer.expired {
            self.length_timer.reset();
        }
    }

    /// 1システムクロック単位処理
    pub fn system_clock_tick(&mut self, mem: &mut [u8]) {
        // TODO

        // 長さタイマーが時間切れしていたら無効に
        if self.length_timer.expired {
            self.enable = false;
        }

        // 長さタイマーの更新
        self.length_timer.system_clock_tick();
    }
}
