use crate::sound_generator::*;
use crate::types::*;

/// 波形RAM領域終端アドレス
const HWREG_CHANNEL3_WAVE_PATTERN_RAM_END: usize = HWREG_CHANNEL3_WAVE_PATTERN_RAM_START + 16;
/// ゲームボーイのHPFの係数基準値
const DMG_HPF_COEF_BASE: f32 = 0.999958;
/// ゲームボーイカラーのHPFの係数基準値
#[allow(dead_code)]
const CGB_HPF_COEF_BASE: f32 = 0.998943;

/// Audio Processing Unit
pub struct APU {
    /// オーディオON/OFFフラグ
    audio_on: bool,
    /// 各チャンネルのパン
    ch_pan: [Pan; 4],
    /// マスターボリューム
    master_volume: [u8; 2],
    /// VIN（外部音声入力。未使用。現実のタイトルでも未使用）
    vin: [bool; 2],
    /// HPFの係数
    hpf_coef: f32,
    /// HPFの出力バッファ
    hpf_buffer: [f32; 2],
    /// 各チャンネルの最終出力値（4bitデジタル値）
    ch_out: [u8; 4],
    /// パルスジェネレータ
    pulse_generator: [PulseGenerator; 2],
    /// サンプルジェネレータ
    sample_generator: SampleGenerator,
    /// ノイズジェネレータ
    noise_generator: NoiseGenerator,
    /// クロックカウント
    clock_count: u32,
}

/// 4bitデジタル値を[-1,1]の範囲の浮動小数値に変換
fn dac(enable: bool, input: u8) -> f32 {
    if enable {
        // NOTE: 0は1, 0xFは-1にマッピングされる
        const INV7_5: f32 = 1.0 / 7.5;
        -(input as f32) * INV7_5 + 1.0
    } else {
        0.0
    }
}

impl APU {
    /// 出力サンプリングレートの設定
    pub fn set_sampling_rate(&mut self, sampling_rate: u32) {
        // HPFの係数を再計算
        self.hpf_coef = libm::pow(
            DMG_HPF_COEF_BASE as f64,
            (DMG_MASTER_CLOCK_HZ as f64) / (sampling_rate as f64),
        ) as f32;
    }

    /// HPFの適用
    fn apply_hpf(&mut self, input: &mut [f32; 2]) {
        for ch in 0..2 {
            let out = input[ch] - self.hpf_buffer[ch];
            self.hpf_buffer[ch] = input[ch] - out * self.hpf_coef;
            input[ch] = out;
        }
    }

    /// 1ステレオサンプル出力
    /// 現在の出力サンプルを元に出力を計算します。サンプリングレート間隔で実行してください
    pub fn compute_output(&mut self) -> [f32; 2] {
        let mut output = [0.0, 0.0];
        // 4ch分のON/OFFフラグ
        let ch_on = [
            self.pulse_generator[0].enable,
            self.pulse_generator[1].enable,
            self.sample_generator.enable,
            self.noise_generator.enable,
        ];
        // 4ch分の信号を読み取り・浮動小数化
        let ch_out = [
            dac(self.pulse_generator[0].dac_enable, self.ch_out[0]),
            dac(self.pulse_generator[1].dac_enable, self.ch_out[1]),
            dac(self.sample_generator.dac_enable, self.ch_out[2]),
            dac(self.noise_generator.dac_enable, self.ch_out[3]),
        ];
        // パン適用しつつステレオにミックス
        for ch in 0..4 {
            if ch_on[ch] {
                let out = ch_out[ch];
                match self.ch_pan[ch] {
                    Pan::Left => {
                        output[0] += out;
                    }
                    Pan::Right => {
                        output[1] += out;
                    }
                    Pan::Center => {
                        output[0] += out;
                        output[1] += out;
                    }
                    Pan::Ignore => {}
                }
            }
        }
        // マスターボリューム適用
        // NOTE:
        // - master_volume==7で入力をそのまま出力する。
        // - スケールは不明（ドキュメント化されてない）。
        for ch in 0..2 {
            output[ch] *= ((self.master_volume[ch] as f32) + 1.0) / 8.0;
        }
        // HPF適用
        self.apply_hpf(&mut output);
        output
    }
}

impl APUDevice for APU {
    /// コンストラクタ
    fn new() -> Self {
        Self {
            audio_on: false,
            master_volume: [0u8; 2],
            vin: [false; 2],
            ch_pan: [Pan::Center; 4],
            hpf_coef: DMG_HPF_COEF_BASE,
            hpf_buffer: [0.0; 2],
            ch_out: [0; 4],
            sample_generator: SampleGenerator::new(),
            pulse_generator: [PulseGenerator::new(), PulseGenerator::new()],
            noise_generator: NoiseGenerator::new(),
            clock_count: 0,
        }
    }

    /// レジスタへの書き込み
    fn write_register(&mut self, address: usize, value: u8) {
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
                self.pulse_generator[1].set_length_timer_duty_cycle(value);
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
            HWREG_NR41_CHANNEL4_LENGTH_TIMER => {
                self.noise_generator.set_length_timer(value);
            }
            HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE => {
                self.noise_generator.set_volume_envelope(value);
            }
            HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS => {
                self.noise_generator.set_frequency_randomness(value);
            }
            HWREG_NR44_CHANNEL4_CONTROL => {
                self.noise_generator.set_control(value);
            }
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
                // NOTE: 実機ではReadonlyだがパートごとに聞けるとうれしいので設定する
                self.pulse_generator[0].enable = (value & 0x1) != 0;
                self.pulse_generator[1].enable = (value & 0x2) != 0;
                self.sample_generator.enable = (value & 0x4) != 0;
                self.noise_generator.enable = (value & 0x8) != 0;
            }
            HWREG_CHANNEL3_WAVE_PATTERN_RAM_START..HWREG_CHANNEL3_WAVE_PATTERN_RAM_END => {
                let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
                self.sample_generator.set_wave_ram(smpl, value);
            }
            HWREG_PCM12_AUDIO_DIGITAL_OUTPUTS_12 | HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34 => {
                // 書き込みを無視
            }
            _ => {
                // それ以外は無視
            }
        }
    }

    /// レジスタからの読み出し
    fn read_register(&mut self, address: usize) -> u8 {
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
            HWREG_NR41_CHANNEL4_LENGTH_TIMER => self.noise_generator.get_length_timer(),
            HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE => self.noise_generator.get_volume_envelope(),
            HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS => {
                self.noise_generator.get_frequency_randomness()
            }
            HWREG_NR44_CHANNEL4_CONTROL => self.noise_generator.get_control(),
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
                ret |= if self.pulse_generator[0].enable {
                    0x01
                } else {
                    0
                };
                ret |= if self.pulse_generator[1].enable {
                    0x02
                } else {
                    0
                };
                ret |= if self.sample_generator.enable {
                    0x04
                } else {
                    0
                };
                ret |= if self.noise_generator.enable { 0x08 } else { 0 };
                ret
            }
            HWREG_CHANNEL3_WAVE_PATTERN_RAM_START..HWREG_CHANNEL3_WAVE_PATTERN_RAM_END => {
                let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
                self.sample_generator.get_wave_ram(smpl)
            }
            HWREG_PCM12_AUDIO_DIGITAL_OUTPUTS_12 => (self.ch_out[1] << 4) | (self.ch_out[0] << 0),
            HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34 => (self.ch_out[3] << 4) | (self.ch_out[2] << 0),
            _ => {
                // それ以外は0を返す
                0
            }
        }
    }

    /// 2MHzクロック単位処理
    fn clock_tick_2mhz(&mut self) {
        // クロック更新
        self.clock_count = self.clock_count.wrapping_add(1);

        // 出力があればハードウェアレジスタに書き込む
        // パルスジェネレータ
        if self.clock_count % 2 == 0 {
            if let Some(out) = self.pulse_generator[0].clock_tick_1mhz() {
                self.ch_out[0] = out;
            }
            if let Some(out) = self.pulse_generator[1].clock_tick_1mhz() {
                self.ch_out[1] = out;
            }
        }
        // サンプルジェネレータ
        if let Some(out) = self.sample_generator.clock_tick_2mhz() {
            self.ch_out[2] = out;
        }
        // ノイズジェネレータ
        if self.clock_count % 8 == 0 {
            if let Some(out) = self.noise_generator.clock_tick_256khz() {
                self.ch_out[3] = out;
            }
        }
    }
}
