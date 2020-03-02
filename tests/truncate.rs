extern crate mp3_metadata;

use std::time::Duration;

#[test]
fn truncate() {
    let meta = mp3_metadata::read_from_file("assets/trunc_test.mp3").expect("File error");
    if let Some(frame) = meta.frames.first() {
        assert_eq!(frame.size, 417);
        assert_eq!(frame.version, mp3_metadata::Version::MPEG1);
        assert_eq!(frame.layer, mp3_metadata::Layer::Layer3);
        assert_eq!(frame.crc, mp3_metadata::CRC::Added);
        assert_eq!(frame.bitrate, 128);
        assert_eq!(frame.sampling_freq, 44100);
        assert_eq!(frame.padding, false);
        assert_eq!(frame.private_bit, false);
        assert_eq!(frame.chan_type, mp3_metadata::ChannelType::SingleChannel);
        assert_eq!(frame.intensity_stereo, false);
        assert_eq!(frame.ms_stereo, false);
        assert_eq!(frame.copyright, mp3_metadata::Copyright::None);
        assert_eq!(frame.status, mp3_metadata::Status::Copy);
        assert_eq!(frame.emphasis, mp3_metadata::Emphasis::None);
    }
    assert_eq!(meta.duration, Duration::new(12, 120815872));
    assert_eq!(meta.tag, None);
}
