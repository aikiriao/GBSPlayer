use crate::envelope_generator::*;
use crate::length_timer::*;

/// ノイズジェネレータの動作クロック
const NOISE_GENERATOR_CLOCK_HZ: u32 = 262144;

/// CH4: ノイズジェネレータ
#[derive(Debug)]
pub struct NoiseGenerator {
    /// 有効か？
    pub enable: bool,
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

impl NoiseGenerator {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            enable: false,
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
        self.length_timer.set_length_timer(value, 1 << 2); // FIXME: 他CHの4倍の速さで更新
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

        // LFSRの更新頻度を計算
        self.lfsr_update_period = if self.clock_divider == 0 {
            (2 * NOISE_GENERATOR_CLOCK_HZ) >> (self.clock_shift as u32)
        } else {
            NOISE_GENERATOR_CLOCK_HZ
                / ((self.clock_divider as u32) * (1 << (self.clock_shift as u32)))
        };

        // 更新用ビットマスク作成
        self.lfsr_mask = if self.lfsr_short_mode { 0x8080 } else { 0x8000 };
    }

    /// 制御フラグ設定
    pub fn set_control(&mut self, value: u8) {
        self.length_timer.set_enable((value & 0x40) != 0);
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
        ret |= if self.length_timer.get_enable() {
            0x40
        } else {
            0
        };
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
        // LFSRビットのリセット
        self.lfsr = 0;
    }

    /// 1システムクロック単位処理
    pub fn clock_tick_256khz(&mut self) -> Option<u8> {
        let mut out = None;

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

        out
    }
}
