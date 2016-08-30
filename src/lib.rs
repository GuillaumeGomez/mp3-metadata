extern crate libc;

use libc::c_void;

use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::convert::AsRef;
use std::mem;
use std::path::Path;
use std::time::Duration;

pub use enums::{Error, Genre, Version, Layer, CRC, ChannelType, Copyright, Status, Emphasis};

mod enums;

const BITRATES: [[u16; 16]; 5] = [
[0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 0],
[0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 0],
[0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0],
[0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256, 0],
[0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0]
];
const SAMPLING_FREQ: [[u16; 4]; 4] = [
[44100, 48000, 32000, 0],
[22050, 24000, 16000, 0],
[11025, 12000, 8000, 0],
[0, 0, 0, 0],
];
const SAMPLES_PER_FRAME: [[u32; 4]; 2] = [
[384, 1152, 1152, 0],
[384, 1152, 576, 0],
];

fn compute_duration(v: Version, l: Layer, sample_rate: u16) -> Option<Duration> {
    if sample_rate == 0 {
        return None;
    }
    Some(Duration::from_millis(match v {
        Version::MPEG1 => SAMPLES_PER_FRAME[0][get_layer_value(l)] * 1000,
        Version::MPEG2 | Version::MPEG2_5 => SAMPLES_PER_FRAME[1][get_layer_value(l)] * 1000,
        _ => return None,
    } as u64 / sample_rate as u64))
}

fn get_line(v: Version, l: Layer) -> usize {
    match (v, l) {
        (Version::MPEG1, Layer::Layer1) => 0,
        (Version::MPEG1, Layer::Layer2) => 1,
        (Version::MPEG1, Layer::Layer3) => 2,
        (Version::MPEG2, Layer::Layer1) | (Version::MPEG2_5, Layer::Layer1) => 3,
        _ => 4,
    }
}

fn get_layer_value(l: Layer) -> usize {
    match l {
        Layer::Layer1 => 0,
        Layer::Layer2 => 1,
        Layer::Layer3 => 2,
        _ => 3,
    }
}

fn get_samp_line(v: Version) -> usize {
    match v {
        Version::MPEG1 => 0,
        Version::MPEG2 => 1,
        Version::MPEG2_5 => 2,
        _ => 3,
    }
}

pub fn read_from_file<P>(file: P) -> Result<Vec<Frame>, Error>
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

pub fn read_from_slice(buf: &[u8]) -> Result<Vec<Frame>, Error> {
    /*match MP3Header::new(buf) {
        Some(h) => {
            if h.tag[0..3] != ['I' as i8, 'D' as i8, '3' as i8] {
                return Err(Error::NotMP3);
            }
            Ok(h)
        }
        None => Err(Error::NoHeader)
    }*/
    let mut frames: Vec<Frame> = Vec::new();
    let mut i = 0u32;

    'a: while i < buf.len() as u32 {
        let mut c;
        let mut frame: Frame = Default::default();
        loop {
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
                    if let Some(last) = frames.last_mut() {
                        last.size = i - last.size;
                    }
                    frame.size = i;

                    frame.version = Version::from((c & 0x18) >> 3);
                    frame.layer = Layer::from(c & 0x06);
                    frame.crc = CRC::from(c & 0x01);

                    i += 1;
                    if i >= buf.len() as u32 {
                        frames.push(frame);
                        break 'a;
                    }
                    c = buf[i as usize];

                    frame.bitrate = BITRATES[get_line(frame.version,
                                                      frame.layer)][((c & 0xF0) >> 4) as usize];
                    frame.sampling_freq = SAMPLING_FREQ[get_samp_line(frame.version)][((c & 0x0C) >> 2) as usize];
                    frame.slot = c & 0x02 == 0x02;
                    frame.private_bit = c & 0x01 == 1;

                    i += 1;
                    if i >= buf.len() as u32 {
                        frames.push(frame);
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
                    frame.duration = compute_duration(frame.version, frame.layer, frame.sampling_freq);

                    frames.push(frame);
                    break;
                }
            }
        }
        i += 1;
    }
    if let Some(last) = frames.last_mut() {
        last.size = i - last.size - 1;
    }
    Ok(frames)
}

#[derive(Debug, Default)]
pub struct Frame {
    pub size: u32,
    pub version: Version,
    pub layer: Layer,
    pub crc: CRC,
    pub bitrate: u16,
    pub sampling_freq: u16,
    pub slot: bool, // slot to adjust bitrate
    pub private_bit: bool,
    pub chan_type: ChannelType,
    pub intensity_stereo: bool,
    pub ms_stereo: bool,
    pub copyright: Copyright,
    pub status: Status,
    pub emphasis: Emphasis,
    pub duration: Option<Duration>,
}

pub struct MP3Metadata {
    pub duration: Duration,
    pub frames: Vec<Frame>,
    pub tag: Option<AudioTag>,
}

pub struct AudioTag {
    pub title: String,
    pub album: String,
    pub year: u32,
    pub comment: String,
    pub genre: Genre,
}

pub struct MP3Header {
    pub tag: [i8; 3],
    pub maj_ver: u8,
    pub min_ver: u8,
    pub flags: u8,
    pub size: u32,
}

impl MP3Header {
    pub fn new(buf: &[u8]) -> Option<MP3Header> {
        if buf.len() < 10 {
            None
        } else {
            let mut header: MP3Header = unsafe { mem::zeroed() };
            unsafe { libc::memcpy((&mut header) as *mut MP3Header as *mut c_void,
                                  buf.as_ptr() as *const c_void, 10); }
            if header.flags & 0x40 != 0 {
                // read ext header
            }
            Some(header)
        }
    }
}

struct MP3ExtHeader {
    size: u32,
    num_flag_bytes: u8,
    extended_flags: u8,
}

struct MP3FrameHeader {
    frame_id: [i8; 4],
    size: u32,
    flags: [u8; 2],
}
