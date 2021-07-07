# ups-rs
A rust implementation of a UPS file patcher.

I am no rust expert, and this was made for me to actually learn rust, feel free to open an issue pointing any mistake you find.

## Usage

### Apply UPS patch:
 ```rust
 use ups::UpsPatch;
use std::fs::File;
use std::io::{Read, Write};

 //Loading the contents of the source and patch files
 let mut  source_file_content: Vec<u8> = vec![];
 let mut  patch_file_content: Vec<u8> = vec![];
 let mut  source_file = File::open("path/to/source/file")?;
 source_file.read_to_end(&mut source_file_content);
 let mut patch_file = File::open("path/to/patch/file")?;
 patch_file.read_to_end(&mut patch_file_content);

 //Apply the patch
 let patch = UpsPatch::load(&patch_file_content)?;
 let patched_file_content = patch.apply(&source_file_content)?;
 //Write the target to a file
 let mut target_file = File::open("path/to/target/file")?;
 target_file.write_all(&*patched_file_content);

 ```

### Create UPS Patch
 ```rust
 use ups::UpsPatch;
 use std::fs::File;
 use std::io::{Read, Write};
 //Load the contents of the source and patch files
 let mut  source_file_content: Vec<u8> = vec![];
 let mut  target_file_content: Vec<u8> = vec![];
 let mut  source_file = File::open("path/to/source/file")?;
 source_file.read_to_end(&mut source_file_content);
 let mut target_file = File::open("path/to/patch/file")?;
 target_file.read_to_end(&mut target_file_content);

 //Create the UpsPatch
 let patch = UpsPatch::create(&source_file_content, &target_file_content);
 //Write the patch to a file
 let patch_file_content = patch.get_patch_file_contents();
 let mut patch_file = File::open("path/to/target/file")?;
 patch_file.write_all(&patch_file_content);
 ```

##Documentation
The documentation is on [docs.rs](https://docs.rs/ups)
## Contributing:
Feel free to submit pull requests with improvements.

## License
All files in this repository except [UPS-spec.pdf](https://github.com/Laikar/ups-rs/blob/main/ups-spec.pdf "UPS-spec.pdf") and [ups_spec.md](https://github.com/Laikar/ups-rs/blob/main/ups_spec.md "ups_spec.md") are released under [MIT License](https://github.com/Laikar/ups-rs/blob/main/License.md "MIT License").

[ups-spec.pdf](https://github.com/Laikar/ups-rs/blob/main/ups-spec.pdf "ups-spec.pdf") is the original ups spec file released under  [Attribution-NonCommercial-NoDerivs 3.0 Unported (CC BY-NC-ND 3.0)](https://creativecommons.org/licenses/by-nc-nd/3.0/ "Attribution-NonCommercial-NoDerivs 3.0 Unported (CC BY-NC-ND 3.0)").

[ups_spec.md](https://github.com/Laikar/ups-rs/blob/main/ups_spec.md "ups_spec.md") is a conversion of the original ups-spec.pdf into a more github friendly format, and keeps the same license as the original, [Attribution-NonCommercial-NoDerivs 3.0 Unported (CC BY-NC-ND 3.0)](https://creativecommons.org/licenses/by-nc-nd/3.0/ "Attribution-NonCommercial-NoDerivs 3.0 Unported (CC BY-NC-ND 3.0)")