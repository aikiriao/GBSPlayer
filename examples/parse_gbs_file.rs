use sm83::gbs_file::*;
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
        println!(
            "Version: {} \n\
            Number of Songs: {} \n\
            First Song: {} \n\
            Load Address: {:#X} \n\
            Init Address: {:#X} \n\
            Play Address: {:#X} \n\
            Stack Pointer: {:#X} \n\
            Timer Modulo: {:#X} \n\
            Timer Control: {:#X} \n\
            Title: {} \n\
            Author: {} \n\
            Copyright: {} \n",
            header.version,
            header.num_songs,
            header.first_song,
            header.load_address,
            header.init_address,
            header.play_address,
            header.stack_pointer,
            header.timer_modulo,
            header.timer_control,
            std::str::from_utf8(&header.title)
                .unwrap()
                .trim_end_matches('\0'),
            std::str::from_utf8(&header.author)
                .unwrap()
                .trim_end_matches('\0'),
            std::str::from_utf8(&header.copyright)
                .unwrap()
                .trim_end_matches('\0'),
        );
    }

    Ok(())
}
