use crate::binary_reader::BinaryReader;
pub use crate::souls_file_reader::SoulsFileReader;

pub struct FLVER2 {
}

impl SoulsFileReader for FLVER2 {
    fn is(&self, br: &mut BinaryReader) -> bool {
        todo!()
    }

    fn specific_read(&self, br: &mut BinaryReader) {
        FLVER2::default();
    }
}

impl Default for FLVER2 {
    fn default() -> Self {
        Self {  }
    }
}