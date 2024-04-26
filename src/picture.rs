use lofty::picture::{MimeType, PictureType};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct MetaPicture {
    /// The picture type according to ID3v2 APIC
    #[wasm_bindgen(js_name = "picType")]
    pub pic_type: String,

    /// The picture's mimetype
    #[wasm_bindgen(js_name = "mimeType")]
    pub mime_type: Option<String>,

    /// The picture's description
    pub description: Option<String>,

    /// The binary data of the picture
    pub data: Vec<u8>,
}

pub fn mimetype_enum_to_string(mime_type: Option<&MimeType>) -> Option<String> {
    match mime_type {
        Some(MimeType::Png) => Some("image/png".to_string()),
        Some(MimeType::Jpeg) => Some("image/jpeg".to_string()),
        Some(MimeType::Bmp) => Some("image/bmp".to_string()),
        Some(MimeType::Tiff) => Some("image/tiff".to_string()),
        Some(MimeType::Gif) => Some("image/gif".to_string()),
        Some(MimeType::Unknown(type_str)) => {
            let unknown_type = format!("image/{}", type_str);
            Some(unknown_type)
        }
        _ => None,
    }
}

pub fn string_to_mimetype_enum(mime_type: Option<&str>) -> Option<MimeType> {
    match mime_type {
        Some("image/png") => Some(MimeType::Png),
        Some("image/jpeg") => Some(MimeType::Jpeg),
        Some("image/bmp") => Some(MimeType::Bmp),
        Some("image/tiff") => Some(MimeType::Tiff),
        Some("image/gif") => Some(MimeType::Gif),
        _ => None,
    }
}

pub fn from_lofty_picture_vec(
    pictures: &Option<Vec<lofty::picture::Picture>>,
) -> Option<Vec<MetaPicture>> {
    pictures.as_ref().map(|pics| {
        pics.iter()
            .map(|p| MetaPicture {
                pic_type: p.pic_type().as_ape_key().unwrap_or("").to_string(),
                mime_type: mimetype_enum_to_string(p.mime_type()),
                description: p.description().map(|s| s.to_string()),
                data: p.data().to_vec(),
            })
            .collect()
    })
}

pub fn to_lofty_picture(pic: &MetaPicture) -> Option<lofty::picture::Picture> {
    if pic.data.is_empty() {
        return None;
    }
    Some(lofty::picture::Picture::new_unchecked(
        PictureType::from_ape_key(pic.pic_type.as_str()),
        string_to_mimetype_enum(pic.mime_type.as_deref()),
        pic.description.clone(),
        pic.data.clone(),
    ))
}