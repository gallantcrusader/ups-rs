
use ups::{
    UpsError,
    UpsPatch,
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
    let mut content = load_file_content(PATCH_PATH_1);
    content[7] = content[7]+1;
    let result = UpsPatch::load(&content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), UpsError::Load(IsCorrupted))
}

#[test]
fn throws_wrong_source_error(){
    let patch_content = load_file_content(PATCH_PATH_1);
    let patch = UpsPatch::load(&patch_content).unwrap();
    let mut source_content = load_file_content(SOURCE_PATH);
    assert!(patch.file_is_source(&source_content));
    source_content[7] = source_content[7]+1;
    let result = patch.apply(&source_content);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), UpsError::Apply(SourceMismatch))
}