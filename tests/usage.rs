use ups::UpsPatch;
mod common;
use common::*;
#[test]
fn can_load_patch() {
    let content_1 = load_file_content(PATCH_PATH_1);
    let patch_1 = UpsPatch::load(&content_1).unwrap();
    assert_eq!(patch_1.source_file_size, 28);
    assert_eq!(patch_1.target_file_size, 27);
    assert_eq!(
        patch_1.changes,
        vec![(
            17,
            vec![0x15, 0x06, 0x1B, 0x13, 0x0F, 0x45, 0x46, 0x0F, 0x05, 0x09, 0x65,]
        )]
    );
    assert_eq!(patch_1.source_crc32, 0x29E0B36E);
    assert_eq!(patch_1.target_crc32, 0x23a777e3);
    assert_eq!(patch_1.patch_crc32, 0xffa6802b);

    let content_2 = load_file_content(PATCH_PATH_2);
    let patch_2 = UpsPatch::load(&content_2).unwrap();
    assert_eq!(patch_2.source_file_size, 28);
    assert_eq!(patch_2.target_file_size, 34);
    assert_eq!(
        patch_2.changes,
        vec![
            (17, vec![0x1f]),
            (19, vec![0x1B, 0x15, 0x06, 0x17]),
            (26, vec![0x02, 0x04, 0x6C, 0x20, 0x66, 0x69, 0x6C, 0x65]),
        ]
    );
    assert_eq!(patch_2.source_crc32, 0x29E0B36E);
    assert_eq!(patch_2.target_crc32, 0xE1CFB1EB);
    assert_eq!(patch_2.patch_crc32, 0x34616045);
}

#[test]
fn can_verify_source() {
    let source_content = load_file_content(SOURCE_PATH);

    let content_1 = load_file_content(PATCH_PATH_1);
    let patch_1 = UpsPatch::load(&content_1).unwrap();
    assert!(patch_1.file_is_source(&source_content));

    let content_2 = load_file_content(PATCH_PATH_2);
    let patch_2 = UpsPatch::load(&content_2).unwrap();
    assert!(patch_2.file_is_source(&source_content));
}
#[test]
fn can_verify_target() {
    let content_1 = load_file_content(PATCH_PATH_1);
    let patch_1 = UpsPatch::load(&content_1).unwrap();
    let target_content_1 = load_file_content(TARGET_PATH_1);
    assert!(patch_1.file_is_target(&target_content_1));

    let content_2 = load_file_content(PATCH_PATH_2);
    let patch_2 = UpsPatch::load(&content_2).unwrap();
    let target_content_2 = load_file_content(TARGET_PATH_2);
    assert!(patch_2.file_is_target(&target_content_2));
}

#[test]
fn can_create_patch() {
    let source_content = load_file_content(SOURCE_PATH);

    let target_content_1 = load_file_content(TARGET_PATH_1);
    let patch_content_1 = load_file_content(PATCH_PATH_1);
    let created_patch_1 = UpsPatch::create(&source_content, &target_content_1);
    let loaded_patch_1 = UpsPatch::load(&patch_content_1).unwrap();
    assert_eq!(created_patch_1, loaded_patch_1);
    assert_eq!(
        created_patch_1.get_patch_file_contents(),
        loaded_patch_1.get_patch_file_contents()
    );

    let target_content_2 = load_file_content(TARGET_PATH_2);
    let patch_content_2 = load_file_content(PATCH_PATH_2);
    let created_patch_2 = UpsPatch::create(&source_content, &target_content_2);
    let loaded_patch_2 = UpsPatch::load(&patch_content_2).unwrap();
    assert_eq!(created_patch_2, loaded_patch_2);
    assert_eq!(
        created_patch_2.get_patch_file_contents(),
        loaded_patch_2.get_patch_file_contents()
    );
}

/*#[test]
fn can_create_patch(){
    let source_content = load_file_content(SOURCE_PATH);

    let target_content_1 = load_file_content(TARGET_PATH_1);
    let patch_file_content_1 = load_file_content(PATCH_PATH_1);
    let created_patch_1 = UpsPatch::create(&source_content, &target_content_1);
    assert_eq!(created_patch_1.get_patch_file_contents(), patch_file_content_1);

    let target_content_2 = load_file_content(TARGET_PATH_2);
    let patch_file_content_2 = load_file_content(PATCH_PATH_2);
    let created_patch_2 = UpsPatch::create(&source_content, &target_content_2);
    assert_eq!(created_patch_2.get_patch_file_contents(), patch_file_content_2);
}*/

#[test]
fn can_apply_no_test() {
    let source_content = load_file_content(SOURCE_PATH);

    let target_content_1 = load_file_content(TARGET_PATH_1);
    let patch_file_content_1 = load_file_content(PATCH_PATH_1);
    let patch_1 = UpsPatch::load(&patch_file_content_1).unwrap();
    let final_file_content_1 = patch_1.apply_no_check(&source_content);
    assert_eq!(final_file_content_1, target_content_1);

    let target_content_2 = load_file_content(TARGET_PATH_2);
    let patch_file_content_2 = load_file_content(PATCH_PATH_2);
    let patch_2 = UpsPatch::load(&patch_file_content_2).unwrap();
    let final_file_content_2 = patch_2.apply_no_check(&source_content);

    assert_eq!(final_file_content_2, target_content_2);
}

#[test]
fn can_apply() {
    let source_content = load_file_content(SOURCE_PATH);

    let target_content_1 = load_file_content(TARGET_PATH_1);
    let patch_file_content_1 = load_file_content(PATCH_PATH_1);
    let patch_1 = UpsPatch::load(&patch_file_content_1).unwrap();
    let final_file_content_1 = patch_1.apply(&source_content).unwrap();
    assert_eq!(final_file_content_1, target_content_1);

    let target_content_2 = load_file_content(TARGET_PATH_2);
    let patch_file_content_2 = load_file_content(PATCH_PATH_2);
    let patch_2 = UpsPatch::load(&patch_file_content_2).unwrap();
    let final_file_content_2 = patch_2.apply(&source_content).unwrap();
    assert_eq!(final_file_content_2, target_content_2);
}
