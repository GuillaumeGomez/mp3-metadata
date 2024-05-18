extern crate mp3_metadata;
extern crate simplemad;

use std::time::Duration;

use std::fs::File;

#[test]
fn basic() {
    let meta = mp3_metadata::read_from_file("assets/test.mp3").expect("File error");
    let file = File::open("assets/test.mp3").unwrap();
    let decoder = simplemad::Decoder::decode(file).unwrap();
    let mut i = 0;
    let mut sum = Duration::new(0, 0);
    for decoding_result in decoder {
        match decoding_result {
            Err(_) => {}
            Ok(frame) => {
                if i >= meta.frames.len() {
                    println!(
                        "==> {} > {} {:?} {:?}",
                        i,
                        meta.frames.len(),
                        meta.frames.last().unwrap().duration,
                        frame.duration
                    );
                } else {
                    if u32::from(meta.frames[i].sampling_freq) != frame.sample_rate {
                        println!(
                            "[{}] [SAMPLE_RATE] {} != {}",
                            i, meta.frames[i].sampling_freq, frame.sample_rate
                        );
                        panic!();
                    }
                    if u32::from(meta.frames[i].bitrate) * 1000 != frame.bit_rate {
                        println!(
                            "[{}] [BIT_RATE] {} != {}",
                            i,
                            u32::from(meta.frames[i].bitrate) * 1000,
                            frame.bit_rate
                        );
                        panic!();
                    }
                    if meta.frames[i].duration.unwrap() != frame.duration {
                        println!(
                            "[{}] [DURATION] {:?} != {:?}",
                            i, meta.frames[i].duration, frame.duration
                        );
                        panic!();
                    }
                    if meta.frames[i].position != frame.position {
                        println!(
                            "[{}] [POSITION] {:?} != {:?}",
                            i, meta.frames[i].position, frame.position
                        );
                        panic!();
                    }
                }
                sum += frame.duration;
            }
        }
        i += 1;
    }
    if let Some(frame) = meta.frames.first() {
        assert_eq!(frame.size, 417, "frame size");
        assert_eq!(frame.version, mp3_metadata::Version::MPEG1, "version");
        assert_eq!(frame.layer, mp3_metadata::Layer::Layer3, "layer");
        assert_eq!(frame.crc, mp3_metadata::CRC::Added, "crc");
        assert_eq!(frame.bitrate, 128, "bitrate");
        assert_eq!(frame.sampling_freq, 44100, "sampling freq");
        assert!(!frame.padding, "padding");
        assert!(!frame.private_bit, "private bit");
        assert_eq!(
            frame.chan_type,
            mp3_metadata::ChannelType::SingleChannel,
            "channel type"
        );
        assert!(!frame.intensity_stereo, "intensity stereo");
        assert!(!frame.ms_stereo, "ms stereo");
        assert_eq!(frame.copyright, mp3_metadata::Copyright::None, "copyright");
        assert_eq!(frame.status, mp3_metadata::Status::Copy, "status");
        assert_eq!(frame.emphasis, mp3_metadata::Emphasis::None, "emphasis");
    }
    assert_eq!(meta.frames.len(), 475, "number of frames");
    assert_eq!(meta.duration, Duration::new(12, 408_162_800), "duration");
    assert_eq!(
        meta.tag,
        Some(mp3_metadata::AudioTag {
            title: "Test of MP3 File              ".to_owned(),
            artist: "Me                            ".to_owned(),
            album: "Me                            ".to_owned(),
            year: 2006,
            comment: "test                        ".to_owned(),
            genre: mp3_metadata::Genre::Other,
        }),
        "tag"
    );
    assert_eq!(meta.duration, sum, "time check");
}
