use crate::gbs_file::*;
use crate::sm83::*;
use crate::types::*;

pub struct GBSPlayer<'a> {
    gbs_header: GBSFileHeader,
    cpu: SM83<'a>,
    sampling_rate: u32,
    audio_output_interval_cycles: u32,
    elapsed_cycles: u32,
    interrupt_cycles: u32,
}

impl<'a> GBSPlayer<'a> {
    /// コンストラクタ
    pub fn new(gbs_header: &GBSFileHeader, rom: &'a [u8], sampling_rate: u32) -> Self {
        Self {
            gbs_header: gbs_header.clone(),
            cpu: SM83::new(rom),
            sampling_rate: sampling_rate,
            audio_output_interval_cycles: (DMG_SYSTEM_CLOCK_HZ as f32 / sampling_rate as f32).round() as u32,
            elapsed_cycles: 0,
            interrupt_cycles: 0,
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

        // サンプリングレート設定
        self.cpu.set_audio_sampling_rate(self.sampling_rate);

        // 経過クロックカウントをリセット
        self.elapsed_cycles = 0;
        self.interrupt_cycles = 0;

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

    /// 再生ルーチン（割り込み間隔で実行）
    fn play(&mut self) {
        self.cpu.regs.pc = self.gbs_header.play_address;

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

            // システムクロックティック
            for _ in 0..cycle {
                self.cpu.system_clock_tick();
            }
        }
    }

    /// playルーチンの割り込みシステムクロック間隔を計算
    fn compute_play_interrupt_interval_system_clocks(&self) -> u32 {
        // タイマー割り込み無効ならV-blank割り込みを使用
        if (self.cpu.mem[HWREG_TAC_TIMER_CONTROL] & 0x4) == 0 {
            return (DMG_SYSTEM_CLOCK_HZ as f32 / 59.7).round() as u32;
        }

        // タイマー割り込みを使用
        // TODO: double-speed modeだと違う
        let counter_rate_hz = match self.cpu.mem[HWREG_TAC_TIMER_CONTROL] & 0x3 {
            0 => 4096,
            1 => 262144,
            2 => 65536,
            3 => 16384,
            _ => unreachable!(),
        };

        // 割り込み間隔時間を計算
        let clock_interval = counter_rate_hz as f32 / (256.0 - self.cpu.mem[HWREG_TMA_TIMER_MODULO] as f32);

        (DMG_SYSTEM_CLOCK_HZ as f32 / clock_interval).round() as u32
    }

    /// 1ステレオサンプル出力
    pub fn output_audio_sample(&mut self) -> [f32; 2] {
        let interrupt_cycles = self.compute_play_interrupt_interval_system_clocks();
        while self.elapsed_cycles < self.audio_output_interval_cycles {
            let (_, cycle) = self.cpu.execute_step();
            // システムクロックティック
            for _ in 0..cycle {
                self.cpu.system_clock_tick();
            }
            self.elapsed_cycles += cycle as u32;
            self.interrupt_cycles += cycle as u32;
            if self.interrupt_cycles > interrupt_cycles {
                self.play();
                self.interrupt_cycles -= interrupt_cycles; 
            }
        }
        self.elapsed_cycles -= self.audio_output_interval_cycles;
        self.cpu.output_audio_sample()
    }
}
