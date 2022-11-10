use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main () {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let fx1 = BufReader::new(File::open("../audio/fx/lessgo.mp3").unwrap());
    let fx2 = BufReader::new(File::open("../audio/fx/mario-kart.mp3").unwrap());

    let idle1 = BufReader::new(File::open("../audio/idle/cleopatra.mp3").unwrap());
    let idle2 = BufReader::new(File::open("../audio/idle/prince-of-egypt.mp3").unwrap());
    let idle3 = BufReader::new(File::open("../audio/idle/princess-of-persia.mp3").unwrap());
    let idle4 = BufReader::new(File::open("../audio/idle/sands-of-arabia.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(fx2).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(2));
}