use crate::types::*;

/// パルスジェネレータの動作クロック
const PULSE_GENERATOR_CLOCK_HZ: u32 = 1048576;
/// サンプルジェネレータの動作クロック
const SAMPLE_GENERATOR_CLOCK_HZ: u32 = 2097152;
/// ノイズジェネレータの動作クロック
const NOISE_GENERATOR_CLOCK_HZ: u32 = 262144;
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

/// 長さタイマー
#[derive(Debug)]
pub struct LengthTimer {
    /// 有効か
    enable: bool,
    /// タイマー時間切れか？
    pub expired: bool,
    /// 持続時間
    initial_length_timer: u8,
    /// タイマーカウント
    length_timer: u16,
    /// タイムアウトカウント
    timeout: u16,
    /// クロックカウント
    clock_count: u32,
    /// 更新クロック周期
    update_period: u32,
}

/// エンベロープ（ボリューム）ジェネレータ
#[derive(Debug)]
pub struct EnvelopeGenerator {
    /// ボリューム現在値（更新を簡易にするために符号付き）
    volume: i8,
    /// ボリューム更新方向（false: 負、true: 正）
    volume_delta_dir: bool,
    /// ボリューム更新値
    volume_delta: i8,
    /// 初期ボリューム
    initial_volume: i8,
    /// ボリューム変更頻度
    volume_sweep_pace: u8,
    /// エンベロープ更新間隔クロックカウント
    volume_update_period: u32,
    /// クロックカウント
    clock_count: u32,
    /// 更新クロック周期
    update_period: u32,
}

/// CH1/CH2: パルス（矩形波）ジェネレータ
#[derive(Debug)]
pub struct PulseGenerator {
    /// 有効か？
    pub enable: bool,
    /// DAC有効か
    pub dac_enable: bool,
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
    /// 再生要求フラグ
    trigger: bool,
}

/// CH3: サンプルジェネレータ
#[derive(Debug)]
pub struct SampleGenerator {
    /// 有効か？
    pub enable: bool,
    /// DAC有効か
    pub dac_enable: bool,
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

/// CH4: ノイズジェネレータ
#[derive(Debug)]
pub struct NoiseGenerator {
    /// 有効か？
    pub enable: bool,
    /// DAC有効か
    pub dac_enable: bool,
    /// 再生要求フラグ
    trigger: bool,
    /// LFSR更新クロックの右シフト量
    clock_shift: u8,
    /// LFSR更新クロックの除数
    clock_divider: u8,
    /// LFSRの長さはショートか
    lfsr_short_mode: bool,
    /// LSFRのレジスタ
    lfsr: u16,
    /// LSFRの更新用ビットマスク
    lfsr_mask: u16,
    /// LSFRのシステムクロックカウンタ
    lfsr_clock_count: u32,
    /// LSFRの更新間隔
    lfsr_update_period: u32,
    /// エンベロープ（ボリューム）ジェネレータ
    eg: EnvelopeGenerator,
    /// 長さタイマー
    length_timer: LengthTimer,
}

impl LengthTimer {
    /// コンストラクタ
    pub fn new(clock_tick_hz: u32) -> Self {
        assert!(clock_tick_hz % APU_SOUND_LENGTH_HZ == 0);

        Self {
            enable: false,
            expired: true,
            initial_length_timer: 0,
            length_timer: 0,
            timeout: 0,
            clock_count: 0,
            update_period: clock_tick_hz / APU_SOUND_LENGTH_HZ,
        }
    }

    /// 長さタイマーの設定
    pub fn set_length_timer(&mut self, initial_timer: u8, timeout: u16) {
        self.initial_length_timer = initial_timer;
        self.timeout = timeout;
    }

    /// 長さタイマーの設定
    pub fn get_initial_length_timer(&self) -> u8 {
        self.initial_length_timer
    }

    /// トリガー時の処理
    pub fn process_trigger(&mut self) {
        self.length_timer = self.initial_length_timer as u16;
        self.expired = false;
        self.clock_count = 0;
    }

    /// クロック単位処理
    pub fn clock_tick(&mut self) {
        if self.enable && !self.expired {
            self.clock_count += 1;
            if self.clock_count >= self.update_period {
                self.length_timer += 1;
                if self.length_timer >= self.timeout {
                    self.expired = true;
                }
                self.clock_count -= self.update_period;
            }
        }
    }
}

impl EnvelopeGenerator {
    /// コンストラクタ
    pub fn new(clock_tick_hz: u32) -> Self {
        assert!(clock_tick_hz % APU_ENVELOPE_SWEEP_HZ == 0);

        Self {
            volume: 0,
            volume_delta_dir: false,
            volume_delta: 0,
            initial_volume: 0,
            volume_sweep_pace: 0,
            volume_update_period: 0,
            clock_count: 0,
            update_period: clock_tick_hz / APU_ENVELOPE_SWEEP_HZ,
        }
    }

    /// ボリューム・エンベロープの設定
    pub fn set_volume_envelope(&mut self, value: u8) {
        self.initial_volume = ((value >> 4) & 0xF) as i8;
        self.volume_delta_dir = (value & 0x8) != 0;
        self.volume_sweep_pace = value & 0x7;
    }

    /// ボリューム・エンベロープの取得
    pub fn get_volume_envelope(&self) -> u8 {
        let mut ret = 0;
        ret |= (self.initial_volume as u8) << 4;
        ret |= if self.volume_delta_dir { 0x8 } else { 0 };
        ret |= self.volume_sweep_pace & 0x7;
        ret
    }

    /// 現在のボリューム値の取得
    pub fn get_volume(&self) -> u8 {
        self.volume as u8
    }

    /// トリガー時の処理
    pub fn process_trigger(&mut self) {
        // エンベロープタイマーのリセット
        self.clock_count = 0;
        // ボリュームのリセット
        self.volume = self.initial_volume;
        // 更新間隔クロックの設定
        self.volume_update_period = (self.volume_sweep_pace as u32) * self.update_period;
        // ボリューム更新方向の設定
        self.volume_delta = if self.volume_delta_dir { 1 } else { -1 };
    }

    /// クロック単位処理
    pub fn clock_tick(&mut self) {
        if self.volume_update_period > 0 {
            self.clock_count += 1;
            if self.clock_count >= self.volume_update_period {
                self.volume += self.volume_delta;
                self.volume = self.volume.clamp(0, 0xF);
                self.clock_count -= self.volume_update_period;
            }
        }
    }
}

impl PulseGenerator {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            enable: false,
            dac_enable: false,
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
        (self.duty, self.pulse_table) = match (value >> 6) & 0x3 {
            0 => (DutyRatio::Duty125, &PULSE_TABLE_DUTY125),
            1 => (DutyRatio::Duty250, &PULSE_TABLE_DUTY250),
            2 => (DutyRatio::Duty500, &PULSE_TABLE_DUTY500),
            3 => (DutyRatio::Duty750, &PULSE_TABLE_DUTY750),
            _ => unreachable!(),
        };
        self.length_timer.set_length_timer(value & 0x3F, 64);
    }

    /// ボリューム・エンベロープ設定
    pub fn set_volume_envelope(&mut self, value: u8) {
        self.eg.set_volume_envelope(value);
        self.dac_enable = (value & 0xF8) != 0;
        if !self.dac_enable {
            self.enable = false;
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
        self.length_timer.enable = (value & 0x40) != 0;
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
        ret |= if self.length_timer.enable { 0x40 } else { 0 };
        ret |= if self.trigger { 0x80 } else { 0 };
        ret
    }

    /// トリガーON時の処理
    fn process_trigger(&mut self) {
        // チャンネルを有効に
        self.enable = true;
        // 長さタイマーが切れていたら再度トリガー処理
        if self.length_timer.expired {
            self.length_timer.process_trigger();
        }
        // 周期の設定
        self.sample_update_period = 2048 - self.period;
        // エンベロープジェネレータのトリガー処理
        self.eg.process_trigger();
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

        if self.enable {
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

            // 長さタイマーが時間切れしていたら止める
            if self.length_timer.expired {
                self.enable = false;
            }
        }

        out
    }
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
        };
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

impl NoiseGenerator {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            enable: false,
            dac_enable: false,
            trigger: false,
            clock_shift: 0,
            clock_divider: 0,
            lfsr_short_mode: false,
            lfsr: 0,
            lfsr_mask: 0,
            lfsr_clock_count: 0,
            lfsr_update_period: 0,
            eg: EnvelopeGenerator::new(NOISE_GENERATOR_CLOCK_HZ),
            length_timer: LengthTimer::new(NOISE_GENERATOR_CLOCK_HZ),
        }
    }

    /// 長さタイマーの設定
    pub fn set_length_timer(&mut self, value: u8) {
        self.length_timer.set_length_timer(value & 0x3F, 64);
    }

    /// ボリューム・エンベロープの設定
    pub fn set_volume_envelope(&mut self, value: u8) {
        self.eg.set_volume_envelope(value);
        self.dac_enable = (value & 0xF8) != 0;
        if !self.dac_enable {
            self.enable = false;
        }
    }

    /// 更新頻度・ランダムネスの設定
    pub fn set_frequency_randomness(&mut self, value: u8) {
        self.clock_shift = ((value >> 4) & 0xF) as u8;
        self.lfsr_short_mode = (value & 0x08) != 0;
        self.clock_divider = value & 0x7;
        // 更新用ビットマスク作成
        self.lfsr_mask = if self.lfsr_short_mode { 0x8080 } else { 0x8000 };
        // LFSRの更新頻度を計算
        self.lfsr_update_period = if self.clock_divider == 0 {
            (1 << (self.clock_shift as u32)) / 2
        } else {
            (self.clock_divider as u32) * (1 << (self.clock_shift as u32))
        };
    }

    /// 制御フラグ設定
    pub fn set_control(&mut self, value: u8) {
        self.length_timer.enable = (value & 0x40) != 0;
        self.trigger = (value & 0x80) != 0;
        if self.trigger {
            self.process_trigger();
        }
    }

    /// 長さタイマーの取得
    pub fn get_length_timer(&self) -> u8 {
        self.length_timer.get_initial_length_timer()
    }

    /// ボリューム・エンベロープの取得
    pub fn get_volume_envelope(&self) -> u8 {
        self.eg.get_volume_envelope()
    }

    /// 更新頻度・ランダムネスの取得
    pub fn get_frequency_randomness(&self) -> u8 {
        let mut ret = 0;
        ret |= self.clock_shift << 4;
        ret |= if self.lfsr_short_mode { 0x8 } else { 0 };
        ret |= self.clock_divider;
        ret
    }

    /// 制御フラグ設定
    pub fn get_control(&self) -> u8 {
        let mut ret = 0;
        ret |= if self.length_timer.enable { 0x40 } else { 0 };
        ret |= if self.trigger { 0x80 } else { 0 };
        ret
    }

    /// トリガーON時の処理
    fn process_trigger(&mut self) {
        // チャンネルを有効に
        self.enable = true;

        // 長さタイマーが切れていたら再度トリガー処理
        if self.length_timer.expired {
            self.length_timer.process_trigger();
        }
        // エンベロープジェネレータのトリガー処理
        self.eg.process_trigger();
        // LFSRビットのリセット
        self.lfsr = 0;
    }

    /// 1システムクロック単位処理
    pub fn clock_tick_256khz(&mut self) -> Option<u8> {
        let mut out = None;

        if self.enable {
            // カウンタ増加
            self.lfsr_clock_count += 1;
            if self.lfsr_clock_count >= self.lfsr_update_period {
                // LFSRの更新
                let lfsr0 = (self.lfsr >> 0) & 1;
                let lfsr1 = (self.lfsr >> 1) & 1;
                if lfsr0 == lfsr1 {
                    self.lfsr |= self.lfsr_mask;
                } else {
                    self.lfsr &= !(self.lfsr_mask);
                }
                self.lfsr >>= 1;

                // 出力（右1bitシフトした結果のbit0を使うので変更前の1bitを使う）
                out = Some(if lfsr1 != 0 { self.eg.get_volume() } else { 0 });

                self.lfsr_clock_count -= self.lfsr_update_period;
            }

            // 長さタイマーが時間切れしていたら無効に
            if self.length_timer.expired {
                self.enable = false;
            }

            // エンベロープジェネレータの更新
            self.eg.clock_tick();

            // 長さタイマーの更新
            self.length_timer.clock_tick();
        }

        out
    }
}
