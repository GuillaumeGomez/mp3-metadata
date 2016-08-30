use std::time::Duration;

use enums::{ChannelType, Copyright, CRC, Emphasis, Genre, Layer, Status, Version};

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct MP3Metadata {
    pub duration: Duration,
    pub frames: Vec<Frame>,
    pub tag: Option<AudioTag>,
}

#[derive(Debug, PartialEq)]
pub struct AudioTag {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: u16,
    pub comment: String,
    pub genre: Genre,
}
