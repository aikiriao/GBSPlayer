use crate::gbs_file::*;
use crate::sm83::*;
use crate::types::*;

/// init/playの戻り先アドレス
const GBSPLAYER_INIT_PLAY_RETURN_ADDRESS: u16 = 0x0000;

pub struct GBSPlayer<R, A>
where
    R: AsRef<[u8]>,
    A: APUDevice,
{
    gbs_header: GBSFileHeader,
    cpu: SM83<R, A>,
    sampling_rate: u32,
    elapsed_cycles: u32,
}

impl<R, A> GBSPlayer<R, A>
where
    R: AsRef<[u8]>,
    A: APUDevice,
{
    /// コンストラクタ
    pub fn new(gbs_header: &GBSFileHeader, rom: R, sampling_rate: u32) -> Self {
        Self {
            gbs_header: gbs_header.clone(),
            cpu: SM83::new(rom),
            sampling_rate: sampling_rate,
            elapsed_cycles: 0,
        }
    }

    /// ロード
    pub fn load(&mut self) {
        let mut load_size = self.cpu.rom.as_ref().len();
        assert!(load_size >= 0x4000);
        assert!((load_size % 0x4000) == 0);

        // 読み出しサイズがROM領域を飛び出ていたら制限
        // 残りのメモリはROMバンク切り替えでアクセスする
        if load_size >= 0x8000 {
            load_size = 0x8000;
        }

        self.cpu.mem[..load_size].copy_from_slice(&self.cpu.rom.as_ref()[..load_size]);
    }

    /// スタックにデータをPUSH（SM83にも同様の関数はあるがSM83の関数は隠蔽したいため自前実装）
    fn push_stack(&mut self, value: u8) {
        self.cpu.regs.sp = self.cpu.regs.sp.wrapping_sub(1);
        self.cpu.write_mem_u8(self.cpu.regs.sp as usize, value);
    }

    /// playルーチンの割り込みが発生しているか判定
    fn check_play_interrupt(&self) -> bool {
        // 判定用フラグの取得
        let flag = if (self.cpu.mem[HWREG_TAC_TIMER_CONTROL] & 0x4) == 0 {
            // タイマー割り込み無効ならV-blank割り込みを使用
            SM83_INTERRUPT_FLAG_VBLANK
        } else {
            SM83_INTERRUPT_FLAG_TIMER
        };

        (self.cpu.mem[HWREG_IF_INTERRUPT_FLAG] & flag) != 0
    }

    /// playルーチンの割り込みフラグをクリア
    fn clear_play_interrupt_flag(&mut self) {
        // 判定用フラグの取得
        let flag = if (self.cpu.mem[HWREG_TAC_TIMER_CONTROL] & 0x4) == 0 {
            // タイマー割り込み無効ならV-blank割り込みを使用
            SM83_INTERRUPT_FLAG_VBLANK
        } else {
            SM83_INTERRUPT_FLAG_TIMER
        };

        // 割り込みフラグクリア
        self.cpu.mem[HWREG_IF_INTERRUPT_FLAG] &= !flag;
    }

    /// 初期化
    pub fn init(&mut self, song_number: u8) {
        // レジスタ・フラグ・RAMのクリア
        self.cpu.reset_registers();
        self.cpu.reset_flags();
        self.cpu.clear_ram();

        // アキュムレータに曲番号
        assert!(song_number < self.gbs_header.num_songs);
        self.cpu.regs.a = song_number;
        // スタックポインタ初期化
        self.cpu.regs.sp = self.gbs_header.stack_pointer;

        // タイマー初期化
        self.cpu
            .write_mem_u8(HWREG_TMA_TIMER_MODULO, self.gbs_header.timer_modulo);
        self.cpu
            .write_mem_u8(HWREG_TAC_TIMER_CONTROL, self.gbs_header.timer_control);

        // サンプリングレート設定
        self.cpu.set_audio_sampling_rate(self.sampling_rate);

        // 経過クロックカウントをリセット
        self.elapsed_cycles = 0;

        // 初期化（initアドレスに飛ぶ）
        self.push_stack(((GBSPLAYER_INIT_PLAY_RETURN_ADDRESS >> 8) & 0xFF) as u8);
        self.push_stack(((GBSPLAYER_INIT_PLAY_RETURN_ADDRESS >> 0) & 0xFF) as u8);
        self.cpu.regs.pc = self.gbs_header.init_address;
        while self.cpu.regs.pc != GBSPLAYER_INIT_PLAY_RETURN_ADDRESS {
            let _ = self.cpu.execute_step();
        }
        // ここから時刻0で再生開始したいため、タイマーをリセット
        self.cpu.reset_timers();
    }

    /// 1ステレオサンプル出力
    pub fn output_audio_sample(&mut self) -> A::Output {
        while self.elapsed_cycles < DMG_SYSTEM_CLOCK_HZ {
            // 命令実行
            let (_, cycle) = self.cpu.execute_step();
            // サイクルカウントを累積
            self.elapsed_cycles += cycle as u32 * self.sampling_rate;
            // 待機処理でplay/initアドレスに来てしまったら戻す
            if self.cpu.regs.pc == self.gbs_header.play_address
                || self.cpu.regs.pc == self.gbs_header.init_address
                || self.cpu.regs.pc == self.gbs_header.load_address
            {
                self.cpu.regs.pc = GBSPLAYER_INIT_PLAY_RETURN_ADDRESS;
            }
            // 割り込み処理
            if self.check_play_interrupt() {
                // 割り込みフラグをクリア
                self.clear_play_interrupt_flag();
                // スタックポインタが進みすぎていたら初期値に戻す
                // （割り込みのたびにスタックポインタが進んでいる場合、放置するとメモリ破壊が起きる）
                if self.cpu.regs.sp > self.gbs_header.stack_pointer {
                    self.cpu.regs.sp = self.gbs_header.stack_pointer;
                }
                // 再生ルーチンのアドレスをCALL
                self.push_stack(((GBSPLAYER_INIT_PLAY_RETURN_ADDRESS >> 8) & 0xFF) as u8);
                self.push_stack(((GBSPLAYER_INIT_PLAY_RETURN_ADDRESS >> 0) & 0xFF) as u8);
                self.cpu.regs.pc = self.gbs_header.play_address;
            }
        }
        self.elapsed_cycles -= DMG_SYSTEM_CLOCK_HZ;
        self.cpu.compute_audio_output()
    }
}
