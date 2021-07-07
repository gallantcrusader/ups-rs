use std::{fmt, error};
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq)]
pub enum UpsError{
    Load(LoadError),
    Apply(ApplyError),
    Create(CreateError),
}
#[derive(Debug, Eq, PartialEq)]
pub enum LoadError{
    IsNotUpsFile,
    IsCorrupted
}
#[derive(Debug, Eq, PartialEq)]
pub enum ApplyError{
    SourceMismatch,
    TargetMismatch
}
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
            UpsError::Load(loadError) => {match loadError {
                LoadError::IsNotUpsFile => {"File provided is not a UPS Patch file"}
                LoadError::IsCorrupted => {"FIle provided apears to be corrupted, doesn't match crc32"}
            }}
            UpsError::Apply(applyError) => { match applyError {
                ApplyError::SourceMismatch => {"Source file doesn't match crc32 for source file"}
                ApplyError::TargetMismatch => {"Final target file doesn't match crc32 for target file"}
            }}
            UpsError::Create(createError) => {"Unknown Error during patch creation"}
        }
    }
}