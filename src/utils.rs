use std::time::Duration;

use consts::SAMPLES_PER_FRAME;
use enums::{Layer, Version};
use types::Url;

pub fn compute_duration(v: Version, l: Layer, sample_rate: u16) -> Option<Duration> {
    if sample_rate == 0 {
        return None;
    }
    let mut big = match v {
        Version::MPEG1 => SAMPLES_PER_FRAME[0][get_layer_value(l)] as u64 * 1_000_000_000,
        Version::MPEG2 | Version::MPEG2_5 => {
            SAMPLES_PER_FRAME[1][get_layer_value(l)] as u64 * 1_000_000_000
        }
        _ => return None,
    };
    big /= sample_rate as u64;
    Some(Duration::new(big / 1_000_000_000, (big % 1_000_000_000) as u32))
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
        _ => 1,
    }
}

pub fn create_latin1_str(buf: &[u8]) -> String {
    // interpret each byte as full codepoint. UTF-16 is big enough to
    // represent those, surrogate pairs can't be created that way
    let utf16 = buf.iter().map(|c| *c as u16).collect::<Vec<u16>>();
    String::from_utf16_lossy(utf16.as_ref())
}

pub fn create_utf16_str(buf: &[u8]) -> String {
    let mut v = Vec::<u16>::new();
    if buf.len() >= 2 {
        // BOM: \u{feff}
        if buf[0] == 0xfe && buf[1] == 0xff {
            // UTF-16BE
            v.reserve(buf.len() / 2 - 1);
            for i in 1..buf.len() / 2 {
                v.push(
                    (buf[2*i+0] as u16) << 8
                    | (buf[2*i+1] as u16)
                )
            }
            return String::from_utf16_lossy(v.as_ref());
        } else if buf[0] == 0xff && buf[1] == 0xfe {
            // UTF-16LE
            v.reserve(buf.len() / 2 - 1);
            for i in 1..buf.len() / 2 {
                v.push(
                    (buf[2*i+1] as u16) << 8
                    | (buf[2*i+0] as u16)
                )
            }
            return String::from_utf16_lossy(v.as_ref());
        }
    }
    // try as UTF-16LE
    v.reserve(buf.len() / 2);
    for i in 0..buf.len() / 2 {
        v.push(
            (buf[2*i+1] as u16) << 8
            | (buf[2*i+0] as u16)
        )
    }
    return String::from_utf16_lossy(v.as_ref());
}

pub fn create_utf8_str(buf: &[u8]) -> String {
    // String::from_utf8_lossy(buf).into_owned()
    String::from_utf8(buf.to_owned()).unwrap_or(String::new())
}

pub fn get_url_field(buf: &[u8], pos: usize, size: u32, changes: &mut bool,
                     value: &mut Option<Url>) {
    if value.is_some() || size < 2 {
        return;
    }
    if *changes == false {
        *changes = true;
    }
    let tmp_v = buf[pos..pos + size as usize].to_vec();
    *value = Some(Url(String::from_utf8(tmp_v).unwrap_or(String::new())));
}

pub fn get_url_fields(buf: &[u8], pos: usize, size: u32, changes: &mut bool,
                      value: &mut Vec<Url>) {
    let mut tmp = None;
    get_url_field(buf, pos, size, changes, &mut tmp);
    if let Some(tmp) = tmp {
        value.push(tmp);
    }
}

pub fn get_field(buf: &[u8], pos: usize, size: u32) -> String {
    let buf = &buf[pos..][..size as usize];
    if buf.len() < 1 {
        String::new()
    } else if buf[0] == 0 {
        // ISO-8859-1
        create_latin1_str(&buf[1..])
    } else if buf[0] == 1 {
        // UTF-16, requires a BOM
        create_utf16_str(&buf[1..])
    } else if buf[0] == 3 {
        // UTF-8
        create_utf8_str(&buf[1..])
    } else {
        String::new()
    }
}

pub fn get_text_field(buf: &[u8], pos: usize, size: u32, changes: &mut bool,
                      value: &mut Option<String>) {
    if value.is_some() || size < 2 {
        return;
    }
    if *changes == false {
        *changes = true;
    }
    *value = Some(get_field(buf, pos, size));
}

pub fn get_text_fields(buf: &[u8], pos: usize, size: u32, changes: &mut bool,
                       value: &mut Vec<String>) {
    let tmp = get_field(buf, pos, size);
    let tmp_v = tmp.split("/");
    for entry in tmp_v {
        if entry.len() > 0 {
            value.push(entry.to_owned());
        }
    }
    if *changes == false {
        *changes = true;
    }
}
