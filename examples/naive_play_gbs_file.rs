use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use gbs_player::gbs_file::*;
use gbs_player::gbs_player::*;
use gbs_player::types::*;
use gbs_player::apu::*;
use std::env;
use std::fmt::Error;

const NUM_CHANNELS: usize = 2;
const NORMALIZED_CONST: f32 = 1.0 / 4.0; // [-1,1]の範囲の波形が4ch分あるため範囲は[-4,4]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // 引数が合っていないときは説明を表示
    if args.len() != 3 {
        eprintln!("Usage: {} GBS_FILE SONG_NUMBER", args[0]);
        return Err(Box::new(Error));
    }

    // データ読み込み
    let data = std::fs::read(&args[1])?;
    let song_number: u8 = args[2].parse().expect("Failed to parse as u8 number");
    if let Some(header) = parse_gbs_header(&data) {
        // cpalの初期化
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let stream_config: cpal::StreamConfig = device.default_output_config().unwrap().into();
        let sampling_rate = stream_config.sample_rate.0;

        // 読み込み用のROMを作成
        // - ロードアドレスによっては最悪0x4000必要になるため追加
        // - 0x4000の倍数サイズに切り上げる
        let rom: Box<[u8]> = Box::from(&data[0x70..]);
        let mut load_rom_size = rom.len() + DMG_ROM_BANK_SIZE;
        load_rom_size =
            ((load_rom_size + DMG_ROM_BANK_SIZE - 1) / DMG_ROM_BANK_SIZE) * DMG_ROM_BANK_SIZE;
        let mut load_rom = vec![0u8; load_rom_size].into_boxed_slice();
        load_rom[(header.load_address as usize)..(header.load_address as usize + rom.len())]
            .copy_from_slice(&rom);

        // エミュレータ作成
        let mut player: GBSPlayer<_, APU> = GBSPlayer::new(&header, load_rom, sampling_rate);
        player.load();
        player.init(song_number);

        // 再生ストリーム作成
        let stream = device
            .build_output_stream(
                &stream_config,
                move |buffer: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for i in (0..buffer.len()).step_by(NUM_CHANNELS) {
                        let out = player.output_audio_sample();
                        buffer[i + 0] = out[0] * NORMALIZED_CONST;
                        buffer[i + 1] = out[1] * NORMALIZED_CONST;
                    }
                },
                |err| eprintln!("[GBS Player] {err}"),
                None,
            )
            .unwrap();

        // 再生開始
        stream.play()?;
        loop {}
    }

    Ok(())
}
