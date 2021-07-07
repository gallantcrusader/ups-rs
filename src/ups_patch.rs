use std::convert::TryInto;

use crate::{
    crc32,
    UpsError,
    LoadError::*,
    CreateError::*,
    ApplyError::*,
};


///Represents a  ups patch
#[derive(Debug)]
pub struct UpsPatch {
    ///The file size of the original file
    pub source_file_size: u64,
    ///The file size of the final file
    pub target_file_size: u64,
    /// The changes that have to be applied to the original file to get the final file.
    /// They are stored in a vector of tuples containing offset and change list  where:
    ///
    /// 0, Offset:
    /// A u64 pointer describing where the current difference starts.
    ///
    /// 1, ChangeList:
    /// A vector of the XOR bytes that have to be applied to the source file in the current position.
    pub changes: Vec<(u64, Vec<u8>)>,
    /// The crc32 checksum of the source file
    pub source_crc32: u32,
    /// The crc32 checksum of the final file
    pub target_crc32: u32,
    /// The crc 32 checksum of the patch file itself
    pub patch_crc32: u32,

}

impl UpsPatch {
    const CANON_HEADER: [u8; 4] = [0x55, 0x50, 0x53, 0x31];

    /// Creates a Patch from the given source and target files
    /// # Arguments
    /// * `source_content` - The contents of the source/original file
    /// * `target_content` - The contents of the target/final file
    ///
    pub fn create(source_content: &Vec<u8>, target_content: &Vec<u8>) -> UpsPatch {
        let source_crc32 = crc32::calculate(&*source_content);
        let target_crc32 = crc32::calculate(&*target_content);

        let source_file_size = source_content.len() as u64;
        let target_file_size = target_content.len() as u64;
        let mut changes: Vec<(u64, Vec<u8>)> = vec![];

        let max_size = if source_file_size > target_file_size { source_file_size } else { target_file_size };


        let mut i: u64 = 0;
        while i < max_size {
            let mut x: u8 = if i < source_file_size { source_content[i as usize] } else { 0x00 };
            let mut y: u8 = if i < target_file_size { target_content[i as usize] } else { 0x00 };
            if x != y {
                let change_offset = i;
                let mut changed_bytes: Vec<u8> = vec![];
                while x != y && i < max_size {
                    changed_bytes.push(x ^ y);
                    i += 1;
                    x = if i < source_file_size { source_content[i as usize] } else { 0x00 };
                    y = if i < target_file_size { target_content[i as usize] } else { 0x00 };
                }
                changes.push((change_offset, changed_bytes))
            }
            i += 1
        }

        let bytearray = UpsPatch::tailless_bytearray(source_file_size,
                                                     target_file_size,
                                                     &changes,
                                                     source_crc32,
                                                     target_crc32);
        let patch_crc32 = crc32::calculate(&*bytearray);

        UpsPatch {
            source_file_size,
            target_file_size,
            changes,
            source_crc32,
            target_crc32,
            patch_crc32,
        }
    }
    /// Loads an already existing patch, if the given file contents don't contain a valid UPS patch returns a UpsError
    /// # Arguments
    /// * `content` - The content of the patch file to load
    /// # Examples
    /// Load a patch and throw a panic if the file isn't a valid ups Patch
    /// ```no_run
    /// # use ups::UpsPatch;
    /// # let file_content=vec![];
    /// let patch = match UpsPatch::load(&file_content) {
    ///     Ok(patch) => patch,
    ///     Err(why) => panic!("Couldn't load UPS patch:{}", why)
    /// };
    ///
    /// ```
    ///
    pub fn load(content: &Vec<u8>) -> Result<UpsPatch, UpsError> {
        if content[0..4] != UpsPatch::CANON_HEADER {
            return Err(UpsError::Load(IsNotUpsFile));
        }
        let l = content.len();
        let patch_crc32 = u32::from_le_bytes(content[l - 4..l].try_into().unwrap());
        let patch_computed_crc32 = crc32::calculate(&content[0..l - 4]);

        if patch_computed_crc32 != patch_crc32 {
            return Err(UpsError::Load(IsCorrupted));
        }
        let source_crc32 = u32::from_le_bytes(content[l - 12..l - 8].try_into().unwrap());
        let target_crc32 = u32::from_le_bytes(content[l - 8..l - 4].try_into().unwrap());

        let mut i: usize = 4;

        let (new_i, source_file_size) = UpsPatch::find_pointer(&content, i);
        i = new_i;
        let (new_i, target_file_size) = UpsPatch::find_pointer(&content, i);
        i = new_i;

        let mut changes: Vec<(u64, Vec<u8>)> = vec![];
        while i < l - 13 {
            let (new_i, offset_dif) = UpsPatch::find_pointer(&content, i);
            i = new_i;
            let mut xor_bytes: Vec<u8> = vec![];
            while content[i] != 0 {
                xor_bytes.push(content[i]);
                i += 1;
            }
            changes.push((offset_dif, xor_bytes))
        }


        let file = UpsPatch {
            source_file_size,
            target_file_size,
            source_crc32,
            target_crc32,
            patch_crc32,
            changes,
        };
        Ok(file)
    }

    /// Given the contents of a file, verifies that it is the expected source for the patch,
    /// applies the patch and verifies that the output is the expected target for the patch.
    /// # Arguments
    /// * `source` - The content of the source file
    /// # Examples
    /// Load a patch, apply it and save to a variable if everything is ok or panic if something went wrong
    ///
    pub fn apply(&self, source:&Vec<u8>) -> Result<Vec<u8>, UpsError>{
        if !self.file_is_source(&source) {
            return Err(UpsError::Apply(SourceMismatch))
        }
        let target = self.apply_no_check(&source);
        if !self.file_is_target(&target) {
            return Err(UpsError::Apply(TargetMismatch))
        }
        Ok(target)

    }

    /// Applies a patch to a given source file contents.
    /// This function doesn't check for file to actually be the correct source file, it just
    /// applies the patch.
    pub fn apply_no_check(&self, source: &Vec<u8>) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        let mut i: u64 = 0;

        for change in &self.changes {
            while i < change.0 {
                output.push(source[i as usize]);
                i += 1;
            }
            for xor_byte in &change.1 {
                let source_byte = if i < source.len() as u64 { source[i as usize] } else { 0x00 };

                output.push(source_byte ^ xor_byte);
                i += 1;
            }
        }
        while i < self.target_file_size {
            output.push(source[i as usize]);
            i += 1;
        }
        if output.len() > self.target_file_size as usize {
            output = output[0..self.target_file_size as usize].to_owned()
        }

        return output;
    }
    fn tailless_bytearray(source_file_size: u64,
                          target_file_size: u64,
                          changes: &Vec<(u64, Vec<u8>)>,
                          source_crc32: u32,
                          target_crc32: u32, ) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        output.extend(UpsPatch::CANON_HEADER);
        output.extend(UpsPatch::encode(source_file_size));
        output.extend(UpsPatch::encode(target_file_size));
        for change in changes {
            output.extend(UpsPatch::encode(change.0));
            for byte in &change.1 {
                output.push(*byte)
            }
            output.push(0x00)
        }
        output.extend(source_crc32.to_le_bytes());
        output.extend(target_crc32.to_le_bytes());
        return output;
    }
    /// Returns a vector with the contents of the patch.ups file
    pub fn get_patch_file_contents(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        output.extend(UpsPatch::CANON_HEADER);
        output.extend(UpsPatch::encode(self.source_file_size));
        output.extend(UpsPatch::encode(self.target_file_size));
        for change in &self.changes {
            output.extend(UpsPatch::encode(change.0));
            for byte in &change.1 {
                output.push(*byte)
            }
            output.push(0x00)
        }
        output.extend(self.source_crc32.to_le_bytes());
        output.extend(self.target_crc32.to_le_bytes());
        output.extend(self.patch_crc32.to_le_bytes());
        return output;
    }

    /// Checks if the given source file matches the source file for the UPS patch
    pub fn file_is_source(&self, content: &Vec<u8>) -> bool {
        let file_crc32 = crc32::calculate(&content);
        return file_crc32 == self.source_crc32;
    }

    pub fn file_is_target(&self, content : &Vec<u8>) -> bool {
        let file_crc32 = crc32::calculate(&content);
        return file_crc32 == self.target_crc32;
    }

    fn find_pointer(buff: &Vec<u8>, start: usize) -> (usize, u64) {
        let (i, encoded_pointer) = UpsPatch::find_encoded_value(&buff, start);
        let decoded_pointer = UpsPatch::decode(encoded_pointer);
        return (i, decoded_pointer);
    }
    fn find_encoded_value(buff: &Vec<u8>, start: usize) -> (usize, Vec<u8>) {
        let mut i = start;
        while buff[i] & 0x80 == 0 {
            i += 1;
        }
        i += 1;
        return (i, buff[start..i].to_owned());
    }

    fn decode(input: Vec<u8>) -> u64 {
        let mut value: u64 = 0;
        let mut shift: u32 = 1;
        let mut i: usize = 0;
        let mut x: u8 = input[i];
        i += 1;
        value += ((x & 0x7f) as u32 * shift) as u64;
        while x & 0x80 == 0 {
            shift <<= 7;
            value += shift as u64;
            x = input[i];
            i += 1;
            value += ((x & 0x7F) as u32 * shift) as u64;
        }

        return value;
    }




    fn encode(input: u64) -> Vec<u8> {
        let mut input = input;
        let mut bytes: Vec<u8> = vec![];

        let mut x = input & 0x7f;
        input >>= 7;
        while input != 0
        {
            bytes.push(x as u8);
            input -= 1;
            x = input & 0x7f;
            input >>= 7;
        }
        bytes.push((0x80 | x) as u8);
        return bytes;
    }


}
#[cfg(test)]
mod internal_tests {
    use crate::{
        crc32,
        UpsPatch
    };


    #[test]
    fn can_decode(){
        assert_eq!(UpsPatch::decode(vec![0x0,0x7f,0x7e,0x86]), 16777216)
    }

    #[test]
    fn can_encode(){
        assert_eq!(UpsPatch::encode(16777216), vec![0x0,0x7f,0x7e,0x86])
    }

    #[test]
    fn can_perform_crc32_checksum(){
        let content = vec![0x11, 0x22, 0x33, 0x44];
        assert_eq!(crc32::calculate(&*content), 0x77F29DD1 )
    }

}