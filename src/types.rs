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
    pub position: Duration,
}

#[derive(Debug, PartialEq)]
pub struct MP3Metadata {
    pub duration: Duration,
    pub frames: Vec<Frame>,
    pub tag: Option<AudioTag>,
    pub optional_info: Vec<OptionalAudioTags>,
}

#[derive(Debug, Default, PartialEq)]
pub struct AudioTag {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: u16,
    pub comment: String,
    pub genre: Genre,
}

#[derive(Debug, Default, PartialEq)]
pub struct Url(pub String);

// TODO: Add picture support
/// id3.org/id3v2.3.0#Declared_ID3v2_frames#Text_information_frames_-_details
#[derive(Debug, Default, PartialEq)]
pub struct OptionalAudioTags {
    /// Corresponds to the nth frames `MP3Metadata.frame`.
    pub position: u32,
    pub major_version: u8,
    pub minor_version: u8,
    /// The 'Album/Movie/Show title' frame is intended for the title of the
    /// recording(/source of sound) which the audio in the file is taken from.
    pub album_movie_show: Option<String>,
    /// The 'BPM' frame contains the number of beats per minute in the mainpart
    /// of the audio. The BPM is an integer and represented as a numerical string.
    pub bpm: Option<String>,
    /// The 'Composer(s)' frame is intended for the name of the composer(s).
    pub composers: Vec<String>,
    /// The 'Content type', which previously was stored as a one byte numeric value
    /// only, is now a numeric string. You may use one or several of the types as
    /// ID3v1.1 did or, since the category list would be impossible to maintain with
    /// accurate and up to date categories, define your own.
    ///
    /// References to the ID3v1 genres can be made by, as first byte, enter "("
    /// followed by a number from the genres list (appendix A) and ended with a ")"
    /// character. This is optionally followed by a refinement, e.g. "(21)" or
    /// "(4)Eurodisco". Several references can be made in the same frame, e.g.
    /// "(51)(39)". If the refinement should begin with a "(" character it should be
    /// replaced with "((", e.g. "((I can figure out any genre)" or "(55)((I think...)".
    /// The following new content types is defined in ID3v2 and is implemented in the
    /// same way as the numerig content types, e.g. "(RX)".
    pub content_type: Vec<Genre>,
    /// The 'Copyright message' frame, which must begin with a year and a space character
    /// (making five characters), is intended for the copyright holder of the original
    /// sound, not the audio file itself. The absence of this frame means only that the
    /// copyright information is unavailable or has been removed, and must not be
    /// interpreted to mean that the sound is public domain. Every time this field is
    /// displayed the field must be preceded with "Copyright © ".
    pub copyright: Option<String>,
    /// The 'Date' frame is a numeric string in the DDMM format containing the date for
    /// the recording. This field is always four characters long.
    pub date: Option<String>,
    /// The 'Playlist delay' defines the numbers of milliseconds of silence between
    /// every song in a playlist. The player should use the "ETC" frame, if present,
    /// to skip initial silence and silence at the end of the audio to match the
    /// 'Playlist delay' time. The time is represented as a numeric string.
    pub playlist_delay: Option<String>,
    /// The 'Encoded by' frame contains the name of the person or organisation that
    /// encoded the audio file. This field may contain a copyright message, if the
    /// audio file also is copyrighted by the encoder.
    pub encoded_by: Option<String>,
    /// The 'Lyricist(s)/Text writer(s)' frame is intended for the writer(s) of the text
    /// or lyrics in the recording.
    pub text_writers: Vec<String>,
    ///     The 'File type' frame indicates which type of audio this tag defines. The
    /// following type and refinements are defined:
    ///
    /// * MPG       MPEG Audio
    /// * /1        MPEG 1/2 layer I
    /// * /2        MPEG 1/2 layer II
    /// * /3        MPEG 1/2 layer III
    /// * /2.5      MPEG 2.5
    /// *  /AAC     Advanced audio compression
    /// * VQF       Transform-domain Weighted Interleave Vector Quantization
    /// * PCM       Pulse Code Modulated audio
    ///
    /// but other types may be used, not for these types though. This is used in a
    /// similar way to the predefined types in the "TMED" frame, but without
    /// parentheses. If this frame is not present audio type is assumed to be "MPG".
    pub file_type: Option<String>,
    /// The 'Time' frame is a numeric string in the HHMM format containing the time
    /// for the recording. This field is always four characters long.
    pub time: Option<String>,
    /// The 'Content group description' frame is used if the sound belongs to a larger
    /// category of sounds/music. For example, classical music is often sorted in
    /// different musical sections (e.g. "Piano Concerto", "Weather - Hurricane").
    pub content_group_description: Option<String>,
    /// The 'Subtitle/Description refinement' frame is used for information directly
    /// related to the contents title (e.g. "Op. 16" or "Performed live at Wembley").
    pub subtitle_refinement_description: Option<String>,
    /// The 'Title/Songname/Content description' frame is the actual name of the
    /// piece (e.g. "Adagio", "Hurricane Donna").
    pub title: Option<String>,
    /// The 'Initial key' frame contains the musical key in which the sound starts. It is
    /// represented as a string with a maximum length of three characters. The ground
    /// keys are represented with "A","B","C","D","E", "F" and "G" and halfkeys
    /// represented with "b" and "#". Minor is represented as "m". Example "Cbm". Off
    /// key is represented with an "o" only.
    pub initial_key: Option<String>,
    /// The 'Language(s)' frame should contain the languages of the text or lyrics spoken
    /// or sung in the audio. The language is represented with three characters according
    /// to ISO-639-2. If more than one language is used in the text their language codes
    /// should follow according to their usage.
    pub language: Option<String>,
    /// The 'Length' frame contains the length of the audiofile in milliseconds,
    /// represented as a numeric string.
    pub length: Option<String>,
    /// The 'Media type' frame describes from which media the sound originated. This may
    /// be a text string or a reference to the predefined media types found in the list
    /// below. References are made within "(" and ")" and are optionally followed by a
    /// text refinement, e.g. "(MC) with four channels". If a text refinement should
    /// begin with a "(" character it should be replaced with "((" in the same way as in
    /// the "TCO" frame. Predefined refinements is appended after the media type, e.g.
    /// "(CD/A)" or "(VID/PAL/VHS)".
    ///
    /// DIG     Other digital media
    ///     /A  Analog transfer from media
    /// ///
    /// ANA     Other analog media
    ///    /WAC Wax cylinder
    ///    /8CA 8-track tape cassette
    ///
    /// CD      CD
    ///      /A Analog transfer from media
    ///     /DD DDD
    ///     /AD ADD
    ///     /AA AAD
    ///
    /// LD      Laserdisc
    ///      /A Analog transfer from media
    ///
    /// TT      Turntable records
    ///     /33 33.33 rpm
    ///     /45 45 rpm
    ///     /71 71.29 rpm
    ///     /76 76.59 rpm
    ///     /78 78.26 rpm
    ///     /80 80 rpm
    ///
    /// MD      MiniDisc
    ///      /A Analog transfer from media
    ///
    /// DAT     DAT
    ///      /A Analog transfer from media
    ///      /1 standard, 48 kHz/16 bits, linear
    ///      /2 mode 2, 32 kHz/16 bits, linear
    ///      /3 mode 3, 32 kHz/12 bits, nonlinear, low speed
    ///      /4 mode 4, 32 kHz/12 bits, 4 channels
    ///      /5 mode 5, 44.1 kHz/16 bits, linear
    ///      /6 mode 6, 44.1 kHz/16 bits, 'wide track' play
    ///
    /// DCC     DCC
    ///      /A Analog transfer from media
    ///
    /// DVD     DVD
    ///      /A Analog transfer from media
    ///
    /// TV      Television
    ///    /PAL PAL
    ///   /NTSC NTSC
    ///  /SECAM SECAM
    ///
    /// VID     Video
    ///    /PAL PAL
    ///   /NTSC NTSC
    ///  /SECAM SECAM
    ///    /VHS VHS
    ///   /SVHS S-VHS
    ///   /BETA BETAMAX
    ///
    /// RAD     Radio
    ///     /FM FM
    ///     /AM AM
    ///     /LW LW
    ///     /MW MW
    ///
    /// TEL     Telephone
    ///      /I ISDN
    ///
    /// MC      MC (normal cassette)
    ///      /4 4.75 cm/s (normal speed for a two sided cassette)
    ///      /9 9.5 cm/s
    ///      /I Type I cassette (ferric/normal)
    ///     /II Type II cassette (chrome)
    ///    /III Type III cassette (ferric chrome)
    ///     /IV Type IV cassette (metal)
    ///
    /// REE     Reel
    ///      /9 9.5 cm/s
    ///     /19 19 cm/s
    ///     /38 38 cm/s
    ///     /76 76 cm/s
    ///      /I Type I cassette (ferric/normal)
    ///     /II Type II cassette (chrome)
    ///    /III Type III cassette (ferric chrome)
    ///     /IV Type IV cassette (metal)
    pub media_type: Option<String>,
    /// The 'Original album/movie/show title' frame is intended for the title of the
    /// original recording (or source of sound), if for example the music in the file
    /// should be a cover of a previously released song.
    pub original_album_move_show_title: Option<String>,
    /// The 'Original filename' frame contains the preferred filename for the file,
    /// since some media doesn't allow the desired length of the filename. The
    /// filename is case sensitive and includes its suffix.
    pub original_filename: Option<String>,
    /// The 'Original lyricist(s)/text writer(s)' frame is intended for the text
    /// writer(s) of the original recording, if for example the music in the file should
    /// be a cover of a previously released song.
    pub original_text_writers: Vec<String>,
    /// The 'Original artist(s)/performer(s)' frame is intended for the performer(s) of
    /// the original recording, if for example the music in the file should be a cover
    /// of a previously released song.
    pub original_artists: Vec<String>,
    /// The 'Original release year' frame is intended for the year when the original
    /// recording, if for example the music in the file should be a cover of a
    /// previously released song, was released. The field is formatted as in the
    /// `year` field.
    pub original_release_year: Option<String>,
    /// The 'File owner/licensee' frame contains the name of the owner or licensee
    /// of the file and it's contents.
    pub file_owner: Option<String>,
    /// The 'Lead artist(s)/Lead performer(s)/Soloist(s)/Performing group' is used
    /// for the main artist(s).
    pub performers: Vec<String>,
    /// The 'Band/Orchestra/Accompaniment' frame is used for additional information
    /// about the performers in the recording.
    pub band: Option<String>,
    /// The 'Conductor' frame is used for the name of the conductor.
    pub conductor: Option<String>,
    /// The 'Interpreted, remixed, or otherwise modified by' frame contains more
    /// information about the people behind a remix and similar interpretations of
    /// another existing piece.
    pub interpreted: Option<String>,
    /// The 'Part of a set' frame is a numeric string that describes which part of a
    /// set the audio came from. This frame is used if the source described in the
    /// "TALB" frame is divided into several mediums, e.g. a double CD. The value may
    /// be extended with a "/" character and a numeric string containing the total
    /// number of parts in the set. E.g. "1/2".
    pub part_of_a_set: Option<String>,
    /// The 'Publisher' frame simply contains the name of the label or publisher.
    pub publisher: Option<String>,
    /// The 'Track number/Position in set' frame is a numeric string containing the
    /// order number of the audio-file on its original recording. This may be extended
    /// with a "/" character and a numeric string containing the total numer of
    /// tracks/elements on the original recording. E.g. "4/9".
    pub track_number: Option<String>,
    /// The 'Recording dates' frame is a intended to be used as complement to the
    /// "TYER", "TDAT" and "TIME" frames. E.g. "4th-7th June, 12th June" in
    /// combination with the "TYER" frame.
    pub recording_dates: Option<String>,
    /// The 'Internet radio station name' frame contains the name of the internet
    /// radio station from which the audio is streamed.
    pub internet_radio_station_name: Option<String>,
    /// The 'Internet radio station owner' frame contains the name of the owner of
    /// the internet radio station from which the audio is streamed.
    pub internet_radio_station_owner: Option<String>,
    /// The 'Size' frame contains the size of the audiofile in bytes, excluding the
    /// ID3v2 tag, represented as a numeric string.
    pub size: Option<String>,
    /// The 'ISRC' frame should contain the International Standard Recording Code
    /// (ISRC) (12 characters).
    pub international_standard_recording_code: Option<String>,
    /// The 'Software/Hardware and settings used for encoding' frame includes the
    /// used audio encoder and its settings when the file was encoded. Hardware
    /// refers to hardware encoders, not the computer on which a program was run.
    pub soft_hard_setting: Option<String>,
    /// The 'Year' frame is a numeric string with a year of the recording. This
    /// frames is always four characters long (until the year 10000).
    pub year: Option<String>,
    /// Since there might be a lot of people contributing to an audio file in
    /// various ways, such as musicians and technicians, the 'Text information
    /// frames' are often insufficient to list everyone involved in a project.
    /// The 'Involved people list' is a frame containing the names of those
    /// involved, and how they were involved. The body simply contains a terminated
    /// string with the involvement directly followed by a terminated string with
    /// the involvee followed by a new involvement and so on.
    pub involved_people: Option<String>,

    /// The 'Commercial information' frame is a URL pointing at a webpage with
    /// information such as where the album can be bought. There may be more than
    /// one "WCOM" frame in a tag, but not with the same content.
    pub commercial_info_url: Vec<Url>,
    /// The 'Copyright/Legal information' frame is a URL pointing at a webpage
    /// where the terms of use and ownership of the file is described.
    pub copyright_info_url: Option<Url>,
    /// The 'Official audio file webpage' frame is a URL pointing at a file specific
    /// webpage.
    pub official_webpage: Option<Url>,
    /// The 'Official artist/performer webpage' frame is a URL pointing at the
    /// artists official webpage. There may be more than one "WOAR" frame in a tag
    /// if the audio contains more than one performer, but not with the same content.
    pub official_artist_webpage: Vec<Url>,
    /// The 'Official audio source webpage' frame is a URL pointing at the official
    /// webpage for the source of the audio file, e.g. a movie.
    pub official_audio_source_webpage: Option<Url>,
    /// The 'Official internet radio station homepage' contains a URL pointing at the
    /// homepage of the internet radio station.
    pub official_internet_radio_webpage: Option<Url>,
    /// The 'Payment' frame is a URL pointing at a webpage that will handle the
    /// process of paying for this file.
    pub payment_url: Option<Url>,
    /// The 'Publishers official webpage' frame is a URL pointing at the official
    /// wepage for the publisher.
    pub publishers_official_webpage: Option<Url>,
}
