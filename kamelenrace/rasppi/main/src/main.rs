use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::fs;
use rust_gpiozero::*;

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
    let racing_paths = [""];

    // Load file from path list - path relative form Cargo.toml
    let fx = BufReader::new(File::open(fx_paths[0]).unwrap());
    let idle = BufReader::new(File::open(idle_paths[0]).unwrap());
    let racing = BufReader::new(File::open(racing_paths[0]).unwrap());

    // Decode and play sound
    let source_idle = Decoder::new(idle).unwrap();
    let source_fx = Decoder::new(fx).unwrap();
    let source_racing = Decoder::new(racing).unwrap();
    sink.append(source_idle);

    // loop {
    //     // check start pin value
    //     // if sink = empty
    //         // append idle song
        
    //     if pin_value == true {
    //         sink.stop();
    //         sink.append(source_fx);
    //         sink.append(source_racing);
    //         // enable racing pin
    //         // disable racing pin
    //         // check finished pin
    //         // while finished pin == false
    //             // check finished pin
    //             // if sink = empty
    //             // append racing song
    //         // stop sink
    //         // play lane effect
    //         // sleep_until_finished
    //     }
    // }
    
    // stream_handle.play_raw(source.convert_samples());


    // Keep program active to resume playing sound
    // std::thread::sleep(std::time::Duration::from_secs(2));
}