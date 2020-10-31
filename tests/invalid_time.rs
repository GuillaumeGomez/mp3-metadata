extern crate mp3_metadata;
extern crate simplemad;

use std::time::Duration;

use std::fs::File;

mod common;

// Still invalid for the moment as it seems...

#[test]
fn invalid_time() {
    common::get_file("assets/invalid_time.mp3");
    let meta = mp3_metadata::read_from_file("assets/invalid_time.mp3").expect("File error");
    let file = File::open("assets/invalid_time.mp3").unwrap();
    let decoder = simplemad::Decoder::decode(file).unwrap();
    let mut i = 0;
    let mut sum = Duration::new(0, 0);
    for decoding_result in decoder {
        match decoding_result {
            Err(_) => {
                //println!("Error: {:?} {:?}", e, meta.frames[i]);
            }
            Ok(frame) => {
                if i >= meta.frames.len() {
                    println!("==> {} > {}", i, meta.frames.len());
                    i += 1;
                    continue;
                }
                if meta.frames[i].sampling_freq as u32 != frame.sample_rate {
                    println!(
                        "[{}] [SAMPLE_RATE] {} != {}",
                        i, meta.frames[i].sampling_freq, frame.sample_rate
                    );
                }
                if meta.frames[i].bitrate as u32 * 1000 != frame.bit_rate {
                    println!(
                        "[{}] [BIT_RATE] {} != {}",
                        i,
                        meta.frames[i].bitrate as u32 * 1000,
                        frame.bit_rate
                    );
                }
                if meta.frames[i].duration.unwrap() != frame.duration {
                    println!(
                        "[{}] [DURATION] {:?} != {:?}",
                        i, meta.frames[i].duration, frame.duration
                    );
                }
                if meta.frames[i].position != frame.position {
                    println!(
                        "[{}] [POSITION] {:?} != {:?}",
                        i, meta.frames[i].position, frame.position
                    );
                }
                sum += frame.duration;
            }
        }
        i += 1;
    }
    //assert_eq!(meta.duration, Duration::new(162, 611095984));
    //assert_eq!(meta.duration, sum);
}
