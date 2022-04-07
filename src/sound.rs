use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader};

pub struct Audio {
    sink: Sink,
    _stream: OutputStream,
}

pub fn init() -> Audio {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    Audio {
        sink,
        _stream,
    }
}

// fixme: find a way to open file once and not each play time
pub fn play_perfect_sound(audio: &Audio) {
    let file = File::open("ressources/perfect.wav").unwrap();
    audio
        .sink
        .append(Decoder::new_wav(BufReader::new(file)).unwrap());
    audio.sink.sleep_until_end();
}
pub fn play_misplaced_sound(audio: &Audio) {
    let file = File::open("ressources/misplaced.wav").unwrap();
    audio
        .sink
        .append(Decoder::new_wav(BufReader::new(file)).unwrap());
    audio.sink.sleep_until_end();
}
pub fn play_not_found_sound(audio: &Audio) {
    let file = File::open("ressources/not_found.wav").unwrap();
    audio
        .sink
        .append(Decoder::new_wav(BufReader::new(file)).unwrap());
    audio.sink.sleep_until_end();
}
