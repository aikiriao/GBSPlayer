use sm83::gbs_file::*;
use sm83::types::*;
use sm83::gbs_player::*;
use std::env;
use std::fmt::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // 引数が合っていないときは説明を表示
    if args.len() != 2 {
        eprintln!("Usage: {} GBS_FILE", args[0]);
        return Err(Box::new(Error));
    }

    // データ読み込み
    let data = std::fs::read(&args[1])?;
    if let Some(header) = parse_gbs_header(&data) {
        const ROM_BANK_SIZE: usize = 0x4000;
        let rom: Box<[u8]> = Box::from(&data[0x70..]);
        // 読み込み用のROMを作成
        let load_rom_size = ((rom.len() + ROM_BANK_SIZE - 1) / ROM_BANK_SIZE) * ROM_BANK_SIZE;
        let mut load_rom = vec![0u8; load_rom_size].into_boxed_slice();
        load_rom[(header.load_address as usize)..(header.load_address as usize + rom.len())].copy_from_slice(&rom);
        // エミュレータ作成
        let mut player = GBSPlayer::new(&header, &load_rom);
        player.load();
        player.init(0);
    }

    Ok(())
}
