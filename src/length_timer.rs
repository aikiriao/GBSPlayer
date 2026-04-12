use crate::types::*;

/// 長さタイマーの更新間隔
const APU_SOUND_LENGTH_PER_SYSTEM_CLOCKS: u32 = DMG_SYSTEM_CLOCK_HZ / APU_SOUND_LENGTH_HZ;

/// 長さタイマー
#[derive(Debug)]
pub struct LengthTimer {
    /// 有効か
    enable: bool,
    /// タイマー時間切れか？
    pub expired: bool,
    /// 持続時間
    initial_length_timer: u8,
    /// 残り時間
    length_timer: u8,
    /// タイマー増分
    timer_delta: u8,
    /// システムクロックカウント
    system_clock_count: u32,
}

impl LengthTimer {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            enable: false,
            expired: true,
            initial_length_timer: 0,
            length_timer: 0,
            timer_delta: 0,
            system_clock_count: 0,
        }
    }

    /// 有効フラグの設定
    pub fn set_enable(&mut self, flag: bool) {
        self.enable = flag;
    }

    /// 長さタイマーの設定
    pub fn set_length_timer(&mut self, initial_timer: u8, timer_delta: u8) {
        self.initial_length_timer = initial_timer;
        self.timer_delta = timer_delta;
    }

    /// 有効フラグの取得
    pub fn get_enable(&self) -> bool {
        self.enable
    }

    /// 長さタイマーの設定
    pub fn get_initial_length_timer(&self) -> u8 {
        self.initial_length_timer
    }

    /// タイマーリセット
    pub fn reset(&mut self) {
        self.length_timer = self.initial_length_timer;
        self.expired = false;
        self.system_clock_count = 0;
    }

    /// 1システムクロック単位処理
    pub fn system_clock_tick(&mut self) {
        self.system_clock_count += 1;
        if self.enable
            && !self.expired
            && self.system_clock_count >= APU_SOUND_LENGTH_PER_SYSTEM_CLOCKS
        {
            let (timer, overflow) = self.length_timer.overflowing_add(self.timer_delta);
            if overflow {
                self.expired = true;
            } else {
                self.length_timer = timer;
            }
            self.system_clock_count -= APU_SOUND_LENGTH_PER_SYSTEM_CLOCKS;
        }
    }
}
