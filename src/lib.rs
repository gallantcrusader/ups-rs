//! # ups
//! Rust implementation of a UPS Patcher
//! ## Usage
//!
//! ### Apply UPS patch:
//! ```no_run
//! use ups::UpsPatch;
//! use std::fs::File;
//! use std::io::{Read, Write};
//!
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>>{
//! //Loading the contents of the source and patch files
//! let mut  source_file_content: Vec<u8> = vec![];
//! let mut  patch_file_content: Vec<u8> = vec![];
//! let mut  source_file = File::open("path/to/source/file")?;
//! source_file.read_to_end(&mut source_file_content);
//! let mut patch_file = File::open("path/to/patch/file")?;
//! patch_file.read_to_end(&mut patch_file_content);
//!
//! //Actually applying the patch
//! let patch = UpsPatch::load(&patch_file_content)?;
//! let patched_file_content = patch.apply(&source_file_content)?;
//! //Saving the target file contents to a file
//! let mut target_file = File::open("path/to/target/file")?;
//! target_file.write_all(&*patched_file_content);
//!
//! # Ok(())
//! # }
//! ```
//! ### Create UPS Patch
//! ```no_run
//! use ups::UpsPatch;
//! use std::fs::File;
//! use std::io::{Read, Write};
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>>{
//! //LOad the contents of the source and patch files
//! let mut  source_file_content: Vec<u8> = vec![];
//! let mut  target_file_content: Vec<u8> = vec![];
//! let mut  source_file = File::open("path/to/source/file")?;
//! source_file.read_to_end(&mut source_file_content);
//! let mut target_file = File::open("path/to/patch/file")?;
//! target_file.read_to_end(&mut target_file_content);
//!
//! //Create the UpsPatch
//! let patch = UpsPatch::create(&source_file_content, &target_file_content);
//! //Write the patch to a file
//! let patch_file_content = patch.get_patch_file_contents();
//! let mut patch_file = File::open("path/to/target/file")?;
//! patch_file.write_all(&patch_file_content);
//!
//! # Ok(())
//! # }
//!
//! ```


pub use crate::ups_patch::UpsPatch;
pub use crate::ups_error::{UpsError, ApplyError, CreateError, LoadError};
mod ups_patch;
mod crc32;
mod ups_error;