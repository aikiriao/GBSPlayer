use crate::envelope_generator::*;
use crate::types::*;

const APU_SOUND_LENGTH_PER_SYSTEM_CLOCKS: u32 = DMG_SYSTEM_CLOCK_HZ / APU_SOUND_LENGTH_HZ;

/// LFSRの長さ
#[derive(Debug)]
enum LFSRLength {
    /// 15bit
    Bit15,
    /// 7bit
    Bit7,
}

/// CH4: ノイズジェネレータ
#[derive(Debug)]
pub struct NoiseGenerator {
    /// 有効か？
    pub enable: bool,
    /// 持続時間
    initial_length_timer: u8,
    /// 残り時間
    length_timer: u8,
    /// 更新クロックの右シフト量
    clock_shift: u8,
    /// 更新クロックの除数
    clock_divider: u8,
    /// LFSRの長さはショートか
    lfsr_short_mode: bool,
    /// 持続時間有効か
    length_enable: bool,
    /// 再生要求フラグ
    trigger: bool,
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
}

impl NoiseGenerator {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            enable: false,
            initial_length_timer: 0,
            length_timer: 0,
            clock_shift: 0,
            clock_divider: 0,
            lfsr_short_mode: false,
            length_enable: false,
            trigger: false,
            lfsr: 0,
            lfsr_mask: 0,
            lfsr_clock_count: 0,
            lfsr_update_period: 0,
            eg: EnvelopeGenerator::new(),
        }
    }

    /// 長さタイマーの設定
    pub fn set_length_timer(&mut self, value: u8) {
        self.initial_length_timer = value;
    }

    /// ボリューム・エンベロープの設定
    pub fn set_volume_envelope(&mut self, value: u8) {
        self.eg.set_volume_envelope(value);
    }

    /// 更新頻度・ランダムネスの設定
    pub fn set_frequency_randomness(&mut self, value: u8) {
        self.clock_shift = ((value >> 4) & 0xF) as u8;
        self.lfsr_short_mode = (value & 0x08) != 0;
        self.clock_divider = value & 0x7;

        // システムクロック単位の更新頻度を計算
        const NOISE_GENERATOR_FREQUENCY_BASE: u32 = 262144;
        let freq = if self.clock_divider == 0 {
            (2 * NOISE_GENERATOR_FREQUENCY_BASE) >> (self.clock_shift as u32)
        } else {
            NOISE_GENERATOR_FREQUENCY_BASE
                / ((self.clock_divider as u32) * (1 << (self.clock_shift as u32)))
        };
        assert!(freq != 0);
        self.lfsr_update_period = DMG_SYSTEM_CLOCK_HZ / freq;

        // 更新用ビットマスク作成
        self.lfsr_mask = if self.lfsr_short_mode { 0x8080 } else { 0x8000 };
    }

    /// 制御フラグ設定
    pub fn set_control(&mut self, value: u8) {
        self.length_enable = (value & 0x40) != 0;
        self.trigger = (value & 0x80) != 0;
        if self.trigger {
            self.process_trigger();
        }
    }

    /// 長さタイマーの取得
    pub fn get_length_timer(&self) -> u8 {
        self.initial_length_timer
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
        ret |= if self.length_enable { 0x40 } else { 0 };
        ret |= if self.trigger { 0x80 } else { 0 };
        ret
    }

    /// トリガーON時の処理
    fn process_trigger(&mut self) {
        // チャンネルを有効に
        self.enable = true;
        // 長さタイマーが切れていたらリセット
        if self.length_timer == 0 {
            self.length_timer = self.initial_length_timer;
        }
        // エンベロープジェネレータのリセット
        self.eg.reset();
        // LFSRビットのリセット
        self.lfsr = 0;
    }

    /// 1システムクロック単位処理
    pub fn system_clock_tick(&mut self, mem: &mut [u8]) {
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

            // 出力書きこみ（右1bitシフトした結果のbit0を使うので変更前の1bitを使う）
            let prev_mem = mem[HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34];
            mem[HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34] = (prev_mem & 0x0F)
                | if lfsr1 != 0 {
                    self.eg.get_volume() << 4
                } else {
                    0
                };

            self.lfsr_clock_count -= self.lfsr_update_period;
        }

        // エンベロープジェネレータの更新
        self.eg.system_clock_tick();
    }
}
