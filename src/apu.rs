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

/// デューティ比
#[derive(Debug, Clone, Copy)]
enum DutyRatio {
    /// 12.5%
    Duty12_5,
    /// 25%
    Duty25,
    /// 50%
    Duty50,
    /// 75%
    Duty75,
}

/// スイープ（変化）の方向
#[derive(Debug, Clone, Copy)]
enum SweepDirection {
    /// 正
    Positive,
    /// 負
    Negative,
}

/// CH1/CH2: パルス（矩形波）ジェネレータ
#[derive(Debug)]
struct PulseGenerator {
    /// 周期変更頻度
    period_sweep_pace: u8,
    /// 周期変更方向
    period_sweep_direction: SweepDirection,
    /// 周期変更ステップ
    period_sweep_step: u8,
    /// 矩形波のデューティ比
    duty: DutyRatio,
    /// 持続時間
    initial_length_timer: u8,
    /// 残り時間
    length_timer: u8,
    /// 初期ボリューム
    initial_volume: u8,
    /// ボリューム現在値
    volume: u8,
    /// ボリューム変更頻度
    volume_sweep_pace: u8,
    /// ボリューム変更方向
    volume_sweep_direction: SweepDirection,
    /// 周期
    period: u16,
    /// 持続時間有効か
    length_enable: bool,
    /// 再生要求フラグ
    trigger: bool,
}

/// CH3: サンプルジェネレータ
#[derive(Debug)]
struct SampleGenerator {
    /// DAC有効か
    dac_enable: bool,
    /// 持続時間
    initial_length_timer: u8,
    /// 残り時間
    length_timer: u8,
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
}

/// Audio Processing Unit
pub struct APU {
    /// オーディオON/OFFフラグ
    audio_on: bool,
    /// チャンネル単位のON/OFFフラグ
    ch_on: [bool; 4],
    /// 各チャンネルのパン
    ch_pan: [Pan; 4],
    /// マスターボリューム
    master_volume: [u8; 2],
    /// VIN（外部音声入力。未使用。現実のタイトルでも未使用）
    vin: [bool; 2],
    /// パルスジェネレータ
    pulse_generator: [PulseGenerator; 2],
    /// サンプルジェネレータ
    sample_generator: SampleGenerator,
}

impl PulseGenerator {
    /// コンストラクタ
    fn new() -> Self {
        Self {
            period_sweep_pace: 0,
            period_sweep_direction: SweepDirection::Positive,
            period_sweep_step: 0,
            duty: DutyRatio::Duty12_5,
            initial_length_timer: 0,
            length_timer: 0,
            initial_volume: 0,
            volume: 0,
            volume_sweep_pace: 0,
            volume_sweep_direction: SweepDirection::Positive,
            period: 0,
            length_enable: false,
            trigger: false,
        }
    }

    /// スイープの設定
    fn set_sweep(&mut self, value: u8) {
        self.period_sweep_pace = (value >> 4) & 0x7;
        self.period_sweep_direction = if (value & 0x8) == 0 {
            SweepDirection::Positive
        } else {
            SweepDirection::Negative
        };
        self.period_sweep_step = value & 0x7;
    }

    /// 長さタイマー・デューティの設定
    fn set_length_timer_duty_cycle(&mut self, value: u8) {
        self.duty = match (value >> 5) & 0x3 {
            0 => DutyRatio::Duty12_5,
            1 => DutyRatio::Duty25,
            2 => DutyRatio::Duty50,
            3 => DutyRatio::Duty75,
            _ => unreachable!(),
        };
        self.initial_length_timer = value & 0x1F;
    }

    /// ボリューム・エンベロープ設定
    fn set_volume_envelope(&mut self, value: u8) {
        self.initial_volume = (value >> 4) & 0xF;
        self.volume_sweep_direction = if (value & 0x8) == 0 {
            SweepDirection::Positive
        } else {
            SweepDirection::Negative
        };
        self.volume_sweep_pace = value & 0x7;
    }

    /// 周期下位ビット設定
    fn set_period_low(&mut self, value: u8) {
        self.period = (self.period & 0xFF00) | (value as u16);
    }

    /// 周期上位ビット・制御フラグ設定
    fn set_period_high_control(&mut self, value: u8) {
        self.period = (((value & 0x7) as u16) << 8) | (self.period & 0x00FF);
        self.length_enable = (value & 0x40) != 0;
        self.trigger = (value & 0x80) != 0;
    }

    /// スイープ設定値の取得
    fn get_sweep(&self) -> u8 {
        let mut ret = 0;
        ret |= self.period_sweep_pace << 4;
        ret |= match self.period_sweep_direction {
            SweepDirection::Positive => 0,
            SweepDirection::Negative => 0x8,
        };
        ret |= self.period_sweep_step;
        ret
    }

    /// 長さタイマー・デューティ設定値の取得
    fn get_length_timer_duty_cycle(&self) -> u8 {
        let mut ret = 0;
        ret |= match self.duty {
            DutyRatio::Duty12_5 => 0 << 5,
            DutyRatio::Duty25 => 1 << 5,
            DutyRatio::Duty50 => 2 << 5,
            DutyRatio::Duty75 => 3 << 5,
        };
        ret |= self.initial_length_timer;
        ret
    }

    /// ボリューム・エンベロープ設定値の取得
    fn get_volume_envelope(&self) -> u8 {
        let mut ret = 0;
        ret |= self.initial_volume << 4;
        ret |= match self.volume_sweep_direction {
            SweepDirection::Positive => 0 << 3,
            SweepDirection::Negative => 1 << 3,
        };
        ret |= self.volume_sweep_pace;
        ret
    }

    /// 周期下位ビット設定値の取得
    fn get_period_low(&self) -> u8 {
        (self.period & 0x00FF) as u8
    }

    /// 周期上位ビット・制御フラグ設定値の取得
    fn get_period_high_control(&self) -> u8 {
        let mut ret = 0;
        ret |= (self.period & 0x7) as u8;
        ret |= if self.length_enable { 0x40 } else { 0 };
        ret |= if self.trigger { 0x80 } else { 0 };
        ret
    }
}

impl SampleGenerator {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            dac_enable: false,
            initial_length_timer: 0,
            length_timer: 0,
            output_level_shift: 0,
            length_enable: false,
            period: 0,
            trigger: false,
            wave_ram: [0u8; 32],
        }
    }

    /// DACのON/OFF
    fn set_dac_enable(&mut self, value: u8) {
        self.dac_enable = (value & 0x80) != 0;
    }

    /// 長さタイマーの設定
    fn set_length_timer(&mut self, value: u8) {
        self.initial_length_timer = value;
    }

    /// 出力レベルの設定
    fn set_output_level(&mut self, value: u8) {
        self.output_level_shift = match (value >> 4) & 0x3 {
            0 => 4,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => unreachable!(),
        }
    }

    /// 周期下位ビット設定
    fn set_period_low(&mut self, value: u8) {
        self.period = (self.period & 0xFF00) | (value as u16);
    }

    /// 周期上位ビット・制御フラグ設定
    fn set_period_high_control(&mut self, value: u8) {
        self.period = (((value & 0x7) as u16) << 8) | (self.period & 0x00FF);
        self.length_enable = (value & 0x40) != 0;
        self.trigger = (value & 0x80) != 0;
    }

    /// DACのON/OFF
    fn get_dac_enable(&self) -> u8 {
        if self.dac_enable {
            0x80
        } else {
            0x00
        }
    }

    /// 長さタイマーの取得
    fn get_length_timer(&self) -> u8 {
        self.initial_length_timer
    }

    /// 出力レベルの設定
    fn get_output_level(&self) -> u8 {
        match self.output_level_shift {
            4 => 0 << 4,
            0 => 1 << 4,
            1 => 2 << 4,
            2 => 3 << 4,
            _ => unreachable!(),
        }
    }

    /// 周期下位ビット設定値の取得
    fn get_period_low(&self) -> u8 {
        (self.period & 0x00FF) as u8
    }

    /// 周期上位ビット・制御フラグ設定値の取得
    fn get_period_high_control(&self) -> u8 {
        let mut ret = 0;
        ret |= (self.period & 0x7) as u8;
        ret |= if self.length_enable { 0x40 } else { 0 };
        ret |= if self.trigger { 0x80 } else { 0 };
        ret
    }
}

impl APU {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            audio_on: false,
            ch_on: [false; 4],
            master_volume: [0u8; 2],
            vin: [false; 2],
            ch_pan: [Pan::Center; 4],
            sample_generator: SampleGenerator::new(),
            pulse_generator: [PulseGenerator::new(), PulseGenerator::new()],
        }
    }

    /// レジスタへの書き込み
    pub fn write_register(&mut self, address: usize, value: u8) {
        match address {
            HWREG_NR10_CHANNEL1_SWEEP => {
                self.pulse_generator[0].set_sweep(value);
            }
            HWREG_NR11_CHANNEL1_LENGTH_TIMER_DURY_CYCLE => {
                self.pulse_generator[0].set_length_timer_duty_cycle(value)
            }
            HWREG_NR12_CHANNEL1_VOLUME_ENVELOPE => {
                self.pulse_generator[0].set_volume_envelope(value);
            }
            HWREG_NR13_CHANNEL1_PERIOD_LOW => {
                self.pulse_generator[0].set_period_low(value);
            }
            HWREG_NR14_CHANNEL1_PERIOD_HIGH_CONTROL => {
                self.pulse_generator[0].set_period_high_control(value);
            }
            HWREG_NR21_CHANNEL2_LENGTH_TIMER_DURY_CYCLE => {
                self.pulse_generator[1].set_sweep(value);
            }
            HWREG_NR22_CHANNEL2_VOLUME_ENVELOPE => {
                self.pulse_generator[1].set_volume_envelope(value);
            }
            HWREG_NR23_CHANNEL2_PERIOD_LOW => {
                self.pulse_generator[1].set_period_low(value);
            }
            HWREG_NR24_CHANNEL2_PERIOD_HIGH_CONTROL => {
                self.pulse_generator[1].set_period_high_control(value);
            }
            HWREG_NR30_CHANNEL3_DAC_ENABLE => {
                self.sample_generator.set_dac_enable(value);
            }
            HWREG_NR31_CHANNEL3_LENGTH_TIMER => {
                self.sample_generator.set_length_timer(value);
            }
            HWREG_NR32_CHANNEL3_OUTPUT_LEVEL => {
                self.sample_generator.set_output_level(value);
            }
            HWREG_NR33_CHANNEL3_PERIOD_LOW => {
                self.sample_generator.set_period_low(value);
            }
            HWREG_NR33_CHANNEL3_PERIOD_HIGH_CONTROL => {
                self.sample_generator.set_period_high_control(value);
            }
            HWREG_NR41_CHANNEL4_LENGTH_TIMER => {}
            HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE => {}
            HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS => {}
            HWREG_NR44_CHANNEL4_CONTROL => {}
            HWREG_NR50_MASTER_VOLUME_VIN_PANNING => {
                self.vin[0] = (value & 0x80) != 0;
                self.vin[1] = (value & 0x08) != 0;
                self.master_volume[0] = (value >> 4) & 0x7;
                self.master_volume[1] = (value >> 0) & 0x7;
            }
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
                let wg = &mut self.sample_generator;
                // TODO: 再生中にセットすると書き込みは無視される
                let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
                wg.wave_ram[smpl + 0] = (value >> 4) & 0xF;
                wg.wave_ram[smpl + 1] = (value >> 0) & 0xF;
            }
            _ => {
                // それ以外は無視
            }
        }
    }

    /// レジスタからの読み出し
    pub fn read_register(&mut self, address: usize) -> u8 {
        match address {
            HWREG_NR10_CHANNEL1_SWEEP => self.pulse_generator[0].get_sweep(),
            HWREG_NR11_CHANNEL1_LENGTH_TIMER_DURY_CYCLE => {
                self.pulse_generator[0].get_length_timer_duty_cycle()
            }
            HWREG_NR12_CHANNEL1_VOLUME_ENVELOPE => self.pulse_generator[0].get_volume_envelope(),
            HWREG_NR13_CHANNEL1_PERIOD_LOW => self.pulse_generator[0].get_period_low(),
            HWREG_NR14_CHANNEL1_PERIOD_HIGH_CONTROL => {
                self.pulse_generator[0].get_period_high_control()
            }
            HWREG_NR21_CHANNEL2_LENGTH_TIMER_DURY_CYCLE => {
                self.pulse_generator[1].get_length_timer_duty_cycle()
            }
            HWREG_NR22_CHANNEL2_VOLUME_ENVELOPE => self.pulse_generator[1].get_volume_envelope(),
            HWREG_NR23_CHANNEL2_PERIOD_LOW => self.pulse_generator[1].get_period_low(),
            HWREG_NR24_CHANNEL2_PERIOD_HIGH_CONTROL => {
                self.pulse_generator[1].get_period_high_control()
            }
            HWREG_NR30_CHANNEL3_DAC_ENABLE => self.sample_generator.get_dac_enable(),
            HWREG_NR31_CHANNEL3_LENGTH_TIMER => self.sample_generator.get_length_timer(),
            HWREG_NR32_CHANNEL3_OUTPUT_LEVEL => self.sample_generator.get_output_level(),
            HWREG_NR33_CHANNEL3_PERIOD_LOW => self.sample_generator.get_period_low(),
            HWREG_NR33_CHANNEL3_PERIOD_HIGH_CONTROL => {
                self.sample_generator.get_period_high_control()
            }
            HWREG_NR41_CHANNEL4_LENGTH_TIMER => 0,
            HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE => 0,
            HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS => 0,
            HWREG_NR44_CHANNEL4_CONTROL => 0,
            HWREG_NR50_MASTER_VOLUME_VIN_PANNING => {
                let mut ret = 0;
                if self.vin[0] {
                    ret |= 0x80;
                }
                if self.vin[1] {
                    ret |= 0x08;
                }
                ret |= self.master_volume[0] << 4;
                ret |= self.master_volume[1] << 0;
                ret
            }
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
                let wg = &self.sample_generator;
                // TODO: 再生中に読み出すと0xFFを返す
                let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
                (wg.wave_ram[smpl + 0] << 4) | (wg.wave_ram[smpl + 1])
            }
            _ => {
                // それ以外は0を返す
                0
            }
        }
    }

    /// 1システムクロック単位で実行される処理
    pub fn system_clock_tick(&mut self, mem: &mut [u8]) {}

    /// 1ステレオサンプル出力
    /// 現在の出力サンプルを元に出力を計算します。サンプリングレート間隔で実行してください
    pub fn compute_output(&mut self, mem: &[u8]) -> [f32; 2] {
        [0.0, 0.0]
    }
}
