use std::fs::File;
use std::io::Read;

use ups::{
    UpsPatch
};

const SOURCE_PATH: &str = "tests/source.txt";
const TARGET_PATH: &str = "tests/final.txt";
const PATCH_PATH: &str = "tests/patch.ups";







fn load_file_content(path: &str) -> Vec<u8> {
    let mut patch_file = File::open(path).unwrap();
    let mut content : Vec<u8> = vec![];
    patch_file.read_to_end(&mut content);
    return content
}

#[test]
fn can_load_patch(){
    let content = load_file_content(PATCH_PATH);
    let patch = UpsPatch::load(content).unwrap();
    assert_eq!(patch.source_file_size, 28);
    assert_eq!(patch.target_file_size, 27);
    assert_eq!(patch.source_crc32, 0x29E0B36E);
    assert_eq!(patch.target_crc32, 0x23a777e3);
    assert_eq!(patch.patch_crc32, 0xffa6802b)
}

#[test]
fn can_verify_source(){
    let content = load_file_content(PATCH_PATH);
    let patch = UpsPatch::load(content).unwrap();
    let source_content = load_file_content(SOURCE_PATH);
    assert!(patch.file_is_source(&source_content));
}


#[test]
fn can_create_patch(){
    let source_content = load_file_content(SOURCE_PATH);
    let target_content = load_file_content(TARGET_PATH);
    let created_patch = UpsPatch::create(&source_content, &target_content);
    let loaded_patch = UpsPatch::load(load_file_content(PATCH_PATH)).unwrap();
    assert_eq!(created_patch.patch_crc32, loaded_patch.patch_crc32)
}

#[test]
fn created_patch_is_equal_to_sample(){
    let source_content = load_file_content(SOURCE_PATH);
    let target_content = load_file_content(TARGET_PATH);
    let patch_file_content = load_file_content(PATCH_PATH);
    let created_patch = UpsPatch::create(&source_content, &target_content);
    assert_eq!(created_patch.get_patch_file_contents(), patch_file_content)
}

#[test]
fn can_apply_patch(){
    let source_content = load_file_content(SOURCE_PATH);
    let target_content = load_file_content(TARGET_PATH);
    let patch_file_content = load_file_content(PATCH_PATH);
    let patch = UpsPatch::load(patch_file_content).unwrap();
    let final_file_content = patch.apply_to(source_content);
    assert_eq!(final_file_content, target_content)
}