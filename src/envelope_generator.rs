use crate::types::*;

/// エンベロープ（ボリューム）ジェネレータ
#[derive(Debug)]
pub struct EnvelopeGenerator {
    /// エンベロープ有効か
    enable: bool,
    /// ボリューム現在値（更新を簡易にするために符号付き）
    volume: i8,
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

impl EnvelopeGenerator {
    /// コンストラクタ
    pub fn new(clock_tick_hz: u32) -> Self {
        assert!(clock_tick_hz % APU_ENVELOPE_SWEEP_HZ == 0);

        Self {
            enable: false,
            volume: 0,
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
        self.volume_delta = if (value & 0x8) == 0 { -1 } else { 1 };
        self.volume_sweep_pace = value & 0x7;
        // 更新間隔クロックの設定
        self.volume_update_period = if self.volume_sweep_pace == 0 {
            self.enable = false;
            0
        } else {
            self.enable = true;
            (self.volume_sweep_pace as u32) * self.update_period
        };
    }

    /// ボリューム・エンベロープの取得
    pub fn get_volume_envelope(&self) -> u8 {
        let mut ret = 0;
        ret |= (self.initial_volume as u8) << 4;
        ret |= if self.volume_delta < 0 { 0x8 } else { 0 };
        ret |= self.volume_sweep_pace;
        ret
    }

    /// 現在のボリューム値の取得
    pub fn get_volume(&self) -> u8 {
        self.volume as u8
    }

    /// 内部状態リセット
    pub fn reset(&mut self) {
        // エンベロープタイマーのリセット
        self.clock_count = 0;
        // ボリュームのリセット
        self.volume = self.initial_volume;
    }

    /// クロック単位処理
    pub fn clock_tick(&mut self) {
        self.clock_count += 1;
        if self.enable && self.clock_count >= self.volume_update_period {
            self.volume += self.volume_delta;
            self.volume = self.volume.clamp(0, 0xF);
            self.clock_count -= self.volume_update_period;
        }
    }
}
