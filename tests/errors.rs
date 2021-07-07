use std::fs::File;
use std::io::Read;

use ups::{UpsPatch};
use ups::ups_error::{
    *,
    LoadError::*,
    CreateError::*,
    ApplyError::*,
};
mod common;
use common::*;
#[test]
fn throws_not_ups_file_error(){
    let source_content = load_file_content(SOURCE_PATH);
    let result = UpsPatch::load(&source_content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), UpsError::Load(IsNotUpsFile))
}
#[test]
fn throws_corrupt_file_error(){
    let mut content = load_file_content(PATCH_PATH);
    content[7] = 0x12;
    let result = UpsPatch::load(&content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), UpsError::Load(IsCorrupted))
}

#[test]
fn throws_wrong_source(){
    let patch_content = load_file_content(PATCH_PATH);
    let patch = UpsPatch::load(&patch_content).unwrap();
    let mut source_content = load_file_content(SOURCE_PATH);
    assert!(patch.file_is_source(&source_content));
    source_content[7] = 0x12;
    assert!(!patch.file_is_source(&source_content))
}

#[test]
fn throws_wrong_target(){
    let patch_content = load_file_content(PATCH_PATH);
    let patch = UpsPatch::load(&patch_content).unwrap();
    let mut target_content = load_file_content(TARGET_PATH);
    assert!(patch.file_is_target(&target_content));
    target_content[7] = 0x12;
    assert!(!patch.file_is_target(&target_content))
}