extern crate mp3_metadata;

#[test]
fn id3v2() {
    let meta = mp3_metadata::read_from_file("assets/id3v2.mp3").expect("File error");
    assert_eq!(meta.optional_info[0].position, 0);
    assert_eq!(meta.optional_info[0].major_version, 4);
    assert_eq!(meta.optional_info[0].minor_version, 0);
    assert_eq!(meta.optional_info[0].album_movie_show, Some("éàµ£ø§".to_owned()));
    assert_eq!(meta.optional_info[0].bpm, None);
    assert_eq!(meta.optional_info[0].composers,
               vec!("not Mozart".to_owned(), "not Beethoven".to_owned()));
    assert_eq!(meta.optional_info[0].content_type,
               vec!(mp3_metadata::Genre::InstrumentalPop));
    assert_eq!(meta.optional_info[0].copyright, Some("Is there?".to_owned()));
    assert_eq!(meta.optional_info[0].date, None);
    assert_eq!(meta.optional_info[0].playlist_delay, None);
    assert_eq!(meta.optional_info[0].encoded_by, Some("some website...".to_owned()));
    assert_eq!(meta.optional_info[0].text_writers.len(), 0);
    assert_eq!(meta.optional_info[0].file_type, None);
    assert_eq!(meta.optional_info[0].time, None);
    assert_eq!(meta.optional_info[0].content_group_description, None);
    assert_eq!(meta.optional_info[0].subtitle_refinement_description, None);
    assert_eq!(meta.optional_info[0].title,
               Some("This is a wonderful title isn't it?".to_owned()));
    assert_eq!(meta.optional_info[0].performers,
               vec!("Someone".to_owned(), "Someone else".to_owned()));
    assert_eq!(meta.optional_info[0].band,
               Some("I like artists! But who to choose? So many of them...".to_owned()));
    assert_eq!(meta.optional_info[0].track_number, Some("01".to_owned()));

    assert_eq!(meta.tag, Some(mp3_metadata::AudioTag {
        title: "This is a wonderful title isn'".to_owned(),
        artist: "Someone/Someone else          ".to_owned(),
        album: "".to_owned(),
        year: 2015,
        comment: "Some random comment because ".to_owned(),
        genre: mp3_metadata::Genre::Other,
    }));
}
