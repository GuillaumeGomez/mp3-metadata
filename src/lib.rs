pub use crate::{
    enums::{ChannelType, Copyright, Emphasis, Error, Genre, Layer, Status, Version, CRC},
    metadata::{read_from_file, read_from_slice},
    types::{AudioTag, Frame, MP3Metadata, OptionalAudioTags, Url},
};

mod consts;
mod enums;
mod metadata;
mod types;
mod utils;
