use std::slice::from_raw_parts;
use std::time::Duration;

use consts::SAMPLES_PER_FRAME;
use enums::{Layer, Version};
use types::Url;

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

pub fn create_utf16_str(buf: &[u8], offset: usize, len: usize) -> String {
    //let tmp_v = buf[offset..offset + len].to_vec();
    let x = unsafe { from_raw_parts(buf.as_ptr().offset(offset as isize) as *const u16, len / 2) };
    String::from_utf16(x).unwrap_or(String::new())
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
    if buf[pos] == 3 {
        create_str(buf, pos + 1, size as usize - 1)
    } else {
        // if `c` == 0, it's supposed to be ISO-8859-1, `String` doesn't handle it.
        let mut s = create_utf16_str(buf, pos + 1, size as usize - 1);
        // It adds a '\u{feff}' character at the beginning of the string.
        if let Some(c) = s.chars().next() {
            if c == '\u{feff}' {
                s.remove(0);
            }
        }
        s
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
