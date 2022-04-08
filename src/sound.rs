use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

pub struct Audio {
    sink: Sink,
    _stream: OutputStream,
    perfect_sound_bytes: &'static [u8],
    misplaced_sound_bytes: &'static [u8],
    not_found_sound_bytes: &'static [u8],
}

pub fn init() -> Audio {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let perfect_sound_bytes = include_bytes!("../ressources/perfect.wav");
    let misplaced_sound_bytes = include_bytes!("../ressources/misplaced.wav");
    let not_found_sound_bytes = include_bytes!("../ressources/not_found.wav");

    Audio {
        sink,
        _stream,
        perfect_sound_bytes,
        misplaced_sound_bytes,
        not_found_sound_bytes,
    }
}

pub fn play_perfect_sound(audio: &Audio) {
    let audio_slice = Cursor::new(audio.perfect_sound_bytes.as_ref());
    audio.sink.append(Decoder::new_wav(audio_slice).unwrap());
    audio.sink.sleep_until_end();
}

pub fn play_misplaced_sound(audio: &Audio) {
    let audio_slice = Cursor::new(audio.misplaced_sound_bytes.as_ref());
    audio.sink.append(Decoder::new_wav(audio_slice).unwrap());
    audio.sink.sleep_until_end();
}
pub fn play_not_found_sound(audio: &Audio) {
    let audio_slice = Cursor::new(audio.not_found_sound_bytes.as_ref());
    audio.sink.append(Decoder::new_wav(audio_slice).unwrap());
    audio.sink.sleep_until_end();
}
