use std::fs::File;
use std::io::Read;

pub const SOURCE_PATH: &str = "tests/source.txt";
pub const TARGET_PATH: &str = "tests/final.txt";
pub const PATCH_PATH: &str = "tests/patch.ups";


pub fn load_file_content(path: &str) -> Vec<u8> {
    let mut patch_file = File::open(path).unwrap();
    let mut content : Vec<u8> = vec![];
    patch_file.read_to_end(&mut content);
    return content
}