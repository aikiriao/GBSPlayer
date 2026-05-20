use crate::envelope_generator::*;
use crate::length_timer::*;
use crate::types::*;

/// パルスジェネレータの動作クロック
const PULSE_GENERATOR_CLOCK_HZ: u32 = 1048576;
/// スイープの更新頻度
const SWEEP_UPDATE_PERIOD_UNIT_HZ: u32 = 128;

/// デューティ比に対応する矩形波テーブル
const PULSE_TABLE_DUTY125: [u8; 8] = [1, 1, 1, 1, 1, 1, 1, 0];
const PULSE_TABLE_DUTY250: [u8; 8] = [0, 1, 1, 1, 1, 1, 1, 0];
const PULSE_TABLE_DUTY500: [u8; 8] = [0, 1, 1, 1, 1, 0, 0, 0];
const PULSE_TABLE_DUTY750: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 1];

/// デューティ比
#[derive(Debug, Clone, Copy)]
enum DutyRatio {
    /// 12.5%
    Duty125,
    /// 25%
    Duty250,
    /// 50%
    Duty500,
    /// 75%
    Duty750,
}

/// CH1/CH2: パルス（矩形波）ジェネレータ
#[derive(Debug)]
pub struct PulseGenerator {
    /// 有効か？
    pub enable: bool,
    /// 周期変更頻度
    period_sweep_pace: u8,
    /// 周期変更方向
    period_sweep_direction: SweepDirection,
    /// 周期変更ステップ
    period_sweep_step: u8,
    /// 矩形波のデューティ比
    duty: DutyRatio,
    /// 矩形波テーブル
    pulse_table: &'static [u8; 8],
    /// 矩形波テーブルのインデックス
    pulse_table_index: usize,
    /// エンベロープ（ボリューム）ジェネレータ
    eg: EnvelopeGenerator,
    /// 長さタイマー
    length_timer: LengthTimer,
    /// 周期
    period: u16,
    /// 周期の変更があったか？
    period_changed: bool,
    /// サンプル更新間隔
    sample_update_period: u16,
    /// サンプル更新のためのシステムクロックカウンタ
    sample_update_counter: u16,
    /// 周期更新するか
    period_update_enable: bool,
    /// 周期更新間隔
    period_update_period: u16,
    /// 周期更新のためのシステムクロックカウンタ
    period_update_counter: u16,
    /// 持続時間有効か
    length_enable: bool,
    /// 再生要求フラグ
    trigger: bool,
}

impl PulseGenerator {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            enable: false,
            period_sweep_pace: 0,
            period_sweep_direction: SweepDirection::Positive,
            period_sweep_step: 0,
            duty: DutyRatio::Duty125,
            pulse_table: &PULSE_TABLE_DUTY125,
            pulse_table_index: 0,
            period: 0,
            period_changed: false,
            sample_update_period: 0,
            sample_update_counter: 0,
            period_update_enable: false,
            period_update_period: 0,
            period_update_counter: 0,
            length_enable: false,
            trigger: false,
            eg: EnvelopeGenerator::new(PULSE_GENERATOR_CLOCK_HZ),
            length_timer: LengthTimer::new(PULSE_GENERATOR_CLOCK_HZ),
        }
    }

    /// スイープの設定
    pub fn set_sweep(&mut self, value: u8) {
        self.period_sweep_pace = (value >> 4) & 0x7;
        self.period_sweep_direction = if (value & 0x8) == 0 {
            SweepDirection::Positive
        } else {
            SweepDirection::Negative
        };
        self.period_sweep_step = value & 0x7;
        if self.period_sweep_pace > 0 {
            self.period_update_period = (PULSE_GENERATOR_CLOCK_HZ
                / (self.period_sweep_pace as u32 * SWEEP_UPDATE_PERIOD_UNIT_HZ))
                as u16;
            self.period_update_enable = true;
        } else {
            self.period_update_enable = false;
        }
    }

    /// 長さタイマー・デューティの設定
    pub fn set_length_timer_duty_cycle(&mut self, value: u8) {
        (self.duty, self.pulse_table) = match (value >> 5) & 0x3 {
            0 => (DutyRatio::Duty125, &PULSE_TABLE_DUTY125),
            1 => (DutyRatio::Duty250, &PULSE_TABLE_DUTY250),
            2 => (DutyRatio::Duty500, &PULSE_TABLE_DUTY500),
            3 => (DutyRatio::Duty750, &PULSE_TABLE_DUTY750),
            _ => unreachable!(),
        };
        self.length_timer.set_length_timer(value & 0x1F, 64);
    }

    /// ボリューム・エンベロープ設定
    pub fn set_volume_envelope(&mut self, value: u8) {
        self.eg.set_volume_envelope(value);
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

    /// スイープ設定値の取得
    pub fn get_sweep(&self) -> u8 {
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
    pub fn get_length_timer_duty_cycle(&self) -> u8 {
        let mut ret = 0;
        ret |= match self.duty {
            DutyRatio::Duty125 => 0 << 5,
            DutyRatio::Duty250 => 1 << 5,
            DutyRatio::Duty500 => 2 << 5,
            DutyRatio::Duty750 => 3 << 5,
        };
        ret |= self.length_timer.get_initial_length_timer();
        ret
    }

    /// ボリューム・エンベロープ設定値の取得
    pub fn get_volume_envelope(&self) -> u8 {
        self.eg.get_volume_envelope()
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

    /// トリガーON時の処理
    fn process_trigger(&mut self) {
        // チャンネルを有効に
        self.enable = true;
        // 長さタイマーが切れていたらリセット
        if self.length_timer.expired {
            self.length_timer.reset();
        }
        // 周期の設定
        self.sample_update_period = 2048 - self.period;
        // エンベロープジェネレータのリセット
        self.eg.reset();
    }

    /// 周期更新処理
    fn update_period(&mut self) {
        if self.period_update_enable {
            self.period_update_counter += 1;
            if self.period_update_counter >= self.period_update_period {
                let overflow;
                let delta = self.period >> (self.period_sweep_step as u16);
                (self.period, overflow) = match self.period_sweep_direction {
                    SweepDirection::Positive => self.period.overflowing_add(delta),
                    SweepDirection::Negative => self.period.overflowing_sub(delta),
                };
                // オーバーフロー時はチャンネルを無効にする
                if overflow {
                    self.enable = false;
                    self.period_sweep_pace = 0;
                    self.period_update_enable = false;
                }
                self.period_update_counter -= self.period_update_period;
            }
        }
    }

    /// クロック単位処理
    pub fn clock_tick_1mhz(&mut self) -> Option<u8> {
        let mut out = None;

        // カウンタ増加
        self.sample_update_counter += 1;

        // サンプル更新
        if self.sample_update_counter >= self.sample_update_period {
            out = Some(self.pulse_table[self.pulse_table_index] * self.eg.get_volume());
            self.pulse_table_index = (self.pulse_table_index + 1) & 0x7;
            self.sample_update_counter -= self.sample_update_period;
            // 周期の反映はサンプル出力後
            if self.period_changed {
                self.sample_update_period = 2048 - self.period;
                self.period_changed = false;
            }
        }

        // 周期更新
        self.update_period();

        // エンベロープジェネレータの更新
        self.eg.clock_tick();

        // 長さタイマーの更新
        self.length_timer.clock_tick();

        // 長さタイマーが時間切れしていたりエンベロープジェネレータが停止していたら止める
        if self.length_timer.expired || !self.eg.enable {
            self.enable = false;
        }

        out
    }
}
