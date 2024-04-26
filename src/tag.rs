use std::io::Cursor;

use lofty::{
    config::WriteOptions,
    file::{AudioFile, TaggedFileExt},
    picture::MimeType,
    probe::Probe,
    tag::{Accessor, ItemKey, TagExt},
};
use serde::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::set_panic_hook;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct MetaPicture {
    /// The picture type according to ID3v2 APIC
    #[wasm_bindgen(js_name = "picType")]
    pub pic_type: String,

    /// The picture's mimetype
    #[wasm_bindgen(js_name = "mimeType")]
    pub mime_type: String,

    /// The picture's description
    pub description: String,

    /// The binary data of the picture
    pub data: Vec<u8>,
}

fn mimetype_enum_to_string(mime_type: Option<&MimeType>) -> String {
    match mime_type {
        Some(MimeType::Png) => "image/png".to_string(),
        Some(MimeType::Jpeg) => "image/jpeg".to_string(),
        Some(MimeType::Bmp) => "image/bmp".to_string(),
        Some(MimeType::Tiff) => "image/tiff".to_string(),
        Some(MimeType::Gif) => "image/gif".to_string(),
        Some(MimeType::Unknown(type_str)) => {
            let unknown_type = format!("image/{}", type_str);
            unknown_type
        }
        _ => "image/unknown".to_string(),
    }
}

fn string_to_mimetype_enum(mime_type: &str) -> Option<MimeType> {
    match mime_type {
        "image/png" => Some(MimeType::Png),
        "image/jpeg" => Some(MimeType::Jpeg),
        "image/bmp" => Some(MimeType::Bmp),
        "image/tiff" => Some(MimeType::Tiff),
        "image/gif" => Some(MimeType::Gif),
        _ => Some(MimeType::Unknown(mime_type.to_string())),
    }
}

fn from_lofty_picture_vec(
    pictures: &Option<Vec<lofty::picture::Picture>>,
) -> Option<Vec<MetaPicture>> {
    pictures.as_ref().map(|pics| {
        pics.iter()
            .map(|p| MetaPicture {
                pic_type: p.pic_type().as_ape_key().unwrap_or("").to_string(),
                mime_type: mimetype_enum_to_string(p.mime_type()),
                description: p.description().unwrap_or("").to_string(),
                data: p.data().to_vec(),
            })
            .collect()
    })
}

fn to_lofty_picture(pic: &MetaPicture) -> Option<lofty::picture::Picture> {
    if pic.data.is_empty() {
        return None;
    }
    Some(lofty::picture::Picture::new_unchecked(
        lofty::picture::PictureType::CoverFront,
        string_to_mimetype_enum(&pic.mime_type),
        Some(pic.description.clone()),
        pic.data.clone(),
    ))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Metadata {
    /// The title of the song
    pub title: Option<String>,

    /// The artist of the song
    pub artist: Option<String>,

    /// The album of the song
    pub album: Option<String>,

    /// The year the song was released
    pub year: Option<u32>,

    /// The disc number of the song
    pub disk: Option<u32>,

    /// The total number of discs
    #[wasm_bindgen(js_name = "diskTotal")]
    pub disk_total: Option<u32>,

    /// The track number of the song
    pub track: Option<u32>,

    /// The total number of tracks
    #[wasm_bindgen(js_name = "trackTotal")]
    pub track_total: Option<u32>,

    /// The genre of the song
    pub genre: Option<String>,

    /// The album artist of the song
    #[wasm_bindgen(js_name = "albumArtist")]
    pub album_artist: Option<String>,

    /// The composer of the song
    pub composer: Option<String>,

    /// The performer of the song
    pub lyricist: Option<String>,

    /// The comment of the song
    pub comment: Option<String>,

    /// The track lyrics
    pub lyrics: Option<String>,

    /// The track covers
    pub pictures: Option<Vec<MetaPicture>>,

    /// The bit depth of the song
    #[wasm_bindgen(js_name = "bitDepth")]
    pub bit_depth: Option<u8>,

    /// The number of channels in the song
    pub channels: u8,

    /// The bit rate of the song
    #[wasm_bindgen(js_name = "bitRate")]
    pub bit_rate: u32,

    /// The duration of the song, in milliseconds
    pub duration: u32,

    /// The sample rate of the song
    #[wasm_bindgen(js_name = "sampleRate")]
    pub sample_rate: Option<u32>,

    pub buffer: Vec<u8>,
}

#[wasm_bindgen]
impl Metadata {
    #[wasm_bindgen(constructor)]
    pub fn new(buf: Vec<u8>) -> Self {
        set_panic_hook();
        let buf_size = buf.len();

        let tagged_file = Probe::new(Cursor::new(buf.clone()))
            .guess_file_type()
            .expect("fail to guess file type")
            .read()
            .expect("fail to read file");

        let tag = tagged_file
            .primary_tag()
            .unwrap_or_else(|| tagged_file.first_tag().expect("ERROR: No tags found!"));

        Metadata {
            title: tag.title().map(|s| s.to_string()),
            album: tag.album().map(|s| s.to_string()),
            artist: tag.artist().map(|s| s.to_string()),

            genre: tag.genre().map(|s| s.to_string()),
            year: tag.year(),
            disk: tag.disk(),
            disk_total: tag.disk_total(),
            track: tag.track(),
            track_total: tag.track_total(),

            album_artist: tag.get_string(&ItemKey::AlbumArtist).map(|s| s.to_string()),
            composer: tag.get_string(&ItemKey::Composer).map(|s| s.to_string()),
            lyricist: tag.get_string(&ItemKey::Lyricist).map(|s| s.to_string()),

            comment: tag.comment().map(|s| s.to_string()),
            lyrics: tag.get_string(&ItemKey::Lyrics).map(|s| s.to_string()),
            pictures: from_lofty_picture_vec(&Some(tag.pictures().to_vec())),

            bit_depth: tagged_file.properties().bit_depth(),
            channels: tagged_file.properties().channels().unwrap_or(0),
            bit_rate: tagged_file.properties()
                .audio_bitrate()
                .unwrap_or((buf_size - tag.len()) as u32 * 8 / tagged_file.properties().duration().as_millis() as u32),
            duration: tagged_file.properties().duration().as_millis() as u32,
            sample_rate: tagged_file.properties().sample_rate(),
            buffer: buf,
        }
    }

    #[wasm_bindgen]
    pub fn save(&self) -> Metadata {
        set_panic_hook();
        let mut reader = Cursor::new(self.buffer.clone());

        let mut tagged_file = Probe::new(&mut reader)
            .guess_file_type()
            .expect("fail to guess file type")
            .read()
            .expect("fail to open buffer");

        let tag = tagged_file.primary_tag_mut().expect("No primary tag found");

        if let Some(title) = &self.title {
            tag.set_title(title.to_string());
        } else {
            tag.remove_title();
        }

        if let Some(artist) = &self.artist {
            tag.set_artist(artist.to_string());
        } else {
            tag.remove_artist();
        }

        if let Some(album) = &self.album {
            tag.set_album(album.to_string());
        } else {
            tag.remove_album();
        }

        if let Some(year) = &self.year {
            tag.set_year(*year);
        } else {
            tag.remove_year();
        }

        if let Some(genre) = &self.genre {
            tag.set_genre(genre.to_string());
        } else {
            tag.remove_genre();
        }

        if let Some(disk) = &self.disk {
            tag.set_disk(*disk);
        } else {
            tag.remove_disk();
        }

        if let Some(disk_total) = &self.disk_total {
            tag.set_disk_total(*disk_total);
        } else {
            tag.remove_disk_total();
        }

        if let Some(track) = &self.track {
            tag.set_track(*track);
        } else {
            tag.remove_track();
        }

        if let Some(track_total) = &self.track_total {
            tag.set_track_total(*track_total);
        } else {
            tag.remove_track_total();
        }

        if let Some(composer) = &self.composer {
            tag.insert_text(ItemKey::Composer, composer.to_string());
        } else {
            tag.remove_key(&ItemKey::Composer);
        }

        if let Some(lyricist) = &self.lyricist {
            tag.insert_text(ItemKey::Lyricist, lyricist.to_string());
        } else {
            tag.remove_key(&ItemKey::Lyricist);
        }

        if let Some(lyrics) = &self.lyrics {
            tag.insert_text(ItemKey::Lyrics, lyrics.to_string());
        } else {
            tag.remove_key(&ItemKey::Lyrics);
        }

        if let Some(comment) = &self.comment {
            tag.set_comment(comment.to_string());
        } else {
            tag.remove_comment();
        }

        if let Some(pictures) = &self.pictures {
            let mut i = 0;
            for picture in pictures.iter() {
                if let Some(pic) = to_lofty_picture(picture) {
                    tag.set_picture(i, pic);
                    i += 1;
                }
            }
            if i < tag.pictures().len() {
                for j in i..tag.pictures().len() {
                    tag.remove_picture(j);
                }
            }
        } else {
            for i in 0..tag.pictures().len() {
                tag.remove_picture(i);
            }
        }
        let _ = tag.save_to(&mut reader, WriteOptions::default());
        Metadata::new(reader.into_inner())
    }
}

