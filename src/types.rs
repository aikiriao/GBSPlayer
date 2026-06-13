// ハードウェア仕様
/// ゲームボーイのマスタークロック(Hz)
pub const DMG_MASTER_CLOCK_HZ: u32 = 4194304;
/// ゲームボーイのシステムクロック(Hz)
pub const DMG_SYSTEM_CLOCK_HZ: u32 = DMG_MASTER_CLOCK_HZ / 4;
/// ROMの単一バンクサイズ
pub const DMG_ROM_BANK_SIZE: usize = 0x4000;
/// VBlankあたりのマスタークロック数（PPUの1ライン456cycles x 1フレーム154ラインから）
pub const MASTER_CLOCKS_PER_VBLANK: u32 = 70224;
/// VBlank（垂直同期）間隔(Hz) 1フレーム当たり70224サイクルであることから導出
pub const DMG_VBLANK_PERIOD_HZ: f32 =
    (DMG_MASTER_CLOCK_HZ as f32) / (MASTER_CLOCKS_PER_VBLANK as f32);

// オーディオ仕様
/// エンベロープスイープの更新頻度(Hz)
pub const APU_ENVELOPE_SWEEP_HZ: u32 = 64;
/// 持続時間タイマーの更新頻度(Hz)
pub const APU_SOUND_LENGTH_HZ: u32 = 256;
/// CH1周波数スイープの更新頻度(Hz)
pub const APU_FREQUENCY_SWEEP_HZ: u32 = 128;

// アドレス
/// Bank0 ROM開始アドレス
pub const ROM_BANK0_START_ADDRESS: usize = 0x0000;
/// Bank1 ROM開始アドレス
pub const ROM_BANK1_START_ADDRESS: usize = 0x4000;
/// VRAM開始アドレス
pub const VRAM_START_ADDRESS: usize = 0x8000;
/// 外部RAM開始アドレス
pub const EXTERNAL_RAM_START_ADDRESS: usize = 0xA000;
/// Work RAM
pub const WRAM_BANK0_START_ADDRESS: usize = 0xC000;
/// Work RAM Bank 1-7
pub const WRAM_BANK1_START_ADDRESS: usize = 0xD000;
/// Echo RAM
pub const ECHO_RAM_START_ADDRESS: usize = 0xE000;
/// Object Attribute Memory (OAM)
pub const OAM_START_ADDRESS: usize = 0xFE00;
pub const NOT_USABLE_START_ADDRESS: usize = 0xFEA0;
/// ハードウェアレジスタの開始アドレス
pub const HWREG_START_ADDRESS: usize = 0xFF00;
/// High RAM (HRAM)
pub const HRAM_START_ADDRESS: usize = 0xFF80;

// ハードウェアレジスタアドレス
/// ジョイパッド
pub const HWREG_P1_JOYPAD: usize = 0xFF00;
/// シリアル通信データ
pub const HWREG_SB_SERIAL_TRANSFER_DATA: usize = 0xFF01;
/// シリアル通信制御
pub const HWREG_SC_SERIAL_TRANSFER_CONTROL: usize = 0xFF02;
/// 単調増加タイマー
pub const HWREG_DIV_REGISTER: usize = 0xFF04;
/// タイマーカウンタ
pub const HWREG_TIMA_TIMER_COUNTER: usize = 0xFF05;
/// タイマー剰余
pub const HWREG_TMA_TIMER_MODULO: usize = 0xFF06;
/// タイマー制御
pub const HWREG_TAC_TIMER_CONTROL: usize = 0xFF07;
/// 割り込みフラグ
pub const HWREG_IF_INTERRUPT_FLAG: usize = 0xFF0F;
/// チャンネル1スイープ
pub const HWREG_NR10_CHANNEL1_SWEEP: usize = 0xFF10;
/// チャンネル1長さタイマー・デューティ比
pub const HWREG_NR11_CHANNEL1_LENGTH_TIMER_DURY_CYCLE: usize = 0xFF11;
/// チャンネル1ボリューム・エンベロープ
pub const HWREG_NR12_CHANNEL1_VOLUME_ENVELOPE: usize = 0xFF12;
/// チャンネル1周期下位ビット
pub const HWREG_NR13_CHANNEL1_PERIOD_LOW: usize = 0xFF13;
/// チャンネル1周期上位ビット・制御ビット
pub const HWREG_NR14_CHANNEL1_PERIOD_HIGH_CONTROL: usize = 0xFF14;
/// チャンネル2長さタイマー・デューティ比
pub const HWREG_NR21_CHANNEL2_LENGTH_TIMER_DURY_CYCLE: usize = 0xFF16;
/// チャンネル2ボリューム・エンベロープ
pub const HWREG_NR22_CHANNEL2_VOLUME_ENVELOPE: usize = 0xFF17;
/// チャンネル2周期下位ビット
pub const HWREG_NR23_CHANNEL2_PERIOD_LOW: usize = 0xFF18;
/// チャンネル2周期上位ビット・制御ビット
pub const HWREG_NR24_CHANNEL2_PERIOD_HIGH_CONTROL: usize = 0xFF19;
/// チャンネル3DAC有効フラグ
pub const HWREG_NR30_CHANNEL3_DAC_ENABLE: usize = 0xFF1A;
/// チャンネル3長さタイマー
pub const HWREG_NR31_CHANNEL3_LENGTH_TIMER: usize = 0xFF1B;
/// チャンネル3出力レベル
pub const HWREG_NR32_CHANNEL3_OUTPUT_LEVEL: usize = 0xFF1C;
/// チャンネル3周期下位ビット
pub const HWREG_NR33_CHANNEL3_PERIOD_LOW: usize = 0xFF1D;
/// チャンネル3周期上位ビット・制御ビット
pub const HWREG_NR33_CHANNEL3_PERIOD_HIGH_CONTROL: usize = 0xFF1E;
/// チャンネル4長さタイマー
pub const HWREG_NR41_CHANNEL4_LENGTH_TIMER: usize = 0xFF20;
/// チャンネル4ボリューム・エンベロープ
pub const HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE: usize = 0xFF21;
/// チャンネル4周波数・ランダム度
pub const HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS: usize = 0xFF22;
/// チャンネル4制御
pub const HWREG_NR44_CHANNEL4_CONTROL: usize = 0xFF23;
/// マスターボリューム・外部音声出力のパン
pub const HWREG_NR50_MASTER_VOLUME_VIN_PANNING: usize = 0xFF24;
/// 各チャンネルのパン
pub const HWREG_NR51_SOUND_PANNING: usize = 0xFF25;
/// 全体と各チャンネルのオーディオON/OFF
pub const HWREG_NR52_AUDIO_MASTER_CONTROL: usize = 0xFF26;
/// チャンネル3の波形RAM
pub const HWREG_CHANNEL3_WAVE_PATTERN_RAM_START: usize = 0xFF30;
/// チャンネル3の波形RAM領域終端アドレス
pub const HWREG_CHANNEL3_WAVE_PATTERN_RAM_END: usize = HWREG_CHANNEL3_WAVE_PATTERN_RAM_START + 15;
/// LCD制御
pub const HWREG_LCDC_LCD_CONTROL: usize = 0xFF40;
/// LCDステータス
pub const HWREG_STAT_LCD_STATUS: usize = 0xFF41;
/// ビューポートのY座標
pub const HWREG_SCY_VIEWPORT_Y: usize = 0xFF42;
/// ビューポートのX座標
pub const HWREG_SCX_VIEWPORT_X: usize = 0xFF43;
/// LCDのY座標
pub const HWREG_LY_LCD_Y_COORDINATE: usize = 0xFF44;
/// LYとの比較
pub const HWREG_LYC_LY_COMPARE: usize = 0xFF45;
/// ROM/RAMからOAM(Object Attribute Memory)へのDMA転送アドレス
pub const HWREG_DMA_SOURCE_ADDRESS_START: usize = 0xFF46;
/// BGパレットデータ
pub const HWREG_BGP_BG_PALETTE_DATA: usize = 0xFF47;
/// パレット0のカラーインデックス
pub const HWREG_OBP0_OBJ_PALETTE0: usize = 0xFF48;
/// パレット1のカラーインデックス
pub const HWREG_OBP1_OBJ_PALETTE1: usize = 0xFF49;
/// ウィンドウのY座標
pub const HWREG_WY_WINDOW_Y: usize = 0xFF4A;
/// ウィンドウのX座標+7
pub const HWREG_WX_WINDOW_X_PLUS_7: usize = 0xFF4B;
/// CPUモードセレクト
pub const HWREG_KEY0_CPU_MODE_SELECT: usize = 0xFF4C;
/// CGBの倍速モード/通常速モードに備える
pub const HWREG_KEY1_PREPARE_SPEED_SWITCH: usize = 0xFF4D;
/// VRAMバンク
pub const HWREG_VBK_VRAM_BANK: usize = 0xFF4F;
/// ブートROMのマッピング制御
pub const HWREG_BANK_BOOTROM_MAPPING_CONTROL: usize = 0xFF50;
/// VRAM DMAのソースアドレス上位
pub const HWREG_HDMA1_VRAMDMA_SOURCE_HIGH: usize = 0xFF51;
/// VRAM DMAのソースアドレス下位
pub const HWREG_HDMA2_VRAMDMA_SOURCE_LOW: usize = 0xFF52;
/// VRAM DMAのターゲットアドレス上位
pub const HWREG_HDMA3_VRAMDMA_DESTINATION_HIGH: usize = 0xFF53;
/// VRAM DMAのターゲットアドレス下位
pub const HWREG_HDMA4_VRAMDMA_DESTINATION_LOW: usize = 0xFF54;
/// VRAM DMAの長さ/モード/開始
pub const HWREG_HDMA5_LENGTH_MODE_START: usize = 0xFF55;
/// 赤外線通信ポート
pub const HWREG_RP_INFRATED_COMMUNICATIONS_PORT: usize = 0xFF56;
/// 背景カラーパレット仕様・インデックス
pub const HWREG_BCPS_BACKGROUND_COLOR_PALETTE_SPECIFICATION: usize = 0xFF68;
/// 背景カラーパレットデータ
pub const HWREG_BCPS_BACKGROUND_COLOR_PALETTE_DATA: usize = 0xFF69;
/// OBJカラーパレット仕様・インデックス
pub const HWREG_OCPS_OBJ_COLOR_PALETTE_SPECIFICATION: usize = 0xFF6A;
/// OBJカラーパレットデータ
pub const HWREG_OCPS_OBJ_COLOR_PALETTE_DATA: usize = 0xFF6B;
/// オブジェクト優先度
pub const HWREG_OPRI_OBJECT_PRIOROTY_MODE: usize = 0xFF6C;
/// WRAMバンク
pub const HWREG_SVBK_WRAM_BANK: usize = 0xFF70;
/// チャンネル1,2のオーディオデジタル出力
pub const HWREG_PCM12_AUDIO_DIGITAL_OUTPUTS_12: usize = 0xFF76;
/// チャンネル3,4のオーディオデジタル出力
pub const HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34: usize = 0xFF77;
/// 割り込み有効フラグ
pub const HWREG_IE_INTERRUPT_ENABLE: usize = 0xFFFF;

/// SM83レジスタ群
#[derive(Debug, Clone)]
pub struct SM83Registers {
    /// A（アキュムレータ）
    pub a: u8,
    /// F（フラグ）
    pub f: u8,
    /// B（汎用レジスタ）
    pub b: u8,
    /// C（汎用レジスタ）
    pub c: u8,
    /// D（汎用レジスタ）
    pub d: u8,
    /// E（汎用レジスタ）
    pub e: u8,
    /// H（汎用+メモリアクセスレジスタ）
    pub h: u8,
    /// L（汎用+メモリアクセスレジスタ）
    pub l: u8,
    /// SP（スタックポインタ）
    pub sp: u16,
    /// PC（プログラムカウンタ）
    pub pc: u16,
}

/// SM83の16ビットレジスタ種別
#[derive(Debug)]
pub enum SM83Register16 {
    /// AF
    AF,
    /// BC
    BC,
    /// DE
    DE,
    /// HL
    HL,
    /// SP
    SP,
    /// HL+
    HLincrement,
    /// HL-
    HLdecrement,
}

/// SM83の8ビットレジスタ種別
#[derive(Debug)]
pub enum SM83Register8 {
    /// A
    A,
    /// B
    B,
    /// C
    C,
    /// D
    D,
    /// E
    E,
    /// H
    H,
    /// L
    L,
}

/// SM83の条件コード
#[derive(Debug)]
pub enum SM83ConditionCode {
    /// Z is set
    Z,
    /// Z is not set
    NZ,
    /// C is set
    C,
    /// C is not set
    NC,
}

/// SM83オペランド
#[derive(Debug)]
pub enum SM83Oprand {
    N16ToR16 {
        dst: SM83Register16,
        n16: u16,
    },
    R16ToA16 {
        a16: u16,
        src: SM83Register16,
    },
    R16 {
        r16: SM83Register16,
    },
    R8 {
        r8: SM83Register8,
    },
    R16Indirect {
        r16: SM83Register16,
    },
    N8ToR8 {
        dst: SM83Register8,
        n8: u8,
    },
    N8ToR8Indirect {
        dst: SM83Register8,
        n8: u8,
    },
    N8ToR16Indirect {
        dst: SM83Register16,
        n8: u8,
    },
    R16ToR16 {
        dst: SM83Register16,
        src: SM83Register16,
    },
    R8ToR8 {
        dst: SM83Register8,
        src: SM83Register8,
    },
    R16IndirectToR8 {
        dst: SM83Register8,
        src: SM83Register16,
    },
    R8ToR16Indirect {
        dst: SM83Register16,
        src: SM83Register8,
    },
    E8 {
        e8: i8,
    },
    CCAndE8 {
        cc: SM83ConditionCode,
        e8: i8,
    },
    R8AndR8 {
        r1: SM83Register8,
        r2: SM83Register8,
    },
    R8AndR16Indirect {
        r8: SM83Register8,
        r16: SM83Register16,
    },
    CC {
        cc: SM83ConditionCode,
    },
    CCAndA16 {
        cc: SM83ConditionCode,
        a16: u16,
    },
    A16 {
        a16: u16,
    },
    R8AndN8 {
        r8: SM83Register8,
        n8: u8,
    },
    R8ToA8 {
        dst: u8,
        src: SM83Register8,
    },
    R8ToR8Indirect {
        dst: SM83Register8,
        src: SM83Register8,
    },
    R16AndR16 {
        r1: SM83Register16,
        r2: SM83Register16,
    },
    R16AndE8 {
        r16: SM83Register16,
        e8: i8,
    },
    R8ToA16 {
        dst: u16,
        src: SM83Register8,
    },
    A8ToR8 {
        dst: SM83Register8,
        src: u8,
    },
    R8IndirectToR8 {
        dst: SM83Register8,
        src: SM83Register8,
    },
    R16E8IndirectToR16 {
        dst: SM83Register16,
        src_r16: SM83Register16,
        src_e8: i8,
    },
    A16ToR8 {
        dst: SM83Register8,
        src: u16,
    },
}

/// SM83オペコード
#[derive(Debug)]
pub enum SM83Opcode {
    /// NOP
    NOP,
    /// LD (Load)
    LD { oprand: SM83Oprand },
    /// INC (Increment)
    INC { oprand: SM83Oprand },
    /// DEC (Decrement)
    DEC { oprand: SM83Oprand },
    /// RLCA (Rotate Register A Left)
    RLCA,
    /// ADD (Add)
    ADD { oprand: SM83Oprand },
    /// RRCA (Rotate Register A Right)
    RRCA,
    /// STOP (Stop)
    STOP,
    /// JR (Relative Jump)
    JR { oprand: SM83Oprand },
    /// RLA (Rotate Accumulator Left, Through the Carry Flag)
    RLA,
    /// RLA (Rotate Accumulator Right, Through the Carry Flag)
    RRA,
    /// DAA (Decimal Adjust Accumulator)
    DAA,
    /// CPL (ComPLement Accumulator)
    CPL,
    /// SCF (Set Carry Flag)
    SCF,
    /// CCF (Complement Carry Flag)
    CCF,
    /// HALT (Halt)
    HALT,
    /// ADC (Add with Carry Flag)
    ADC { oprand: SM83Oprand },
    /// SUB (Sub)
    SUB { oprand: SM83Oprand },
    /// SBC (Sub with Carry Flag)
    SBC { oprand: SM83Oprand },
    /// AND (Bitwise And)
    AND { oprand: SM83Oprand },
    /// XOR (Bitwise Xor)
    XOR { oprand: SM83Oprand },
    /// OR (Bitwise Or)
    OR { oprand: SM83Oprand },
    /// CP (ComPare)
    CP { oprand: SM83Oprand },
    /// 無条件RET (Return from Subroutine)
    RETNooprand,
    /// RET (Return from Subroutine)
    RET { oprand: SM83Oprand },
    /// POP (Pop from the Stack)
    POP { oprand: SM83Oprand },
    /// JP (Jump to the Address)
    JP { oprand: SM83Oprand },
    /// CALL (Call Address)
    CALL { oprand: SM83Oprand },
    /// PUSH (Push into the Stack)
    PUSH { oprand: SM83Oprand },
    /// REST (Call Vector Address)
    RST { vec: u8 },
    /// RETI (Return from Subroutine and Enable Interrupts)
    RETI,
    /// LDH (Copy the Register A into the Address)
    LDH { oprand: SM83Oprand },
    /// DI (Disable Interrupts)
    DI,
    /// EI (Enable Interrupts)
    EI,
    // --- Prefixed Opcodes ---
    /// RLC (Rotate Register Left)
    RLC { oprand: SM83Oprand },
    /// RRC (Rotate Register Right)
    RRC { oprand: SM83Oprand },
    /// RR (Rotate Register Right, Through the Carry Flag)
    RR { oprand: SM83Oprand },
    /// RL (Rotate Register Left, Through the Carry Flag)
    RL { oprand: SM83Oprand },
    /// SLA (Shift Left Arithmetically)
    SLA { oprand: SM83Oprand },
    /// SRA (Shift Right Arithmetically)
    SRA { oprand: SM83Oprand },
    /// SWAP (Swap the Upper 4 Bits and the Lower 4 Bits)
    SWAP { oprand: SM83Oprand },
    /// SRL (Shift Right Logically)
    SRL { oprand: SM83Oprand },
    /// BIT (Test Bit u3 in Register)
    BIT { u3: u8, oprand: SM83Oprand },
    /// RES (Reset Bit u3 in Register)
    RES { u3: u8, oprand: SM83Oprand },
    /// SET (Set Bit u3 in Register)
    SET { u3: u8, oprand: SM83Oprand },
}

/// スイープ（変化）の方向
#[derive(Debug, Clone, Copy)]
pub enum SweepDirection {
    /// 正
    Positive,
    /// 負
    Negative,
}

/// パンニング
#[derive(Debug, Clone, Copy)]
pub enum Pan {
    /// 左
    Left,
    /// 右
    Right,
    /// 中央
    Center,
    /// 設定なし（無音）
    Ignore,
}

/// デューティ比
#[derive(Debug, Clone, Copy)]
pub enum DutyRatio {
    /// 12.5%
    Duty125,
    /// 25%
    Duty250,
    /// 50%
    Duty500,
    /// 75%
    Duty750,
}

/// APU(Audio Processing Unit)の共通インターフェース
pub trait APUDevice {
    /// コンストラクタ
    fn new() -> Self;
    /// レジスタ書き込み
    fn write_register(&mut self, address: usize, value: u8);
    /// レジスタ読み込み
    fn read_register(&mut self, address: usize) -> u8;
    /// 2MHzクロックティック
    fn clock_tick_2mhz(&mut self);
}

/// メモリ上にあるデータから16bitデータを読みだす
pub fn make_u16_from_u8(data: &[u8]) -> u16 {
    assert_eq!(data.len(), 2);
    ((data[1] as u16) << 8) | data[0] as u16
}
