
mod picture;
mod tag;
mod utils;

#[test]
fn test_image_mime() {
    use crate::tag::MetaFile;

    let path = "./samples/mp3.mp3";
    let path1 = "./samples/mp31.mp3";
    let buf = std::fs::read(path).unwrap();
    let mut data = MetaFile::new(buf).ok().unwrap();
    data.set_album("test".to_string());
    assert_eq!(data.get_pictures().expect("no pictures")[0].mime_type, Some("image/png".to_string()));
    let _ = data.save();

    let _ = std::fs::write(path1, data.buffer());
    let buf1 = std::fs::read(path1).unwrap();
    let data1 = MetaFile::new(buf1).ok().unwrap();
    assert_eq!(data1.get_pictures().expect("no pictures")[0].mime_type, Some("image/png".to_string()));
    assert_eq!(data1.get_album(), Some("test".to_string()));
}