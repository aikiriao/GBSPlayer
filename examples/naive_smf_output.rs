use gbs_player::gbs_file::*;
use gbs_player::gbs_player::*;
use gbs_player::midiapu::*;
use gbs_player::types::*;
use rimd::{Event as MidiEvent, MidiMessage, SMFFormat, SMFWriter, Track, TrackEvent, SMF};
use std::env;
use std::fmt::Error;
use std::path::Path;

// MIDIの出力レート
const MIDI_OUTPUT_RATE: u32 = 1 << 16;
// 四分の一あたりのティック数
const MIDI_TICK_PER_QUARTER: i16 = 480;
// BPM
const BEATS_PER_MINUTE: u32 = 120;
// 2MHz周波数あたりのMIDIティック数
const TICKS_PER_2MHZ: f64 =
    (BEATS_PER_MINUTE as f64 * MIDI_TICK_PER_QUARTER as f64) / (60.0 * 2.0 * (1 << 20) as f64);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // 引数が合っていないときは説明を表示
    if args.len() != 5 {
        eprintln!(
            "Usage: {} GBS_FILE SONG_NUMBER SMF_FILE OUTPUT_DURATION_SEC",
            args[0]
        );
        return Err(Box::new(Error));
    }

    // データ読み込み
    let data = std::fs::read(&args[1])?;
    let song_number: u8 = args[2].parse().expect("Failed to parse as u8 number");
    let output_smf_path = Path::new(&args[3]);
    let output_duration_sec: u64 = args[4].parse().expect("Failed to parse as u64 number");

    if let Some(header) = parse_gbs_header(&data) {
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
        let mut player: GBSPlayer<_, MIDIAPU> = GBSPlayer::new(&header, load_rom, MIDI_OUTPUT_RATE);
        player.load();
        player.init(song_number);

        // MIDI作成
        let mut smf = SMF {
            format: SMFFormat::Single,
            tracks: Vec::new(),
            division: MIDI_TICK_PER_QUARTER,
        };
        smf.tracks.push(Track {
            copyright: None,
            name: Some(String::from_utf8_lossy(&header.title).to_string()),
            events: Vec::new(),
        });

        // 指定時間のMIDIメッセージを出力
        let num_output_msgs = output_duration_sec * MIDI_OUTPUT_RATE as u64;
        let mut total_output_msgs: u64 = 0;
        let mut previous_elapsed_ticks: u64 = 0;
        while total_output_msgs < num_output_msgs {
            let msgs = player.output_audio_sample();
            for i in 0..msgs.num_messages {
                let msg = msgs.messages[i];
                // 2MHzのティック数から累積ティック数を計算
                let total_elapsed_ticks =
                    (msg.clock_tick_2mhz as f64 * TICKS_PER_2MHZ).round() as u64;
                smf.tracks[0].events.push(TrackEvent {
                    vtime: total_elapsed_ticks - previous_elapsed_ticks,
                    event: MidiEvent::Midi(MidiMessage {
                        data: msg.data[..msg.length].to_vec(),
                    }),
                });
                previous_elapsed_ticks = total_elapsed_ticks;
            }
            total_output_msgs += 1;
        }

        // SMF書き出し
        let writer = SMFWriter::from_smf(smf);
        writer.write_to_file(output_smf_path).unwrap();
    }

    Ok(())
}
