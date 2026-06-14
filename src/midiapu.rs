use crate::sound_generator::*;
use crate::types::*;
use libm;

/// 1度に出力できる最大のMIDIメッセージ数
pub const MAX_NUM_MIDI_OUTPUT_MESSAGES: usize = 100;

/// パーカッションパートのチャンネル
const MIDI_PERCUSSION_CHANNEL: u8 = 0x09;

/// MIDIメッセージ：ノートオン
const MIDIMSG_NOTE_ON: u8 = 0x90;
/// MIDIメッセージ：ノートオフ
const MIDIMSG_NOTE_OFF: u8 = 0x80;
/// MIDIメッセージ：コントロールチェンジ
const MIDIMSG_CONTROL_CHANGE: u8 = 0xB0;
/// MIDIメッセージ：プログラムチェンジ
const MIDIMSG_PROGRAM_CHANGE: u8 = 0xC0;
/// MIDIメッセージ：ピッチベンド
const MIDIMSG_PITCH_BEND: u8 = 0xE0;

/// MIDIコントロールチェンジ：チャンネルボリューム
const MIDICC_CHANNEL_VOLUME: u8 = 0x07;
/// MIDIコントロールチェンジ：パンポット
const MIDICC_PANPOT: u8 = 0x0A;
/// MIDIコントロールチェンジ：エクスプレッション
const MIDICC_EXPRESSION: u8 = 0x0B;
/// MIDIコントロールチェンジ：RPN LSB
const MIDICC_RPN_LSB: u8 = 0x64;
/// MIDIコントロールチェンジ：RPN MSB
const MIDICC_RPN_MSB: u8 = 0x65;
/// MIDIコントロールチェンジ：RPN データエントリーLSB
const MIDICC_RPN_DATA_ENTRY_LSB: u8 = 0x06;
/// MIDIコントロールチェンジ：RPN データエントリーMSB
const MIDICC_RPN_DATA_ENTRY_MSB: u8 = 0x26;
/// MIDIコントロールチェンジ：エフェクト1デプス
const MIDICC_EFFECT1_DEPTH: u8 = 0x5B;

/// ノートオン時のピッチベンド設定値
const NOTEON_PITCH_BEND: u16 = 8192;
/// ノートオン時のベロシティ
const NOTEON_VELOCITY: u8 = 127;

/// 更新間隔
const MIDIAPU_DEFAULT_UPDATE_PERIOD_HZ: u64 = 64;

/// ボリュームカーブ
#[derive(Copy, Clone, Debug)]
enum MIDIVolumeCurve {
    /// 平方根
    /// MIDIのボリューム値の2乗がSPCの振幅に比例するよう(GMの推奨値)に変換
    SquareRoot,
    /// 対数
    Log,
    /// 線形
    /// ゲインをそのままボリューム値に変換。ほとんどの場合音圧が小さくなるため非推奨だがデバッグに有効
    Linear,
}

/// MIDIメッセージ
#[derive(Debug, Clone, Copy)]
pub struct MIDIMessage {
    /// バイトデータ
    pub data: [u8; 3],
    /// データ長
    pub length: usize,
    /// 2MHz単位のクロックティック数
    pub clock_tick_2mhz: u64,
}

/// MIDI出力
#[derive(Debug, Clone)]
pub struct MIDIOutput {
    /// メッセージ列
    pub messages: [MIDIMessage; MAX_NUM_MIDI_OUTPUT_MESSAGES],
    /// メッセージ数
    pub num_messages: usize,
}

/// チャンネルステータス
#[derive(Debug, Clone, Copy)]
struct MIDIChannelStatus {
    /// ミュートされているか？
    mute: bool,
    /// ノートオンされているか？
    noteon: bool,
    /// ノート番号
    noteno: u8,
    /// ピッチベンド
    pitch_bend: u16,
    /// ピッチベンドの基準ピッチ（最後に発声した音のピッチ）
    pitch_bend_base: f32,
    /// エクスプレッション
    expression: u8,
    /// パン
    pan: u8,
    /// ボリューム
    volume: u8,
}

/// MIDIを出力するAudio Processing Unit
pub struct MIDIAPU {
    /// オーディオON/OFFフラグ
    audio_on: bool,
    /// 各チャンネルのパン
    ch_pan: [Pan; 4],
    /// マスターボリューム
    master_volume: [u8; 2],
    /// パルスジェネレータ
    pulse_generator: [PulseGenerator; 2],
    /// サンプルジェネレータ
    sample_generator: SampleGenerator,
    /// ノイズジェネレータ
    noise_generator: NoiseGenerator,
    /// MIDIメッセージバッファ
    message_buffer: MIDIOutput,
    /// APUから見たMIDIチャンネルの状態
    apu_ch_status: [MIDIChannelStatus; 4],
    /// MIDIステータスバイト
    status_byte: u8,
    /// MIDIの更新間隔
    midi_update_period_cycles: u64,
    /// クロックカウント
    clock_count: u64,
}

/// ピッチ・ピッチ基準値からピッチベンド設定値の計算
fn pitch_to_pitch_bend(pitch: f32, pitch_base: f32, sensitivity: u8) -> u16 {
    let max_semitone = sensitivity as f32;
    // [-max_semitone,max_semitone]半音を[-8192,8192]に対応付ける
    let pitchbend_ratio = libm::log2f(pitch / pitch_base) * 12.0 / max_semitone;
    (libm::roundf((pitchbend_ratio * 8192.0).clamp(-8192.0, 8191.0)) as i16 + 8192) as u16
}

/// 周波数からノート番号を計算
fn herz_to_noteno(hz: f32) -> u8 {
    libm::roundf(69.0 + 12.0 * libm::log2f(hz / 440.0)).clamp(0.0, 127.0) as u8
}

/// ゲームボーイのボリューム設定値[0,15]をMIDIのボリューム設定値に変換
fn volume_to_midi_volume(volume_curve: MIDIVolumeCurve, gb_volume: u8) -> u8 {
    // [0, 127]の範囲に変換
    let gain = gb_volume as f32 * 127.0 / 15.0;

    // ボリュームカーブに応じて変換
    let volume = match volume_curve {
        MIDIVolumeCurve::SquareRoot => libm::sqrtf(gain * 127.0),
        MIDIVolumeCurve::Log => {
            const NORMALIZE_FACTOR: f32 = 59.89151875002212; // 126 / log10(127)
            if gain > 0.0 {
                NORMALIZE_FACTOR * libm::log10f(gain) + 1.0
            } else {
                0.0
            }
        }
        MIDIVolumeCurve::Linear => gain,
    };

    libm::roundf(volume).clamp(0.0, 127.0) as u8
}

impl MIDIAPU {
    /// チャンネルメッセージを追加
    fn push_channel_message(&mut self, mute: bool, data: &[u8]) {
        if mute {
            return;
        }

        let length = data.len();
        let message = &mut self.message_buffer.messages[self.message_buffer.num_messages];

        assert!(length <= 3);
        assert!(self.message_buffer.num_messages < MAX_NUM_MIDI_OUTPUT_MESSAGES);

        // 先頭1バイト（ステータスバイト）を見て直前と同じならばステータスバイトを省略（ランニングステータス）
        if self.status_byte == data[0] {
            message.data[..(length - 1)].copy_from_slice(&data[1..length]);
            message.length = length - 1;
        } else {
            message.data[..length].copy_from_slice(&data);
            message.length = length;
        }
        message.clock_tick_2mhz = self.clock_count;
        self.message_buffer.num_messages += 1;

        self.status_byte = data[0];
    }

    /// MIDIメッセージのをシステムサイクル単位で指定
    pub fn set_midi_update_period_cycles(&mut self, cycles: u64) {
        self.midi_update_period_cycles = cycles;
    }

    /// パルスジェネレータのノートオン処理
    fn noteon_pulse_generator(&mut self, ch: u8) {
        let ch_status = self.apu_ch_status[ch as usize];
        // ノートオフが漏れている場合はノートオフを送信
        if ch_status.noteon {
            self.push_channel_message(
                ch_status.mute,
                &[MIDIMSG_NOTE_OFF | ch, ch_status.noteno, 0],
            );
        }

        // エクスプレッション
        let volume = self.pulse_generator[ch as usize].get_volume();
        let expression = volume_to_midi_volume(MIDIVolumeCurve::SquareRoot, volume);
        if expression != ch_status.expression {
            self.push_channel_message(
                ch_status.mute,
                &[MIDIMSG_CONTROL_CHANGE | ch, MIDICC_EXPRESSION, expression],
            );
        }

        // ピッチベンド（基準ピッチベンド値から変わっていれば）
        if NOTEON_PITCH_BEND != ch_status.pitch_bend {
            self.push_channel_message(
                ch_status.mute,
                &[
                    MIDIMSG_PITCH_BEND | ch,
                    (NOTEON_PITCH_BEND & 0x7F) as u8,        // LSB
                    ((NOTEON_PITCH_BEND >> 7) & 0x7F) as u8, // MSB
                ],
            );
        }

        // ノートオン送信
        let pitch = self.pulse_generator[ch as usize].get_pitch_frequency();
        let noteno = herz_to_noteno(pitch);
        self.push_channel_message(
            ch_status.mute,
            &[MIDIMSG_NOTE_ON | ch, noteno, NOTEON_VELOCITY],
        );

        // チャンネル状態更新
        let ch_status = &mut self.apu_ch_status[ch as usize];
        ch_status.noteon = true;
        ch_status.noteno = noteno;
        ch_status.pitch_bend_base = pitch;
        ch_status.pitch_bend = NOTEON_PITCH_BEND;
        ch_status.expression = expression;
    }

    /// 状態変更に基づいてMIDIメッセージバッファを更新
    fn update_midi_message_buffer(&mut self) {
        // ノートオフ
        // パルスジェネレータ
        for ch in 0..2 {
            let ch_status = self.apu_ch_status[ch];
            if ch_status.noteon && !self.pulse_generator[ch].enable {
                self.push_channel_message(
                    ch_status.mute,
                    &[MIDIMSG_NOTE_OFF | (ch as u8), ch_status.noteno, 0],
                );
                self.apu_ch_status[ch].noteon = false;
            }
        }

        // 再生パラメータ更新
        // パルスジェネレータ
        for ch in 0..2 {
            let ch_status = self.apu_ch_status[ch];
            if self.apu_ch_status[ch].noteon {
                // エクスプレッション
                let expression = volume_to_midi_volume(
                    MIDIVolumeCurve::SquareRoot,
                    self.pulse_generator[ch].get_volume(),
                );
                if expression != ch_status.expression {
                    self.push_channel_message(
                        ch_status.mute,
                        &[
                            MIDIMSG_CONTROL_CHANGE | (ch as u8),
                            MIDICC_EXPRESSION,
                            expression,
                        ],
                    );
                    self.apu_ch_status[ch].expression = expression;
                }

                // ピッチベンド
                let pitch_bend = pitch_to_pitch_bend(
                    self.pulse_generator[ch].get_pitch_frequency(),
                    ch_status.pitch_bend_base,
                    12, // TODO: 仮
                );
                if pitch_bend != ch_status.pitch_bend {
                    self.push_channel_message(
                        ch_status.mute,
                        &[
                            MIDIMSG_PITCH_BEND | (ch as u8),
                            (pitch_bend & 0x7F) as u8,        // LSB
                            ((pitch_bend >> 7) & 0x7F) as u8, // MSB
                        ],
                    );
                    self.apu_ch_status[ch].pitch_bend = pitch_bend;
                }
            }
        }
    }
}

impl APUDevice for MIDIAPU {
    type Output = MIDIOutput;

    /// コンストラクタ
    fn new() -> Self {
        Self {
            audio_on: false,
            ch_pan: [Pan::Center; 4],
            master_volume: [0u8; 2],
            sample_generator: SampleGenerator::new(),
            pulse_generator: [PulseGenerator::new(), PulseGenerator::new()],
            noise_generator: NoiseGenerator::new(),
            apu_ch_status: [MIDIChannelStatus {
                mute: false,
                noteon: false,
                noteno: 0,
                pitch_bend: NOTEON_PITCH_BEND,
                pitch_bend_base: 0.0,
                expression: 0,
                pan: 0,
                volume: 0,
            }; 4],
            message_buffer: MIDIOutput {
                messages: [MIDIMessage {
                    data: [0; 3],
                    length: 0,
                    clock_tick_2mhz: 0,
                }; MAX_NUM_MIDI_OUTPUT_MESSAGES],
                num_messages: 0,
            },
            status_byte: 0,
            midi_update_period_cycles: 2 * (1 << 20) / MIDIAPU_DEFAULT_UPDATE_PERIOD_HZ,
            clock_count: 0,
        }
    }

    /// 出力レート設定
    fn set_sampling_rate(&mut self, _sampling_rate: u32) {
        // 何もしない
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
                // ノートオン
                if value & 0x80 != 0 {
                    self.noteon_pulse_generator(0);
                }
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
                // ノートオン
                if value & 0x80 != 0 {
                    self.noteon_pulse_generator(1);
                }
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
                    // パン・ボリュームが変わっていたらメッセージ送信
                    let (pan, volume) = match self.ch_pan[ch] {
                        Pan::Center => (64, 127),
                        Pan::Left => (0, 127),
                        Pan::Right => (127, 127),
                        Pan::Ignore => (64, 0),
                    };
                    if self.apu_ch_status[ch].pan != pan {
                        self.push_channel_message(
                            self.apu_ch_status[ch].mute,
                            &[MIDIMSG_CONTROL_CHANGE | ch as u8, MIDICC_PANPOT, pan],
                        );
                    }
                    if self.apu_ch_status[ch].volume != volume {
                        self.push_channel_message(
                            self.apu_ch_status[ch].mute,
                            &[
                                MIDIMSG_CONTROL_CHANGE | ch as u8,
                                MIDICC_CHANNEL_VOLUME,
                                volume,
                            ],
                        );
                    }
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
            HWREG_CHANNEL3_WAVE_PATTERN_RAM_START..=HWREG_CHANNEL3_WAVE_PATTERN_RAM_END => {
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
                // vin0, vin1はfalse
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
            HWREG_CHANNEL3_WAVE_PATTERN_RAM_START..=HWREG_CHANNEL3_WAVE_PATTERN_RAM_END => {
                let smpl = 2 * (address - HWREG_CHANNEL3_WAVE_PATTERN_RAM_START);
                self.sample_generator.get_wave_ram(smpl)
            }
            HWREG_PCM12_AUDIO_DIGITAL_OUTPUTS_12 | HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34 => {
                // 読み込みに0を返す
                0
            }
            _ => {
                // それ以外は0を返す
                0
            }
        }
    }

    /// 2MHzクロック単位処理
    fn clock_tick_2mhz(&mut self) {
        // 各サウンドジェネレータのクロックティック処理（音声出力は破棄）

        // パルスジェネレータ
        if self.clock_count % 2 == 0 {
            let _ = self.pulse_generator[0].clock_tick_1mhz();
            let _ = self.pulse_generator[1].clock_tick_1mhz();
        }

        // サンプルジェネレータ
        let _ = self.sample_generator.clock_tick_2mhz();

        // ノイズジェネレータ
        if self.clock_count % 8 == 0 {
            let _ = self.noise_generator.clock_tick_256khz();
        }

        // MIDIメッセージ更新
        if self.clock_count % self.midi_update_period_cycles == 0 {
            self.update_midi_message_buffer();
        }

        // クロック更新
        self.clock_count = self.clock_count.wrapping_add(1);
    }

    /// MIDIメッセージの出力
    fn compute_output(&mut self) -> MIDIOutput {
        let mut ret = MIDIOutput {
            messages: [MIDIMessage {
                data: [0; 3],
                length: 0,
                clock_tick_2mhz: 0,
            }; MAX_NUM_MIDI_OUTPUT_MESSAGES],
            num_messages: 0,
        };

        // 結果にコピー
        let num_out_msgs = self.message_buffer.num_messages;
        ret.messages[..num_out_msgs].copy_from_slice(&self.message_buffer.messages[..num_out_msgs]);
        ret.num_messages = num_out_msgs;

        // メッセージ数更新
        self.message_buffer.num_messages = 0;

        ret
    }
}
