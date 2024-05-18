use std::convert::From;
use std::default::Default;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    FileError,
    NotMP3,
    NoHeader,
    DuplicatedIDV3,
    InvalidData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err = match *self {
            Self::FileError => "An I/O error occurred",
            Self::NotMP3 => "The file is not a valid MP3 file",
            Self::NoHeader => "The file is missing an MP3 header",
            Self::DuplicatedIDV3 => "The MP3 file contains a duplicate IDv3 frame",
            Self::InvalidData => "The MP3 metadata is invalid",
        };
        err.fmt(f)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Version {
    Reserved,
    MPEG1,
    MPEG2,
    MPEG2_5,
    Unknown,
}

impl Default for Version {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u32> for Version {
    fn from(c: u32) -> Self {
        match c {
            0x00 => Self::MPEG2_5,
            0x01 => Self::Reserved,
            0x02 => Self::MPEG2,
            0x03 => Self::MPEG1,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Layer {
    Reserved,
    Layer1,
    Layer2,
    Layer3,
    Unknown,
}

impl Default for Layer {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u32> for Layer {
    fn from(c: u32) -> Self {
        match c {
            0x0 => Self::Reserved,
            0x1 => Self::Layer3,
            0x2 => Self::Layer2,
            0x3 => Self::Layer1,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CRC {
    /// Redundancy added.
    Added,
    /// Redundancy not added.
    NotAdded,
}

impl Default for CRC {
    fn default() -> Self {
        Self::NotAdded
    }
}

impl From<u32> for CRC {
    fn from(c: u32) -> Self {
        match c {
            0x00 => Self::Added,
            0x01 => Self::NotAdded,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChannelType {
    Stereo,
    JointStereo,
    DualChannel,
    SingleChannel,
    Unknown,
}

impl Default for ChannelType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u32> for ChannelType {
    fn from(c: u32) -> Self {
        match c {
            0x0 => Self::Stereo,
            0x1 => Self::JointStereo,
            0x2 => Self::DualChannel,
            0x3 => Self::SingleChannel,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Copyright {
    None,
    Some,
}

impl Default for Copyright {
    fn default() -> Self {
        Self::Some
    }
}

impl From<u32> for Copyright {
    fn from(c: u32) -> Self {
        match c {
            0x0 => Self::None,
            0x1 => Self::Some,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Copy,
    Original,
    Unknown,
}

impl Default for Status {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u32> for Status {
    fn from(c: u32) -> Self {
        match c {
            0x0 => Self::Copy,
            0x1 => Self::Original,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Emphasis {
    /// No emphasis
    None,
    /// 50/15 Micro seconds
    MicroSeconds,
    /// Reserved
    Reserved,
    /// CCIT J.17
    CCITT,
    Unknown,
}

impl Default for Emphasis {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<u32> for Emphasis {
    fn from(c: u32) -> Self {
        match c {
            0x0 => Self::None,
            0x1 => Self::MicroSeconds,
            0x2 => Self::Reserved,
            0x3 => Self::CCITT,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Genre {
    Blues,
    ClassicRock,
    Country,
    Dance,
    Disco,
    Funk,
    Grunge,
    HipHop,
    Jazz,
    Metal,
    NewAge,
    Oldies,
    Other,
    Pop,
    RAndB,
    Rap,
    Reggae,
    Rock,
    Techno,
    Industrial,
    Alternative,
    Ska,
    DeathMetal,
    Pranks,
    Soundtrack,
    EuroTechno,
    Ambient,
    TripHop,
    Vocal,
    JazzFunk,
    Fusion,
    Trance,
    Classical,
    Instrumental,
    Acid,
    House,
    Game,
    SoundClip,
    Gospel,
    Noise,
    AlternRock,
    Bass,
    Soul,
    Punk,
    Space,
    Meditative,
    InstrumentalPop,
    InstrumentalRock,
    Ethnic,
    Gothic,
    Darkwave,
    TechnoIndustrial,
    Electronic,
    PopFolk,
    Eurodance,
    Dream,
    SouthernRock,
    Comedy,
    Cult,
    Gangsta,
    Top40,
    ChristianRap,
    PopFunk,
    Jungle,
    NativeAmerican,
    Cabaret,
    NewWave,
    Psychadelic,
    Rave,
    Showtunes,
    Trailer,
    LoFi,
    Tribal,
    AcidPunk,
    AcidJazz,
    Polka,
    Retro,
    Musical,
    RockAndRoll,
    HardRock,
    // Extension from here
    Folk,
    FolkRock,
    NationalFolk,
    Swing,
    FastFusion,
    Bebob,
    Latin,
    Revival,
    Celtic,
    Bluegrass,
    Avantgarde,
    GothicRock,
    ProgressiveRock,
    PsychedelicRock,
    SymphonicRock,
    SlowRock,
    BigBand,
    Chorus,
    EasyListening,
    Acoustic,
    Humour,
    Speech,
    Chanson,
    Opera,
    ChamberMusic,
    Sonata,
    Symphony,
    BootyBrass,
    Primus,
    PornGroove,
    Satire,
    SlowJam,
    Club,
    Tango,
    Samba,
    Folklore,
    Ballad,
    PowerBallad,
    RhytmicSoul,
    Freestyle,
    Duet,
    PunkRock,
    DrumSolo,
    ACapela,
    EuroHouse,
    DanceHall,
    Something(String),
    Unknown,
}

impl Default for Genre {
    fn default() -> Self {
        Self::Unknown
    }
}

impl<'a> From<&'a str> for Genre {
    fn from(c: &'a str) -> Self {
        c.parse::<u8>()
            .map_or_else(|_| Self::Something(c.to_owned()), Self::from)
    }
}

impl From<u8> for Genre {
    #[allow(clippy::too_many_lines)]
    fn from(c: u8) -> Self {
        match c {
            0 => Self::Blues,
            1 => Self::ClassicRock,
            2 => Self::Country,
            3 => Self::Dance,
            4 => Self::Disco,
            5 => Self::Funk,
            6 => Self::Grunge,
            7 => Self::HipHop,
            8 => Self::Jazz,
            9 => Self::Metal,
            10 => Self::NewAge,
            11 => Self::Oldies,
            12 => Self::Other,
            13 => Self::Pop,
            14 => Self::RAndB,
            15 => Self::Rap,
            16 => Self::Reggae,
            17 => Self::Rock,
            18 => Self::Techno,
            19 => Self::Industrial,
            20 => Self::Alternative,
            21 => Self::Ska,
            22 => Self::DeathMetal,
            23 => Self::Pranks,
            24 => Self::Soundtrack,
            25 => Self::EuroTechno,
            26 => Self::Ambient,
            27 => Self::TripHop,
            28 => Self::Vocal,
            29 => Self::JazzFunk,
            30 => Self::Fusion,
            31 => Self::Trance,
            32 => Self::Classical,
            33 => Self::Instrumental,
            34 => Self::Acid,
            35 => Self::House,
            36 => Self::Game,
            37 => Self::SoundClip,
            38 => Self::Gospel,
            39 => Self::Noise,
            40 => Self::AlternRock,
            41 => Self::Bass,
            42 => Self::Soul,
            43 => Self::Punk,
            44 => Self::Space,
            45 => Self::Meditative,
            46 => Self::InstrumentalPop,
            47 => Self::InstrumentalRock,
            48 => Self::Ethnic,
            49 => Self::Gothic,
            50 => Self::Darkwave,
            51 => Self::TechnoIndustrial,
            52 => Self::Electronic,
            53 => Self::PopFolk,
            54 => Self::Eurodance,
            55 => Self::Dream,
            56 => Self::SouthernRock,
            57 => Self::Comedy,
            58 => Self::Cult,
            59 => Self::Gangsta,
            60 => Self::Top40,
            61 => Self::ChristianRap,
            62 => Self::PopFunk,
            63 => Self::Jungle,
            64 => Self::NativeAmerican,
            65 => Self::Cabaret,
            66 => Self::NewWave,
            67 => Self::Psychadelic,
            68 => Self::Rave,
            69 => Self::Showtunes,
            70 => Self::Trailer,
            71 => Self::LoFi,
            72 => Self::Tribal,
            73 => Self::AcidPunk,
            74 => Self::AcidJazz,
            75 => Self::Polka,
            76 => Self::Retro,
            77 => Self::Musical,
            78 => Self::RockAndRoll,
            79 => Self::HardRock,
            80 => Self::Folk,
            81 => Self::FolkRock,
            82 => Self::NationalFolk,
            83 => Self::Swing,
            84 => Self::FastFusion,
            85 => Self::Bebob,
            86 => Self::Latin,
            87 => Self::Revival,
            88 => Self::Celtic,
            89 => Self::Bluegrass,
            90 => Self::Avantgarde,
            91 => Self::GothicRock,
            92 => Self::ProgressiveRock,
            93 => Self::PsychedelicRock,
            94 => Self::SymphonicRock,
            95 => Self::SlowRock,
            96 => Self::BigBand,
            97 => Self::Chorus,
            98 => Self::EasyListening,
            99 => Self::Acoustic,
            100 => Self::Humour,
            101 => Self::Speech,
            102 => Self::Chanson,
            103 => Self::Opera,
            104 => Self::ChamberMusic,
            105 => Self::Sonata,
            106 => Self::Symphony,
            107 => Self::BootyBrass,
            108 => Self::Primus,
            109 => Self::PornGroove,
            110 => Self::Satire,
            111 => Self::SlowJam,
            112 => Self::Club,
            113 => Self::Tango,
            114 => Self::Samba,
            115 => Self::Folklore,
            116 => Self::Ballad,
            117 => Self::PowerBallad,
            118 => Self::RhytmicSoul,
            119 => Self::Freestyle,
            120 => Self::Duet,
            121 => Self::PunkRock,
            122 => Self::DrumSolo,
            123 => Self::ACapela,
            124 => Self::EuroHouse,
            125 => Self::DanceHall,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Something(s) => write!(f, "{s}"),
            _ => fmt::Debug::fmt(self, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fmt_genre() {
        assert_eq!(Genre::Club.to_string(), "Club");
        assert_eq!(Genre::Something("Foo".to_string()).to_string(), "Foo");
    }
}
