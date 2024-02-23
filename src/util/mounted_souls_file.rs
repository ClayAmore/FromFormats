use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use crate::util::sf_util::SFUtil;
use crate::util::binary_reader::BinaryReader;
use crate::formats::CompressionType;
use crate::util::souls_file::SoulsFile;

// Common functions for all souls filetypes
pub trait MountedSoulsFile: SoulsFile {
    fn is(&self, br: &mut BinaryReader) -> bool;
    fn read(file_path: &PathBuf) -> Self {
        // Create an instance of the specified type using the default constructor.
        let format = Self::default();

        // Read and initialize the instance from the provided file path.
        
        MountedSoulsFile::common_read(&format,file_path);

        // Return the initialized instance.
        return format;
    }
    fn common_read(&self, file_path: &PathBuf) {
        // Open the file and handle potential errors
        let file = File::open(file_path).expect("Failed to open file");

        // Prepare to read the file contents into a Vec<u8>
        let mut contents: Vec<u8> = Vec::new();
        let mut file_reader = BufReader::new(file);

        // Read the file content and handle potential errors
        file_reader.read_to_end(&mut contents).expect("Failed to read file!");

        let mut br = BinaryReader::new(false, contents);
        let mut compression = CompressionType::Unknown;

        // Check and decompress the file if necessary
        SFUtil::decompress_if_neccessary(&mut br, &mut compression).expect("Error in decompressing!");

        // Delegate to the specific implementation for the provided reader
        MountedSoulsFile::specific_read(self, &mut br);
    }
    fn specific_read(&self, br: &mut BinaryReader);
}
