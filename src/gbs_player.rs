use crate::gbs_file::*;
use crate::sm83::*;
use crate::types::*;

pub struct GBSPlayer {
    sm83: SM83,
    gbs_header: GBSFileHeader,
}

impl GBSPlayer {
    /// コンストラクタ
    pub fn new(gbs_header: &GBSFileHeader) -> Self {
        Self {
            sm83: SM83::new(),
            gbs_header: gbs_header.clone(),
        }
    }

    /// ロード
    pub fn load(&mut self, data: &[u8]) {
        let load_start = self.gbs_header.load_address as usize;
        let load_end = self.gbs_header.load_address as usize + data.len();

        assert!(load_start < 0x8000);

        self.sm83.mem[load_start..load_end].copy_from_slice(data);
    }

    /// 初期化
    pub fn init(&mut self, song_number: u8) {
        // レジスタ・フラグ・RAMのクリア
        self.sm83.reset_registers();
        self.sm83.reset_flags();
        self.sm83.clear_ram();

        // アキュムレータに曲番号
        assert!(song_number < self.gbs_header.num_songs);
        self.sm83.regs.a = song_number;
        // スタックポインタ初期化
        self.sm83.regs.sp = self.gbs_header.stack_pointer;
        // PCを初期化
        self.sm83.regs.pc = self.gbs_header.init_address;

        // タイマー初期化
        self.sm83
            .write_ram_u8(HWREG_TMA_TIMER_MODULO, self.gbs_header.timer_modulo);
        self.sm83
            .write_ram_u8(HWREG_TAC_TIMER_CONTROL, self.gbs_header.timer_control);

        // RETが実行されるまで実行
        loop {
            let (opcode, _) = self.sm83.execute_step();
            match opcode {
                SM83Opcode::RETNooprand => break,
                // TODO: もしかしたら条件付きRETがくるかも...
                _ => {}
            }
        }
    }

    /// 再生
    pub fn play(&mut self) {
        self.sm83.regs.pc = self.gbs_header.play_address;

        // RETが実行されるまで実行 + 定期音声出力
        loop {
            let (opcode, _) = self.sm83.execute_step();
            match opcode {
                SM83Opcode::RETNooprand => break,
                // TODO: もしかしたら条件付きRETがくるかも...
                _ => {}
            }
        }
    }
}
