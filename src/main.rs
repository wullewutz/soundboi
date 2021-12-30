use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use rodio::source::{Buffered, Source};
use rodio::{Decoder, OutputStream};

use dirs::config_dir;

type SoundBoard = HashMap<char, Buffered<Decoder<BufReader<File>>>>;

fn build_soundboard(p: PathBuf) -> Result<SoundBoard, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    let files = p.read_dir()?.flatten();
    for f in files {
        let buf = BufReader::new(File::open(f.path())?);
        let decoder = Decoder::new(buf)?.buffered();
        let (key, stem) = extract_meta(f.path())?;
        map.insert(key, decoder);
        println!("{}: {}", key, stem);
    }
    Ok(map)
}

fn extract_meta(f: PathBuf) -> Result<(char, String), Box<dyn std::error::Error>> {
    let stem = f
        .file_stem()
        .ok_or("Could not extract file stem from name")?
        .to_str()
        .ok_or("Could not convert file name to utf8 string")?;
    let key: char = stem
        .chars()
        .next()
        .ok_or("Could not extract first character of filename")?;
    let stem: String = if stem.chars().nth(1) == Some('_') {
        stem.split('_')
            .nth(1)
            .ok_or("unable to split filename at '_'")?
            .to_string()
    } else {
        stem.to_string()
    };
    Ok((key, stem))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let dir = config_dir()
        .ok_or("Could not detect config dir")?
        .join("soundboi");
    let map = build_soundboard(dir)?;
    enable_raw_mode()?;
    loop {
        let event = read()?;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Char(x),
                modifiers: KeyModifiers::NONE,
            }) => {
                if let Some(src) = map.get(&x) {
                    stream_handle.play_raw(src.clone().convert_samples())?;
                }
            }
            _ => (), // println!("Event {:?}\r", event),
        }
    }
    disable_raw_mode()?;
    Ok(())
}
