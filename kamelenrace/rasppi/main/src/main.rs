use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, source::Source};
use std::fs;

fn main () {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Get all file paths 
    let mut paths = fs::read_dir("../audio/fx/").unwrap();

    println!("Paths to fx sounds:");
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
    
    paths = fs::read_dir("../audio/idle/").unwrap();

    println!("Paths to idle music:");
    for path in paths {
        println!("{}", path.unwrap().path().display());
    }


    // Arrays containing file paths
    let fx_paths = ["../audio/fx/lessgo.mp3", "../audio/fx/mario-kart.mp3"];
    let idle_paths = ["../audio/idle/cleopatra.mp3", "../audio/idle/prince-of-egypt.mp3", "../audio/idle/princess-of-persia.mp3", "../audio/idle/sands-of-arabia.mp3"];


    // Load file from path list - path relative form Cargo.toml
    let fx = BufReader::new(File::open(fx_paths[0]).unwrap());
    let idle = BufReader::new(File::open(idle_paths[0]).unwrap());

    // Decode and play sound
    let source = Decoder::new(idle).unwrap();
    sink.append(source);

    sink.sleep_until_end();
    
    // stream_handle.play_raw(source.convert_samples());


    // Keep program active to resume playing sound
    // std::thread::sleep(std::time::Duration::from_secs(2));
}