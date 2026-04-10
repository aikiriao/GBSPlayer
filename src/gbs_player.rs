use crate::gbs_file::*;
use crate::sm83::*;
use crate::types::*;

pub struct GBSPlayer<'a> {
    cpu: SM83<'a>,
    gbs_header: GBSFileHeader,
}

impl<'a> GBSPlayer<'a> {
    /// コンストラクタ
    pub fn new(gbs_header: &GBSFileHeader, rom: &'a [u8]) -> Self {
        Self {
            cpu: SM83::new(rom),
            gbs_header: gbs_header.clone(),
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
        // PCを初期化
        self.cpu.regs.pc = self.gbs_header.init_address;

        // タイマー初期化
        self.cpu
            .write_mem_u8(HWREG_TMA_TIMER_MODULO, self.gbs_header.timer_modulo);
        self.cpu
            .write_mem_u8(HWREG_TAC_TIMER_CONTROL, self.gbs_header.timer_control);

        // RETが実行されるまで実行
        loop {
            let (opcode, cycle) = self.cpu.execute_step();
            match opcode {
                SM83Opcode::RETNooprand => break,
                SM83Opcode::RET { .. } => {
                    if cycle == 5 {
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    /// 再生
    pub fn play(&mut self) {
        self.cpu.regs.pc = self.gbs_header.play_address;

        // RETが実行されるまで実行 + 定期音声出力
        loop {
            let (opcode, cycle) = self.cpu.execute_step();
            match opcode {
                SM83Opcode::RETNooprand => break,
                SM83Opcode::RET { .. } => {
                    if cycle == 5 {
                        break;
                    }
                }
                _ => {}
            }

            // システムクロックティック
            for _ in 0..cycle {
                self.cpu.system_clock_tick();
            }
        }
    }
}
