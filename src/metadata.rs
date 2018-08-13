use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use consts::{BITRATES, SAMPLING_FREQ};
use enums::{ChannelType, Copyright, CRC, Emphasis, Error, Genre, Layer, Status, Version};
use types::{AudioTag, Frame, MP3Metadata, OptionalAudioTags};
use utils::{compute_duration, create_utf8_str, get_line, get_samp_line, get_text_field, get_text_fields};
use utils::{get_url_field, get_url_fields};

fn get_id3(i: &mut u32, buf: &[u8], meta: &mut MP3Metadata) -> Result<(), Error> {
    let mut x = *i as usize;
    // Get extended information
    if buf.len() > 127 && x + 127 < buf.len() && // V1
       buf[x] == 'T' as u8 && buf[x + 1] == 'A' as u8 && buf[x + 2] == 'G' as u8 {
        if meta.tag.is_some() {
            return Err(Error::DuplicatedIDV3);
        }
        if let Some(last) = meta.frames.last_mut() {
            if *i <= last.size {
                return Ok(());
            }
            last.size = *i - last.size - 1;
        }
        *i += 126;
        // tag v1
        meta.tag = Some(AudioTag {
            title: create_utf8_str(&buf[x + 3..][..30]),
            artist: create_utf8_str(&buf[x + 33..][..30]),
            album: create_utf8_str(&buf[x + 63..][..30]),
            year: create_utf8_str(&buf[x + 93..][..4]).parse::<u16>().unwrap_or(0),
            comment: create_utf8_str(&buf[x + 97..][..if buf[x + 97 + 28] != 0 { 30 } else { 28 }]),
            genre: Genre::from(buf[x + 127]),
        });
        Ok(())
    } else if buf.len() > x + 13 && // V2 and above
              buf[x] == 'I' as u8 && buf[x + 1] == 'D' as u8 && buf[x + 2] == '3' as u8 {
        let maj_version = buf[x + 3];
        let min_version = buf[x + 4];

        if maj_version > 4 {
            return Ok(());
        }

        let tag_size = ((buf[x + 9] as usize) & 0xFF) |
                       (((buf[x + 8] as usize) & 0xFF) << 7) |
                       (((buf[x + 7] as usize) & 0xFF) << 14) |
                       (((buf[x + 6] as usize) & 0xFF) << 21) + 10;
        let use_sync = buf[x + 5] & 0x80 != 0;
        let has_extended_header = buf[x + 5] & 0x40 != 0;

        if has_extended_header {
            x += 10;
            if x + 4 >= buf.len() {
                *i = x as u32;
                return Ok(())
            }
            let header_size = ((buf[x] as u32) << 21) |
                              ((buf[x + 1] as u32) << 14) |
                              ((buf[x + 2] as u32) << 7) |
                              buf[x + 3] as u32;
            if header_size < 4 {
                return Ok(())
            }
            x += header_size as usize - 4;
        }

        *i = x as u32 + tag_size as u32;
        if x + tag_size >= buf.len() {
            return Ok(())
        }

        // Recreate the tag if desynchronization is used inside; we need to replace
        // 0xFF 0x00 with 0xFF
        let mut v = Vec::new();
        let (buf, length) = if use_sync {
            let mut new_pos = 0;
            let mut skip = false;
            v.reserve(tag_size);

            for i in 0..tag_size {
                if skip == true {
                    skip = false;
                    continue;
                }
                if i + 1 >= buf.len() {
                    return Ok(());
                }
                if i + 1 < tag_size && (buf[i] & 0xFF) == 0xFF && buf[i + 1] == 0 {
                    v[new_pos] = 0xFF;
                    new_pos += 1;
                    skip = true;
                    continue;
                }
                if new_pos >= v.len() {
                    return Ok(());
                }
                v[new_pos] = buf[i];
                new_pos += 1;
            }
            (v.as_slice(), new_pos)
        } else {
            (buf, tag_size)
        };

        let mut pos = x;
        let id3_frame_size = if maj_version < 3 { 6 } else { 10 };
        let mut op = OptionalAudioTags::default();
        let mut changes = false;
        loop {
            if pos + id3_frame_size > x + length {
                break;
            }

            // Check if there is there a frame.
            if buf[pos] < 'A' as u8 || buf[pos] > 'Z' as u8 {
                break;
            }

            // Frame name is 3 chars in pre-ID3v3 and 4 chars after
            let (frame_name, frame_size) = if maj_version < 3 {
                (create_utf8_str(&buf[pos..][..3]), (buf[pos + 5] as u32 & 0xFF) |
                                          ((buf[pos + 4] as u32 & 0xFF) << 8) |
                                          ((buf[pos + 3] as u32 & 0xFF) << 16))
            } else {
                (create_utf8_str(&buf[pos..][..4]), (buf[pos + 7] as u32 & 0xFF) |
                                          ((buf[pos + 6] as u32 & 0xFF) << 8) |
                                          ((buf[pos + 5] as u32 & 0xFF) << 16) |
                                          ((buf[pos + 4] as u32 & 0xFF) << 24))
            };

            pos += id3_frame_size;
            if pos + frame_size as usize > x + length {
                break;
            }

            // http://id3.org/id3v2.3.0#Declared_ID3v2_frames
            match &frame_name[..] {
                // -----------------------
                // ----- TEXT FRAMES -----
                // -----------------------
                "TALB" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.album_movie_show),
                "TBPM" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.bpm),
                "TCOM" => get_text_fields(buf, pos, frame_size, &mut changes,
                                          &mut op.composers),
                "TCON" => {
                    let mut s = None;
                    get_text_field(buf, pos, frame_size, &mut changes, &mut s);
                    if let Some(s) = s {
                        if s.len() > 0 {
                            if s.starts_with('(') && s.ends_with(')') {
                                let v = s.split(')').collect::<Vec<&str>>().into_iter().filter_map(|a| {
                                    match a.replace("(", "").parse::<u8>() {
                                        Ok(num) => Some(Genre::from(num)),
                                        _ => None,
                                    }
                                }).collect::<Vec<Genre>>();
                                if v.len() > 0 {
                                    for entry in v {
                                        op.content_type.push(entry);
                                    }
                                } else {
                                    op.content_type.push(Genre::from(s.as_str()));
                                }
                            } else {
                                op.content_type.push(Genre::from(s.as_str()));
                            }
                        }
                    }
                    
                }
                "TCOP" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.copyright),
                "TDAT" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.date),
                "TDLY" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.playlist_delay),
                "TENC" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.encoded_by),
                "TEXT" => get_text_fields(buf, pos, frame_size, &mut changes,
                                          &mut op.text_writers),
                "TFLT" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.file_type),
                "TIME" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.time),
                "TIT" | "TIT2" => get_text_field(buf, pos, frame_size,
                                                 &mut changes, &mut op.title),
                "TIT1" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.content_group_description),
                "TIT3" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.subtitle_refinement_description),
                "TKEY" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.initial_key),
                "TLAN" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.language),
                "TLEN" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.length),
                "TMED" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.media_type),
                "TOAL" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.original_album_move_show_title),
                "TOFN" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.original_filename),
                "TOLY" => get_text_fields(buf, pos, frame_size, &mut changes,
                                          &mut op.original_text_writers),
                "TOPE" => get_text_fields(buf, pos, frame_size, &mut changes,
                                          &mut op.original_artists),
                "TORY" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.original_release_year),
                "TOWN" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.file_owner),
                "TPE1" => get_text_fields(buf, pos, frame_size, &mut changes,
                                          &mut op.performers),
                "TPE2" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.band),
                "TPE3" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.conductor),
                "TPE4" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.interpreted),
                "TPOS" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.part_of_a_set),
                "TPUB" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.publisher),
                "TRCK" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.track_number),
                "TRDA" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.recording_dates),
                "TRSN" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.internet_radio_station_name),
                "TRSO" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.internet_radio_station_owner),
                "TSIZ" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.size),
                "TSRC" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.international_standard_recording_code),
                "TSSE" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.soft_hard_setting),
                "TYER" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.year),
                "IPLS" => get_text_field(buf, pos, frame_size, &mut changes,
                                         &mut op.involved_people),
                // ----------------------
                // ----- URL FRAMES -----
                // ----------------------
                "WCOM" => get_url_fields(buf, pos, frame_size, &mut changes,
                                         &mut op.commercial_info_url),
                "WCOP" => get_url_field(buf, pos, frame_size, &mut changes,
                                        &mut op.copyright_info_url),
                "WOAF" => get_url_field(buf, pos, frame_size, &mut changes,
                                        &mut op.official_webpage),
                "WOAR" => get_url_fields(buf, pos, frame_size, &mut changes,
                                         &mut op.official_artist_webpage),
                "WOAS" => get_url_field(buf, pos, frame_size, &mut changes,
                                        &mut op.official_audio_source_webpage),
                "WORS" => get_url_field(buf, pos, frame_size, &mut changes,
                                        &mut op.official_internet_radio_webpage),
                "WPAY" => get_url_field(buf, pos, frame_size, &mut changes,
                                        &mut op.payment_url),
                "WPUB" => get_url_field(buf, pos, frame_size, &mut changes,
                                        &mut op.publishers_official_webpage),
                _ => {
                    // TODO: handle other type of fields, like picture
                }
            };

            pos += frame_size as usize;
        }
        if changes == true {
            op.position = meta.frames.len() as u32;
            op.minor_version = min_version;
            op.major_version = maj_version;
            meta.optional_info.push(op);
        }
        Ok(())
    } else {
        Ok(())
    }
}

fn read_header(buf: &[u8], i: &mut u32, meta: &mut MP3Metadata) -> Result<bool, Error> {
    let header = (buf[*i as usize] as u32) << 24 | (buf[*i as usize + 1] as u32) << 16 |
                 (buf[*i as usize + 2] as u32) << 8 | buf[*i as usize + 3] as u32;
    if header & 0xffe00000 == 0xffe00000 && header & (3 << 17) != 0 &&
       header & (0xf << 12) != (0xf << 12) && header & (3 << 10) != (3 << 10) {
        let mut frame: Frame = Default::default();

        frame.version = Version::from((header >> 19) & 3);
        frame.layer = Layer::from((header >> 17) & 3);
        frame.crc = CRC::from((header >> 16) & 1);

        frame.bitrate = BITRATES[get_line(frame.version, frame.layer)]
                                [((header >> 12) & 0xF) as usize];
        frame.sampling_freq = SAMPLING_FREQ[get_samp_line(frame.version)]
                                           [((header >> 10) & 0x3) as usize];
        frame.padding = (header >> 9) & 1 == 1;
        frame.private_bit = (header >> 8) & 1 == 1;

        frame.chan_type = ChannelType::from((header >> 4) & 3);
        let (intensity, ms_stereo) = match (header >> 2) & 3 {
            0x1 => (true, false),
            0x2 => (false, true),
            0x3 => (true, true),
            /*0x00*/ _ => (false, false),
        };
        frame.intensity_stereo = intensity;
        frame.ms_stereo = ms_stereo;
        frame.copyright = Copyright::from((header >> 3) & 1);
        frame.status = Status::from((header >> 2) & 1);
        frame.emphasis = Emphasis::from(header & 0x03);
        frame.duration = compute_duration(frame.version,
                                          frame.layer,
                                          frame.sampling_freq);
        frame.position = meta.duration;

        if let Some(dur) = frame.duration {
            meta.duration += dur;
        }
        /*frame.size = if frame.layer == Layer::Layer1 && frame.sampling_freq > 0 {
            /*println!("{:4}: (12000 * {} / {} + {}) * 4 = {}", i, frame.bitrate as u64, frame.sampling_freq as u64,
                if frame.slot { 1 } else { 0 },
                    (12000 * frame.bitrate as u64 / frame.sampling_freq as u64 +
                if frame.slot { 1 } else { 0 }) * 4);*/

            (12000 * frame.bitrate as u64 / frame.sampling_freq as u64 +
                if frame.slot { 1 } else { 0 }) * 4
        } else if (frame.layer == Layer::Layer2 || frame.layer == Layer::Layer3) && frame.sampling_freq > 0 {
            /*println!("{:4}: 144000 * {} / {} + {} = {}", i, frame.bitrate as u64, frame.sampling_freq as u64,
                if frame.slot { 1 } else { 0 },
                    144000 * frame.bitrate as u64 / frame.sampling_freq as u64 +
                if frame.slot { 1 } else { 0 });*/

            144000 * frame.bitrate as u64 / frame.sampling_freq as u64 +
                if frame.slot { 1 } else { 0 }
        } else {
            continue 'a;
        } as u32;*/
        let samples_per_frame = match frame.layer {
            Layer::Layer3 => {
                if frame.version == Version::MPEG1 {
                    1152
                } else {
                    576
                }
            }
            Layer::Layer2 => 1152,
            Layer::Layer1 => 384,
            _ => unreachable!(),
        };
        frame.size = (samples_per_frame as u64 / 8 * frame.bitrate as u64 * 1000 /
                      frame.sampling_freq as u64) as u32;
        if frame.size < 1 {
            return Ok(false);
        }
        if frame.padding {
            frame.size += 1;
        }
        *i += frame.size;
        meta.frames.push(frame);
        Ok(true)
    } else {
        Ok(false)
    }
}

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
        optional_info: Vec::new(),
    };
    let mut i = 0u32;

    'a: while i < buf.len() as u32 {
        loop {
            if let Err(e) = get_id3(&mut i, buf, &mut meta) {
                return Err(e);
            }
            if i + 3 >= buf.len() as u32 {
                break 'a;
            }
            match read_header(buf, &mut i, &mut meta) {
                Ok(true) => continue 'a,
                Err(e) => return Err(e),
                _ => {}
            }
            let old_i = i;
            if let Err(e) = get_id3(&mut i, buf, &mut meta) {
                return Err(e);
            }
            if i == old_i {
                i += 1;
            }
            if i >= buf.len() as u32 {
                break 'a;
            }
        }
    }
    if meta.tag.is_none() {
        if let Some(last) = meta.frames.last_mut() {
            if i <= last.size {
                return Err(Error::InvalidData);
            }
        }
    }
    if meta.frames.len() < 1 {
        Err(Error::NotMP3)
    } else {
        Ok(meta)
    }
}

#[test]
fn not_mp3() {
    let ret = read_from_file("src/lib.rs");

    match ret {
        Ok(_) => panic!("Wasn't supposed to be ok!"),
        Err(e) => assert_eq!(e, Error::NotMP3),
    }
}

#[test]
fn double_id() {
    let ret = read_from_file("assets/double_id.mp3");

    match ret {
        Ok(_) => panic!("Wasn't supposed to be ok!"),
        Err(e) => assert_eq!(e, Error::DuplicatedIDV3),
    }
}
