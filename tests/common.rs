use std::fs::File;
use std::io::Read;

pub const SOURCE_PATH: &str = "tests/source.txt";
pub const TARGET_PATH_1: &str = "tests/final1.txt";
pub const PATCH_PATH_1: &str = "tests/patch1.ups";
pub const TARGET_PATH_2: &str = "tests/final2.txt";
pub const PATCH_PATH_2: &str = "tests/patch2.ups";


pub fn load_file_content(path: &str) -> Vec<u8> {
    let mut patch_file = File::open(path).unwrap();
    let mut content : Vec<u8> = vec![];
    patch_file.read_to_end(&mut content);
    return content
}