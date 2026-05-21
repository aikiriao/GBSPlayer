use crate::gbs_file::*;
use crate::sm83::*;
use crate::types::*;

pub struct GBSPlayer<'a> {
    gbs_header: GBSFileHeader,
    cpu: SM83<'a>,
    sampling_rate: u32,
    elapsed_cycles: u32,
}

impl<'a> GBSPlayer<'a> {
    /// コンストラクタ
    pub fn new(gbs_header: &GBSFileHeader, rom: &'a [u8], sampling_rate: u32) -> Self {
        Self {
            gbs_header: gbs_header.clone(),
            cpu: SM83::new(rom),
            sampling_rate: sampling_rate,
            elapsed_cycles: 0,
        }
    }

    /// ロード
    pub fn load(&mut self) {
        let mut load_size = self.cpu.rom.len();
        assert!(load_size >= 0x4000);
        assert!((load_size % 0x4000) == 0);

        // 読み出しサイズがROM領域を飛び出ていたら制限
        // 残りのメモリはROMバンク切り替えでアクセスする
        if load_size >= 0x8000 {
            load_size = 0x8000;
        }

        self.cpu.mem[..load_size].copy_from_slice(&self.cpu.rom[..load_size]);
    }

    /// 初期化
    pub fn init(&mut self, song_number: u8) {
        // レジスタ・フラグ・RAMのクリア
        self.cpu.reset_registers();
        self.cpu.reset_flags();
        self.cpu.clear_ram();

        // アキュムレータに曲番号
        assert!(song_number >= (self.gbs_header.first_song - 1));
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

        // PCを初期化 戻り先は割り込みベクタを避けて0x0100とする
        self.push_stack(0x01);
        self.push_stack(0x00);
        self.cpu.regs.pc = self.gbs_header.init_address;
    }

    /// playルーチンの割り込みが発生しているか判定
    fn check_play_interrupt(&mut self) -> bool {
        // 判定用フラグの取得
        let flag = if (self.cpu.mem[HWREG_TAC_TIMER_CONTROL] & 0x4) == 0 {
            // タイマー割り込み無効ならV-blank割り込みを使用
            SM83_INTERRUPT_FLAG_VBLANK
        } else {
            SM83_INTERRUPT_FLAG_TIMER
        };

        // 割り込み判定・フラグクリア
        let ret = (self.cpu.mem[HWREG_IF_INTERRUPT_FLAG] & flag) != 0;
        self.cpu.mem[HWREG_IF_INTERRUPT_FLAG] &= !flag;

        ret
    }

    /// スタックにデータをPUSH（SM83にも同様の関数はあるがSM83の関数は隠蔽したいため自前実装）
    fn push_stack(&mut self, value: u8) {
        self.cpu.regs.sp = self.cpu.regs.sp.wrapping_sub(1);
        self.cpu.write_mem_u8(self.cpu.regs.sp as usize, value);
    }

    /// 1ステレオサンプル出力
    pub fn output_audio_sample(&mut self) -> [f32; 2] {
        while self.elapsed_cycles < DMG_SYSTEM_CLOCK_HZ {
            // 命令実行
            let (_, cycle) = self.cpu.execute_step();
            // サイクルカウントを累積
            self.elapsed_cycles += cycle as u32 * self.sampling_rate;
            // 割り込み処理
            if self.check_play_interrupt() {
                // 再生ルーチンのアドレスをCALL 戻り先は割り込みベクタを避けて0x0100とする
                self.push_stack(0x01);
                self.push_stack(0x00);
                self.cpu.regs.pc = self.gbs_header.play_address;
            }
        }
        self.elapsed_cycles -= DMG_SYSTEM_CLOCK_HZ;
        self.cpu.output_audio_sample()
    }
}
