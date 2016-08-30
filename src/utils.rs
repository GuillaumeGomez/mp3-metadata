use std::time::Duration;

use consts::SAMPLES_PER_FRAME;
use enums::{Layer, Version};

pub fn compute_duration(v: Version, l: Layer, sample_rate: u16) -> Option<Duration> {
    if sample_rate == 0 {
        return None;
    }
    Some(Duration::from_millis(match v {
        Version::MPEG1 => SAMPLES_PER_FRAME[0][get_layer_value(l)] * 1000,
        Version::MPEG2 | Version::MPEG2_5 => SAMPLES_PER_FRAME[1][get_layer_value(l)] * 1000,
        _ => return None,
    } as u64 / sample_rate as u64))
}

pub fn get_line(v: Version, l: Layer) -> usize {
    match (v, l) {
        (Version::MPEG1, Layer::Layer1) => 0,
        (Version::MPEG1, Layer::Layer2) => 1,
        (Version::MPEG1, Layer::Layer3) => 2,
        (Version::MPEG2, Layer::Layer1) | (Version::MPEG2_5, Layer::Layer1) => 3,
        _ => 4,
    }
}

pub fn get_layer_value(l: Layer) -> usize {
    match l {
        Layer::Layer1 => 0,
        Layer::Layer2 => 1,
        Layer::Layer3 => 2,
        _ => 3,
    }
}

pub fn get_samp_line(v: Version) -> usize {
    match v {
        Version::MPEG1 => 0,
        Version::MPEG2 => 1,
        Version::MPEG2_5 => 2,
        _ => 3,
    }
}

pub fn create_str(buf: &[u8], offset: usize, len: usize) -> String {
    let tmp_v = buf[offset..offset + len].to_vec();
    String::from_utf8(tmp_v).unwrap_or(String::new())
}