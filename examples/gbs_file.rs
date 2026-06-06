use sm83::types::*;

/// GBSファイルヘッダ
#[derive(Debug, Clone)]
pub struct GBSFileHeader {
    /// バージョン
    pub version: u8,
    /// 曲数
    pub num_songs: u8,
    /// 最初の曲
    pub first_song: u8,
    /// ロードアドレス (0x0400 - 0x7FFF)
    pub load_address: u16,
    /// 初期化アドレス (0x0400 - 0x7FFF)
    pub init_address: u16,
    /// プレイ（定期実行処理）アドレス (0x0400 - 0x7FFF)
    pub play_address: u16,
    /// スタックポインタ
    pub stack_pointer: u16,
    /// タイマーの剰余
    pub timer_modulo: u8,
    /// タイマーコントロール
    pub timer_control: u8,
    /// タイトル
    pub title: [u8; 32],
    /// 作曲者
    pub author: [u8; 32],
    /// コピーライト
    pub copyright: [u8; 32],
}

/// GBSファイルヘッダのパース
pub fn parse_gbs_header(data: &[u8]) -> Option<GBSFileHeader> {
    // サイズチェック
    if data.len() < 0x70 {
        return None;
    }

    // シグニチャのチェック
    if (data[0] != b'G') || (data[1] != b'B') || (data[2] != b'S') {
        return None;
    }

    Some(GBSFileHeader {
        version: data[0x3],
        num_songs: data[0x4],
        first_song: data[0x5],
        load_address: make_u16_from_u8(&data[0x6..0x6 + 2]),
        init_address: make_u16_from_u8(&data[0x8..0x8 + 2]),
        play_address: make_u16_from_u8(&data[0xA..0xA + 2]),
        stack_pointer: make_u16_from_u8(&data[0xC..0xC + 2]),
        timer_modulo: data[0xE],
        timer_control: data[0xF],
        title: data[0x10..0x10 + 32].try_into().unwrap(),
        author: data[0x30..0x30 + 32].try_into().unwrap(),
        copyright: data[0x50..0x50 + 32].try_into().unwrap(),
    })
}
