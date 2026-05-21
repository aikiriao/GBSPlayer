use crate::types::*;

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
    /// タイムアウトカウント設定値
    saved_timeout: u16,
    /// クロックカウント
    clock_count: u32,
    /// 更新クロック周期
    update_period: u32,
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
            saved_timeout: 0,
            clock_count: 0,
            update_period: clock_tick_hz / APU_SOUND_LENGTH_HZ,
        }
    }

    /// 有効フラグの設定
    pub fn set_enable(&mut self, flag: bool) {
        self.enable = flag;
    }

    /// 長さタイマーの設定
    pub fn set_length_timer(&mut self, initial_timer: u8, timeout: u16) {
        self.initial_length_timer = initial_timer;
        self.saved_timeout = timeout;
    }

    /// 有効フラグの取得
    pub fn get_enable(&self) -> bool {
        self.enable
    }

    /// 長さタイマーの設定
    pub fn get_initial_length_timer(&self) -> u8 {
        self.initial_length_timer
    }

    /// トリガー時の処理
    pub fn process_trigger(&mut self) {
        self.length_timer = self.initial_length_timer as u16;
        self.expired = false;
        self.timeout = self.saved_timeout;
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
