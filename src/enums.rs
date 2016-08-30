use std::convert::From;
use std::default::Default;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    FileError,
    NotMP3,
    NoHeader,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Version {
    Reserved,
    MPEG1,
    MPEG2,
    MPEG2_5,
    Unknown,
}

impl Default for Version {
    fn default() -> Version {
        Version::Unknown
    }
}

impl From<u8> for Version {
    fn from(c: u8) -> Version {
        match c {
            0x00 => Version::MPEG2_5,
            0x01 => Version::Reserved,
            0x02 => Version::MPEG2,
            0x03 => Version::MPEG1,
            _ => Version::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Layer {
    Reserved,
    Layer1,
    Layer2,
    Layer3,
    Unknown,
}

impl Default for Layer {
    fn default() -> Layer {
        Layer::Unknown
    }
}

impl From<u8> for Layer {
    fn from(c: u8) -> Layer {
        match c {
            0x00 => Layer::Reserved,
            0x02 => Layer::Layer3,
            0x04 => Layer::Layer2,
            0x06 => Layer::Layer1,
            _ => Layer::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CRC {
    /// Redundancy added.
    Added,
    /// Redundancy not added.
    NotAdded,
}

impl Default for CRC {
    fn default() -> CRC {
        CRC::NotAdded
    }
}

impl From<u8> for CRC {
    fn from(c: u8) -> CRC {
        match c {
            0x00 => CRC::Added,
            0x01 => CRC::NotAdded,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChannelType {
    Stereo,
    JointStereo,
    DualChannel,
    SingleChannel,
    Unknown,
}

impl Default for ChannelType {
    fn default() -> ChannelType {
        ChannelType::Unknown
    }
}

impl From<u8> for ChannelType {
    fn from(c: u8) -> ChannelType {
        match c {
            0x00 => ChannelType::Stereo,
            0x40 => ChannelType::JointStereo,
            0x80 => ChannelType::DualChannel,
            0xC0 => ChannelType::SingleChannel,
            _ => ChannelType::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Copyright {
    None,
    Some,
}

impl Default for Copyright {
    fn default() -> Copyright {
        Copyright::Some
    }
}

impl From<u8> for Copyright {
    fn from(c: u8) -> Copyright {
        match c {
            0x00 => Copyright::None,
            /*0x08*/ _ => Copyright::Some,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Copy,
    Original,
    Unknown,
}

impl Default for Status {
    fn default() -> Status {
        Status::Unknown
    }
}

impl From<u8> for Status {
    fn from(c: u8) -> Status {
        match c {
            0x00 => Status::Copy,
            0x04 => Status::Original,
            _ => Status::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn default() -> Emphasis {
        Emphasis::Unknown
    }
}

impl From<u8> for Emphasis {
    fn from(c: u8) -> Emphasis {
        match c {
            0 => Emphasis::None,
            1 => Emphasis::MicroSeconds,
            2 => Emphasis::Reserved,
            3 => Emphasis::CCITT,
            _ => Emphasis::Unknown,
        }
    }
}

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
    Unknown
}

impl From<u8> for Genre {
    fn from(c: u8) -> Genre {
        match c {
            0 => Genre::Blues,
            1 => Genre::ClassicRock,
            2 => Genre::Country,
            3 => Genre::Dance,
            4 => Genre::Disco,
            5 => Genre::Funk,
            6 => Genre::Grunge,
            7 => Genre::HipHop,
            8 => Genre::Jazz,
            9 => Genre::Metal,
            10 => Genre::NewAge,
            11 => Genre::Oldies,
            12 => Genre::Other,
            13 => Genre::Pop,
            14 => Genre::RAndB,
            15 => Genre::Rap,
            16 => Genre::Reggae,
            17 => Genre::Rock,
            18 => Genre::Techno,
            19 => Genre::Industrial,
            20 => Genre::Alternative,
            21 => Genre::Ska,
            22 => Genre::DeathMetal,
            23 => Genre::Pranks,
            24 => Genre::Soundtrack,
            25 => Genre::EuroTechno,
            26 => Genre::Ambient,
            27 => Genre::TripHop,
            28 => Genre::Vocal,
            29 => Genre::JazzFunk,
            30 => Genre::Fusion,
            31 => Genre::Trance,
            32 => Genre::Classical,
            33 => Genre::Instrumental,
            34 => Genre::Acid,
            35 => Genre::House,
            36 => Genre::Game,
            37 => Genre::SoundClip,
            38 => Genre::Gospel,
            39 => Genre::Noise,
            40 => Genre::AlternRock,
            41 => Genre::Bass,
            42 => Genre::Soul,
            43 => Genre::Punk,
            44 => Genre::Space,
            45 => Genre::Meditative,
            46 => Genre::InstrumentalPop,
            47 => Genre::InstrumentalRock,
            48 => Genre::Ethnic,
            49 => Genre::Gothic,
            50 => Genre::Darkwave,
            51 => Genre::TechnoIndustrial,
            52 => Genre::Electronic,
            53 => Genre::PopFolk,
            54 => Genre::Eurodance,
            55 => Genre::Dream,
            56 => Genre::SouthernRock,
            57 => Genre::Comedy,
            58 => Genre::Cult,
            59 => Genre::Gangsta,
            60 => Genre::Top40,
            61 => Genre::ChristianRap,
            62 => Genre::PopFunk,
            63 => Genre::Jungle,
            64 => Genre::NativeAmerican,
            65 => Genre::Cabaret,
            66 => Genre::NewWave,
            67 => Genre::Psychadelic,
            68 => Genre::Rave,
            69 => Genre::Showtunes,
            70 => Genre::Trailer,
            71 => Genre::LoFi,
            72 => Genre::Tribal,
            73 => Genre::AcidPunk,
            74 => Genre::AcidJazz,
            75 => Genre::Polka,
            76 => Genre::Retro,
            77 => Genre::Musical,
            78 => Genre::RockAndRoll,
            79 => Genre::HardRock,
            80 => Genre::Folk,
            81 => Genre::FolkRock,
            82 => Genre::NationalFolk,
            83 => Genre::Swing,
            84 => Genre::FastFusion,
            85 => Genre::Bebob,
            86 => Genre::Latin,
            87 => Genre::Revival,
            88 => Genre::Celtic,
            89 => Genre::Bluegrass,
            90 => Genre::Avantgarde,
            91 => Genre::GothicRock,
            92 => Genre::ProgressiveRock,
            93 => Genre::PsychedelicRock,
            94 => Genre::SymphonicRock,
            95 => Genre::SlowRock,
            96 => Genre::BigBand,
            97 => Genre::Chorus,
            98 => Genre::EasyListening,
            99 => Genre::Acoustic,
            100 => Genre::Humour,
            101 => Genre::Speech,
            102 => Genre::Chanson,
            103 => Genre::Opera,
            104 => Genre::ChamberMusic,
            105 => Genre::Sonata,
            106 => Genre::Symphony,
            107 => Genre::BootyBrass,
            108 => Genre::Primus,
            109 => Genre::PornGroove,
            110 => Genre::Satire,
            111 => Genre::SlowJam,
            112 => Genre::Club,
            113 => Genre::Tango,
            114 => Genre::Samba,
            115 => Genre::Folklore,
            116 => Genre::Ballad,
            117 => Genre::PowerBallad,
            118 => Genre::RhytmicSoul,
            119 => Genre::Freestyle,
            120 => Genre::Duet,
            121 => Genre::PunkRock,
            122 => Genre::DrumSolo,
            123 => Genre::ACapela,
            124 => Genre::EuroHouse,
            125 => Genre::DanceHall,
            _ => Genre::Unknown,
        }
    }
}
