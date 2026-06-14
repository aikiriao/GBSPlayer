use gbs_player::gbs_file::*;
use gbs_player::gbs_player::*;
use gbs_player::midiapu::*;
use gbs_player::types::*;
use midir::MidiOutput;
use std::env;
use std::fmt::Error;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::time::Instant;

const MIDI_OUTPUT_INTERVAL_MS: u32 = 1;

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
        // MIDIポートを開く
        let conn_name = &args[0];
        let (_, mut midi_out_conn) = if let Ok(midi_out) = MidiOutput::new(conn_name) {
            let midi_out_ports = midi_out.ports();
            if midi_out_ports.len() == 0 {
                return Err(Box::new(Error));
            }
            let default_midi_port_name = &midi_out_ports[0];
            let port_name = Some(midi_out.port_name(default_midi_port_name).unwrap());
            let midi_out_conn = match midi_out.connect(default_midi_port_name, conn_name) {
                Ok(conn) => conn,
                Err(_) => return Err(Box::new(Error)),
            };
            (port_name, midi_out_conn)
        } else {
            return Err(Box::new(Error));
        };

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
        let mut player: GBSPlayer<_, MIDIAPU> =
            GBSPlayer::new(&header, load_rom, 1000 / MIDI_OUTPUT_INTERVAL_MS);
        player.load();
        player.init(song_number);

        print!("\x1B[2J\x1B[H");

        // 再生処理
        let _midi_thread = thread::spawn(move || {
            let interval = Duration::from_millis(MIDI_OUTPUT_INTERVAL_MS as u64);
            let start = Instant::now();
            let mut next = Instant::now();
            loop {
                // MIDI出力
                let msgs = player.output_audio_sample();
                for i in 0..msgs.num_messages {
                    let msg = msgs.messages[i];
                    midi_out_conn.send(&msg.data[..msg.length]).unwrap();
                    print!(
                        "{:7.3} {:2X?}            \r",
                        start.elapsed().as_secs_f32(),
                        msg.data[..msg.length].to_vec()
                    );
                    io::stdout().flush().unwrap();
                }
                // ビジーループで待つ
                next += interval;
                while Instant::now() < next {
                    thread::yield_now();
                }
            }
        });

        loop {}
    }

    Ok(())
}
