use crate::envelope_generator::*;
use crate::length_timer::*;
use crate::types::*;

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
    /// エンベロープ（ボリューム）ジェネレータ
    eg: EnvelopeGenerator,
    /// 長さタイマー
    length_timer: LengthTimer,
    /// 周期
    period: u16,
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
            duty: DutyRatio::Duty12_5,
            period: 0,
            length_enable: false,
            trigger: false,
            eg: EnvelopeGenerator::new(),
            length_timer: LengthTimer::new(),
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
    }

    /// 長さタイマー・デューティの設定
    pub fn set_length_timer_duty_cycle(&mut self, value: u8) {
        self.duty = match (value >> 5) & 0x3 {
            0 => DutyRatio::Duty12_5,
            1 => DutyRatio::Duty25,
            2 => DutyRatio::Duty50,
            3 => DutyRatio::Duty75,
            _ => unreachable!(),
        };
        self.length_timer.set_length_timer(value & 0x1F, 1);
    }

    /// ボリューム・エンベロープ設定
    pub fn set_volume_envelope(&mut self, value: u8) {
        self.eg.set_volume_envelope(value);
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
            DutyRatio::Duty12_5 => 0 << 5,
            DutyRatio::Duty25 => 1 << 5,
            DutyRatio::Duty50 => 2 << 5,
            DutyRatio::Duty75 => 3 << 5,
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
        ret |= (self.period & 0x7) as u8;
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
        // エンベロープジェネレータのリセット
        self.eg.reset();
    }

    /// 1システムクロック単位処理
    pub fn system_clock_tick(&mut self, mem: &mut [u8]) {
        // TODO

        // 長さタイマーが時間切れしていたら無効に
        if self.length_timer.expired {
            self.enable = false;
        }

        // エンベロープジェネレータの更新
        self.eg.system_clock_tick();

        // 長さタイマーの更新
        self.length_timer.system_clock_tick();
    }
}
