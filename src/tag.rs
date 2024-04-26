use std::io::Cursor;

use lofty::{
    config::WriteOptions,
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::{Accessor, ItemKey, Tag, TagExt},
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    picture::{from_lofty_picture_vec, to_lofty_picture, MetaPicture},
    utils::set_panic_hook,
};

#[wasm_bindgen]
pub struct MetaFile {
    buffer: Vec<u8>,
    tag: Tag,
    props: <TaggedFile as AudioFile>::Properties,
}

#[wasm_bindgen]
impl MetaFile {
    #[wasm_bindgen(constructor)]
    pub fn new(buf: Vec<u8>) -> Self {
        set_panic_hook();
        let tagged_file = Probe::new(Cursor::new(buf.clone()))
            .guess_file_type()
            .expect("fail to guess file type")
            .read()
            .expect("fail to read file");

        MetaFile {
            buffer: buf,
            tag: tagged_file.primary_tag().cloned().unwrap_or_else(|| {
                tagged_file
                    .first_tag()
                    .cloned()
                    .expect("ERROR: No tags found!")
            }),
            props: tagged_file.properties().clone(),
        }
    }

    #[wasm_bindgen]
    pub fn save(&mut self) {
        set_panic_hook();
        let mut buf = Cursor::new(self.buffer.clone());
        let _ = self.tag.save_to(&mut buf, WriteOptions::default());
        self.buffer = buf.into_inner();
    }

    /// File buffer
    #[wasm_bindgen(getter = buffer)]
    pub fn buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    /// The title of the song
    #[wasm_bindgen(getter = title)]
    pub fn get_title(&self) -> Option<String> {
        self.tag.title().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = title)]
    pub fn set_title(&mut self, title: String) {
        self.tag.set_title(title);
    }

    /// The artist of the song
    #[wasm_bindgen(getter = artist)]
    pub fn get_artist(&self) -> Option<String> {
        self.tag.artist().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = artist)]
    pub fn set_artist(&mut self, artist: String) {
        self.tag.set_artist(artist);
    }

    /// The album of the song
    #[wasm_bindgen(getter = album)]
    pub fn get_album(&self) -> Option<String> {
        self.tag.album().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = album)]
    pub fn set_album(&mut self, album: String) {
        self.tag.set_album(album);
    }

    /// The genre of the song
    #[wasm_bindgen(getter = genre)]
    pub fn get_genre(&self) -> Option<String> {
        self.tag.genre().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = genre)]
    pub fn set_genre(&mut self, genre: String) {
        self.tag.set_genre(genre);
    }

    /// The year the song was released
    #[wasm_bindgen(getter = year)]
    pub fn get_year(&self) -> Option<u32> {
        self.tag.year()
    }

    #[wasm_bindgen(setter = year)]
    pub fn set_year(&mut self, year: u32) {
        self.tag.set_year(year);
    }

    /// The disc number
    #[wasm_bindgen(getter = disc)]
    pub fn get_disc(&self) -> Option<u32> {
        self.tag.disk()
    }

    #[wasm_bindgen(setter = disc)]
    pub fn set_disc(&mut self, disc: u32) {
        self.tag.set_disk(disc);
    }

    /// The total number of discs
    #[wasm_bindgen(getter = discTotal)]
    pub fn get_disc_total(&self) -> Option<u32> {
        self.tag.disk_total()
    }

    #[wasm_bindgen(setter = discTotal)]
    pub fn set_disc_total(&mut self, disc_total: u32) {
        self.tag.set_disk_total(disc_total);
    }

    /// The track number
    #[wasm_bindgen(getter = track)]
    pub fn get_track(&self) -> Option<u32> {
        self.tag.track()
    }

    #[wasm_bindgen(setter = track)]
    pub fn set_track(&mut self, track: u32) {
        self.tag.set_track(track);
    }

    /// The total number of tracks
    #[wasm_bindgen(getter = trackTotal)]
    pub fn get_track_total(&self) -> Option<u32> {
        self.tag.track_total()
    }

    #[wasm_bindgen(setter = trackTotal)]
    pub fn set_track_total(&mut self, track_total: u32) {
        self.tag.set_track_total(track_total);
    }

    /// The album artist of the entire album
    #[wasm_bindgen(getter = albumArtist)]
    pub fn get_album_artist(&self) -> Option<String> {
        self.tag
            .get_string(&ItemKey::AlbumArtist)
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = albumArtist)]
    pub fn set_album_artist(&mut self, album_artist: String) {
        self.tag.insert_text(ItemKey::AlbumArtist, album_artist);
    }

    /// The composer of the song
    #[wasm_bindgen(getter = composer)]
    pub fn get_composer(&self) -> Option<String> {
        self.tag
            .get_string(&ItemKey::Composer)
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = composer)]
    pub fn set_composer(&mut self, composer: String) {
        self.tag.insert_text(ItemKey::Composer, composer);
    }

    /// The lyricist of the song
    #[wasm_bindgen(getter = lyricist)]
    pub fn get_lyricist(&self) -> Option<String> {
        self.tag
            .get_string(&ItemKey::Lyricist)
            .map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = lyricist)]
    pub fn set_lyricist(&mut self, lyricist: String) {
        self.tag.insert_text(ItemKey::Lyricist, lyricist);
    }

    /// The comment of the song
    #[wasm_bindgen(getter = comment)]
    pub fn get_comment(&self) -> Option<String> {
        self.tag.comment().map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = comment)]
    pub fn set_comment(&mut self, comment: String) {
        self.tag.set_comment(comment);
    }

    /// The lyrics of the song
    #[wasm_bindgen(getter = lyrics)]
    pub fn get_lyrics(&self) -> Option<String> {
        self.tag.get_string(&ItemKey::Lyrics).map(|s| s.to_string())
    }

    #[wasm_bindgen(setter = lyrics)]
    pub fn set_lyrics(&mut self, lyrics: String) {
        self.tag.insert_text(ItemKey::Lyrics, lyrics);
    }

    /// The pictures of the song
    #[wasm_bindgen(getter = pictures)]
    pub fn get_pictures(&self) -> Option<Vec<MetaPicture>> {
        from_lofty_picture_vec(&Some(self.tag.pictures().to_vec()))
    }

    #[wasm_bindgen(setter = pictures)]
    pub fn set_pictures(&mut self, pictures: Vec<MetaPicture>) {
        let mut i = 0;
        for picture in pictures.iter() {
            if let Some(pic) = to_lofty_picture(picture) {
                self.tag.set_picture(i, pic);
                i += 1;
            }
        }
        if i < self.tag.pictures().len() {
            for j in i..self.tag.pictures().len() {
                self.tag.remove_picture(j);
            }
        }
    }

    /// Audio bit depth in bits
    #[wasm_bindgen(getter = bitDepth)]
    pub fn get_bit_depth(&self) -> Option<u8> {
        self.props.bit_depth()
    }

    /// Audio bit rate in kbps
    #[wasm_bindgen(getter = bitRate)]
    pub fn get_bit_rate(&self) -> u32 {
        self.props.audio_bitrate().unwrap_or(
            (self.buffer.len() - self.tag.len()) as u32 * 8
                / self.props.duration().as_millis() as u32,
        )
    }

    /// Audio sample rate in Hz
    #[wasm_bindgen(getter = sampleRate)]
    pub fn get_sample_rate(&self) -> Option<u32> {
        self.props.sample_rate()
    }

    /// number of channels
    #[wasm_bindgen(getter = channels)]
    pub fn get_channels(&self) -> Option<u8> {
        self.props.channels()
    }

    /// duration in milliseconds
    #[wasm_bindgen(getter = duration)]
    pub fn get_duration(&self) -> u32 {
        self.props.duration().as_millis() as u32
    }
}
