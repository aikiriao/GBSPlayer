use crate::gbs_file::*;
use crate::sm83::*;
use crate::types::*;

pub struct GBSPlayer {
    cpu: SM83,
    gbs_header: GBSFileHeader,
}

impl GBSPlayer {
    /// コンストラクタ
    pub fn new(gbs_header: &GBSFileHeader) -> Self {
        Self {
            cpu: SM83::new(),
            gbs_header: gbs_header.clone(),
        }
    }

    /// ロード
    pub fn load(&mut self, data: &[u8]) {
        let load_start = self.gbs_header.load_address as usize;
        let load_end = self.gbs_header.load_address as usize + data.len();

        assert!(load_start < 0x8000);

        self.cpu.mem[load_start..load_end].copy_from_slice(data);
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
            .write_ram_u8(HWREG_TMA_TIMER_MODULO, self.gbs_header.timer_modulo);
        self.cpu
            .write_ram_u8(HWREG_TAC_TIMER_CONTROL, self.gbs_header.timer_control);

        // RETが実行されるまで実行
        loop {
            let (opcode, _) = self.cpu.execute_step();
            match opcode {
                SM83Opcode::RETNooprand => break,
                // TODO: もしかしたら条件付きRETがくるかも...
                _ => {}
            }
        }
    }

    /// 再生
    pub fn play(&mut self) {
        self.cpu.regs.pc = self.gbs_header.play_address;

        // RETが実行されるまで実行 + 定期音声出力
        loop {
            let (opcode, _) = self.cpu.execute_step();
            match opcode {
                SM83Opcode::RETNooprand => break,
                // TODO: もしかしたら条件付きRETがくるかも...
                _ => {}
            }
        }
    }
}
