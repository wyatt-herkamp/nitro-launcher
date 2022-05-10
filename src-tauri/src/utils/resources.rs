use std::fs::read;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/resources"]
pub struct Resources;

impl Resources {
    pub fn file_get(file: &str) -> Vec<u8> {
        let buf = Path::new("resources").join(file);
        if buf.exists() {
            read(buf).unwrap()
        } else {
            Resources::get(file).unwrap().data.to_vec()
        }
    }
    pub fn file_get_string(file: &str) -> String {
        let vec = Resources::file_get(file);
        String::from_utf8(vec).unwrap()
    }
}


pub fn get_nitro_launcher_dir()->PathBuf{
    dirs::home_dir().unwrap().join(".nitro_launcher")
}