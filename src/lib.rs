
mod tag;
mod utils;

#[test]
fn test_image_mime() {
    use crate::tag::Metadata;
    let path = "./samples/mp3.mp3";
    let buf = std::fs::read(path).unwrap();
    let data = Metadata::new(buf);
    assert_eq!(data.pictures.expect("no pictures")[0].mime_type, "image/png");
}