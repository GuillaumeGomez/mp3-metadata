use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use consts::{BITRATES, SAMPLING_FREQ};
use enums::{ChannelType, Copyright, CRC, Emphasis, Error, Genre, Layer, Status, Version};
use types::{AudioTag, Frame, MP3Metadata};
use utils::{compute_duration, create_str, get_line, get_samp_line};

pub fn read_from_file<P>(file: P) -> Result<MP3Metadata, Error>
where P: AsRef<Path> {
    if let Some(mut fd) = File::open(file).ok() {
        let mut buf = Vec::new();

        match fd.read_to_end(&mut buf) {
            Ok(_) => read_from_slice(&buf),
            Err(_) => Err(Error::FileError),
        }
    } else {
        Err(Error::FileError)
    }
}

pub fn read_from_slice(buf: &[u8]) -> Result<MP3Metadata, Error> {
    let mut meta = MP3Metadata {
        frames: Vec::new(),
        duration: Duration::new(0, 0),
        tag: None,
    };
    let mut i = 0u32;

    'a: while i < buf.len() as u32 {
        let mut c = buf[i as usize];
        let mut frame: Frame = Default::default();
        loop {
            if c == 'T' as u8 {
                let x = i as usize;
                // Get extended information
                if x <= buf.len() - 128 && buf[x + 1] == 'A' as u8 && buf[x + 2] == 'G' as u8 {
                    if let Some(last) = meta.frames.last_mut() {
                        last.size = i - last.size - 1;
                    }
                    // tag v1
                    meta.tag = Some(AudioTag {
                        title: create_str(buf, x + 3, 30),
                        artist: create_str(buf, x + 33, 30),
                        album: create_str(buf, x + 63, 30),
                        year: create_str(buf, x + 93, 4).parse::<u16>().unwrap_or(0),
                        comment: create_str(buf, x + 97,
                                            if buf[x + 97 + 28] != 0 { 30 } else { 28 }),
                        genre: Genre::from(buf[x + 127]),
                    });
                    i += 127;
                }
            }
            i += 1;
            if i >= buf.len() as u32 {
                break 'a;
            }
            c = buf[i as usize];
            if c == 0xFF {
                i += 1;
                if i >= buf.len() as u32 {
                    break 'a;
                }
                c = buf[i as usize];
                if c == 0xFA || c == 0xFB {
                    if let Some(last) = meta.frames.last_mut() {
                        last.size = i - last.size;
                    }
                    frame.size = i;

                    frame.version = Version::from((c & 0x18) >> 3);
                    frame.layer = Layer::from(c & 0x06);
                    frame.crc = CRC::from(c & 0x01);

                    i += 1;
                    if i >= buf.len() as u32 {
                        meta.frames.push(frame);
                        break 'a;
                    }
                    c = buf[i as usize];

                    frame.bitrate = BITRATES[get_line(frame.version,
                                                      frame.layer)][((c & 0xF0) >> 4) as usize];
                    frame.sampling_freq = SAMPLING_FREQ[get_samp_line(frame.version)]
                                                       [((c & 0x0C) >> 2) as usize];
                    frame.slot = c & 0x02 == 0x02;
                    frame.private_bit = c & 0x01 == 1;

                    i += 1;
                    if i >= buf.len() as u32 {
                        meta.frames.push(frame);
                        break 'a;
                    }
                    c = buf[i as usize];

                    frame.chan_type = ChannelType::from(c & 0xC0);
                    let (intensity, ms_stereo) = match (c & 0x30) >> 4 {
                        0x10 => (true, false),
                        0x20 => (false, true),
                        0x30 => (true, true),
                        /*0x00*/ _ => (false, false),
                    };
                    frame.intensity_stereo = intensity;
                    frame.ms_stereo = ms_stereo;
                    frame.copyright = Copyright::from(c & 0x08);
                    frame.status = Status::from(c & 0x04);
                    frame.emphasis = Emphasis::from(c & 0x03);
                    frame.duration = compute_duration(frame.version,
                                                      frame.layer,
                                                      frame.sampling_freq);

                    if let Some(dur) = frame.duration {
                        meta.duration += dur;
                    }

                    meta.frames.push(frame);
                    break;
                }
            }
        }
        i += 1;
    }
    if meta.tag.is_none() {
        if let Some(last) = meta.frames.last_mut() {
            last.size = i - last.size - 1;
        }
    }
    Ok(meta)
}