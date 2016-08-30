extern crate mp3_metadata;

use std::env;

fn main() {
    let file = match env::args().skip(1).next() {
        Some(f) => f,
        None => {
            println!("Need a music file parameter:");
            println!("./basic [music_file].mp3\n");
            println!("If you're using cargo, run it like this:");
            println!("cargo run -- [music_file].mp3");
            return
        }
    };
    let meta = mp3_metadata::read_from_file(file).expect("File error");

    println!("Number of frames: {}", meta.frames.len());
    println!("\nShowing 5 first frames information:");
    for frame in meta.frames[0..5].iter() {
        println!("========== NEW FRAME ==========");
        println!("size:               {}", frame.size);
        println!("version:            {:?}", frame.version);
        println!("layer:              {:?}", frame.layer);
        println!("bitrate:            {} Kb/s", frame.bitrate);
        println!("sampling frequency: {} Hz", frame.sampling_freq);
        println!("channel type:       {:?}", frame.chan_type);
        println!("copyright:          {:?}", frame.copyright);
        println!("status:             {:?}", frame.status);
        println!("emphasis:           {:?}", frame.emphasis);
    }

    println!("\n========== TAGS ==========");
    if let Some(tag) = meta.tag {
        println!("title: {}", tag.title);
        println!("artist: {}", tag.artist);
        println!("album: {}", tag.album);
        println!("year: {}", tag.year);
        println!("comment: {}", tag.comment);
        println!("genre: {:?}", tag.genre);
    } else {
        println!("No tag");
    }
}
