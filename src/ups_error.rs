use std::{fmt, error};
use std::fmt::Formatter;

/// Error type returned when something goes wrong
///
/// This is what is actually returned from all the functions, and its the job of hte program
/// that implements this library to deal with the error and whatever it contains
#[derive(Debug, Eq, PartialEq)]
pub enum UpsError{
    Load(LoadError),
    Apply(ApplyError),
    Create(CreateError),
}
/// Errors that happen when loading an already made patch
#[derive(Debug, Eq, PartialEq)]
pub enum LoadError{
    /// The given UPS file isn't actually a UPS patch file
    IsNotUpsFile,
    /// The given UPS file is a ups file but seems to be corrupted.
    IsCorrupted,
    /// Any other error
    Unknown
}
/// Errors that happen when applying a patch to a file
#[derive(Debug, Eq, PartialEq)]
pub enum ApplyError{
    /// The provided source file doesn't match the patch
    SourceMismatch,
    /// The result after patching a valid source file doesn't match
    TargetMismatch,
    /// Any other error
    Unknown
}
/// Errors that happen when creating a file from source and target files
///
///
/// there is no way for a ups error to actually be a create error at the moment, but in the future
/// it may be necessary
#[derive(Debug, Eq, PartialEq)]
pub enum CreateError{

    Unknown
}

impl fmt::Display for UpsError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Ups error: {}", self.message())
    }
}
impl error::Error for UpsError{}

impl UpsError{
    pub fn message(&self) -> &'static str {
        match self {
            UpsError::Load(load_error) => {match load_error {
                LoadError::IsNotUpsFile => "File provided is not a UPS Patch file",
                LoadError::IsCorrupted => "FIle provided apears to be corrupted, doesn't match crc32",
                _ => "Unknown error during patch load"
            }}
            UpsError::Apply(apply_error) => { match apply_error {
                ApplyError::SourceMismatch => "Source file doesn't match crc32 for source file",
                ApplyError::TargetMismatch => "Final target file doesn't match crc32 for target file",
                _ => "Unknown error during patch apply"
            }}
            UpsError::Create(create_error) => {"Unknown Error during patch creation"}
        }
    }
}