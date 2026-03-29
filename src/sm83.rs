use crate::types::*;

const ROM_BANK0_START_ADDRESS: usize = 0x0000;
const ROM_BANK1_START_ADDRESS: usize = 0x4000;
const VRAM_START_ADDRESS: usize = 0x8000;
const EXTERNAL_RAM_START_ADDRESS: usize = 0xA000;
/// Work RAM
const WRAM_BANK0_START_ADDRESS: usize = 0xC000;
/// Work RAM Bank 1-7
const WRAM_BANK1_START_ADDRESS: usize = 0xD000;
/// 
const ECHO_RAM_START_ADDRESS: usize = 0xE000;
/// Object Attribute Memory (OAM)
const OAM_START_ADDRESS: usize = 0xFE00;
const NOT_USABLE_START_ADDRESS: usize = 0xFEA0;
/// 
const IOREG_START_ADDRESS: usize = 0xFF00;
/// High RAM (HRAM)
const HRAM_START_ADDRESS: usize = 0xFF80;
/// Intterupt Enable Register
const IE_START_ADDRESS: usize = 0xFFFF;

// ハードウェアレジスタアドレス
/// ジョイパッド
const HWREG_P1_JOYPAD: usize = 0xFF00;
/// シリアル通信データ
const HWREG_SB_SERIAL_TRANSFER_DATA: usize = 0xFF01;
/// シリアル通信制御
const HWREG_SC_SERIAL_TRANSFER_CONTROL: usize = 0xFF02;
/// 単調増加タイマー
const HWREG_DIV_REGISTER: usize = 0xFF04;
/// タイマーカウンタ
const HWREG_TIMA_TIMER_COUNTER: usize = 0xFF05;
/// タイマー剰余
const HWREG_TMA_TIMER_MODULO: usize = 0xFF06;
/// タイマー制御
const HWREG_TAC_TIMER_CONTROL: usize = 0xFF07;
/// 割り込みフラグ
const HWREG_IF_INTERRUPT_FLAG: usize = 0xFF0F;
/// チャンネル1スイープ
const HWREG_NR10_CHANNEL1_SWEEP: usize = 0xFF10;
/// チャンネル1長さタイマー・デューティ比
const HWREG_NR11_CHANNEL1_LENGTH_TIMER_DURY_CYCLE: usize = 0xFF11;
/// チャンネル1ボリューム・エンベロープ
const HWREG_NR12_CHANNEL1_VOLUME_ENVELOPE: usize = 0xFF12;
/// チャンネル1周期下位ビット
const HWREG_NR13_CHANNEL1_PERIOD_LOW: usize = 0xFF13;
/// チャンネル1周期上位ビット・制御ビット
const HWREG_NR14_CHANNEL1_PERIOD_HIGH_CONTROL: usize = 0xFF14;
/// チャンネル2長さタイマー・デューティ比
const HWREG_NR21_CHANNEL2_LENGTH_TIMER_DURY_CYCLE: usize = 0xFF16;
/// チャンネル2ボリューム・エンベロープ
const HWREG_NR22_CHANNEL2_VOLUME_ENVELOPE: usize = 0xFF17;
/// チャンネル2周期下位ビット
const HWREG_NR23_CHANNEL2_PERIOD_LOW: usize = 0xFF18;
/// チャンネル2周期上位ビット・制御ビット
const HWREG_NR24_CHANNEL2_PERIOD_HIGH_CONTROL: usize = 0xFF19;
/// チャンネル3DAC有効フラグ
const HWREG_NR30_CHANNEL3_DAC_ENABLE: usize = 0xFF1A;
/// チャンネル3長さタイマー
const HWREG_NR31_CHANNEL3_LENGTH_TIMER: usize = 0xFF1B;
/// チャンネル3出力レベル
const HWREG_NR32_CHANNEL3_OUTPUT_LEVEL: usize = 0xFF1C;
/// チャンネル3周期下位ビット
const HWREG_NR33_CHANNEL3_PERIOD_LOW: usize = 0xFF1D;
/// チャンネル3周期上位ビット・制御ビット
const HWREG_NR33_CHANNEL3_PERIOD_HIGH_CONTROL: usize = 0xFF1E;
/// チャンネル4長さタイマー
const HWREG_NR41_CHANNEL4_LENGTH_TIMER: usize = 0xFF20;
/// チャンネル4ボリューム・エンベロープ
const HWREG_NR42_CHANNEL4_VOLUME_ENVELOPE: usize = 0xFF21;
/// チャンネル4周波数・ランダム度
const HWREG_NR43_CHANNEL4_FREQUENCY_RANDOMNESS: usize = 0xFF22;
/// チャンネル4制御
const HWREG_NR44_CHANNEL4_CONTROL: usize = 0xFF23;
/// マスターボリューム・外部音声出力のパン
const HWREG_NR51_MASTER_VOLUME_VIN_PANNING: usize = 0xFF24;
/// 各チャンネルのパン
const HWREG_NR51_SOUND_PANNING: usize = 0xFF25;
/// 全体と各チャンネルのオーディオON/OFF
const HWREG_NR52_AUDIO_MASTER_CONTROL: usize = 0xFF26;
/// チャンネル3の波形RAM
const HWREG_CHANNEL3_WAVE_PATTERN_RAM_START: usize = 0xFF30;
/// LCD制御
const HWREG_LCDC_LCD_CONTROL: usize = 0xFF40;
/// LCDステータス
const HWREG_STAT_LCD_STATUS: usize = 0xFF41;
/// ビューポートのY座標
const HWREG_SCY_VIEWPORT_Y: usize = 0xFF42;
/// ビューポートのX座標
const HWREG_SCX_VIEWPORT_X: usize = 0xFF43;
/// LCDのY座標
const HWREG_LY_LCD_Y_COORDINATE: usize = 0xFF44;
/// LYとの比較
const HWREG_LYC_LY_COMPARE: usize = 0xFF45;
/// ROM/RAMからOAM(Object Attribute Memory)へのDMA転送アドレス
const HWREG_DMA_SOURCE_ADDRESS_START: usize = 0xFF46;
/// BGパレットデータ
const HWREG_BGP_BG_PALETTE_DATA: usize = 0xFF47;
/// パレット0のカラーインデックス
const HWREG_OBP0_OBJ_PALETTE0: usize = 0xFF48;
/// パレット1のカラーインデックス
const HWREG_OBP1_OBJ_PALETTE1: usize = 0xFF49;
/// ウィンドウのY座標
const HWREG_WY_WINDOW_Y: usize = 0xFF4A;
/// ウィンドウのX座標+7
const HWREG_WX_WINDOW_X_PLUS_7: usize = 0xFF4B;
/// CPUモードセレクト
const HWREG_KEY0_CPU_MODE_SELECT: usize = 0xFF4C;
/// CGBの倍速モード/通常速モードに備える
const HWREG_KEY1_PREPARE_SPEED_SWITCH: usize = 0xFF4D;
/// VRAMバンク
const HWREG_VBK_VRAM_BANK: usize = 0xFF4F;
/// ブートROMのマッピング制御
const HWREG_BANK_BOOTROM_MAPPING_CONTROL: usize = 0xFF50;
/// VRAM DMAのソースアドレス上位
const HWREG_HDMA1_VRAMDMA_SOURCE_HIGH: usize = 0xFF51;
/// VRAM DMAのソースアドレス下位
const HWREG_HDMA2_VRAMDMA_SOURCE_LOW: usize = 0xFF52;
/// VRAM DMAのターゲットアドレス上位
const HWREG_HDMA3_VRAMDMA_DESTINATION_HIGH: usize = 0xFF53;
/// VRAM DMAのターゲットアドレス下位
const HWREG_HDMA4_VRAMDMA_DESTINATION_LOW: usize = 0xFF54;
/// VRAM DMAの長さ/モード/開始
const HWREG_HDMA5_LENGTH_MODE_START: usize = 0xFF55;
/// 赤外線通信ポート
const HWREG_RP_INFRATED_COMMUNICATIONS_PORT: usize = 0xFF56;
/// 背景カラーパレット仕様・インデックス
const HWREG_BCPS_BACKGROUND_COLOR_PALETTE_SPECIFICATION: usize = 0xFF68;
/// 背景カラーパレットデータ
const HWREG_BCPS_BACKGROUND_COLOR_PALETTE_DATA: usize = 0xFF69;
/// OBJカラーパレット仕様・インデックス
const HWREG_OCPS_OBJ_COLOR_PALETTE_SPECIFICATION: usize = 0xFF6A;
/// OBJカラーパレットデータ
const HWREG_OCPS_OBJ_COLOR_PALETTE_DATA: usize = 0xFF6B;
/// オブジェクト優先度
const HWREG_OPRI_OBJECT_PRIOROTY_MODE: usize = 0xFF6C;
/// WRAMバンク
const HWREG_SVBK_WRAM_BANK: usize = 0xFF70;
/// チャンネル1,2のオーディオデジタル出力
const HWREG_PCM12_AUDIO_DIGITAL_OUTPUTS_12: usize = 0xFF76;
/// チャンネル3,4のオーディオデジタル出力
const HWREG_PCM34_AUDIO_DIGITAL_OUTPUTS_34: usize = 0xFF77;
/// 割り込み有効フラグ
const HWREG_IE_INTTERUPT_ENABLE: usize = 0xFFFF;

/// SM83エミュレータ
pub struct SM83
{
    /// レジスタ
    reg: SM83Registers,
    /// 64KBメモリ領域
    mem: [u8; 65536],
}
