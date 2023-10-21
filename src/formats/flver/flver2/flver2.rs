use crate::util::binary_reader::BinaryReader;
use crate::util::SoulsFile;

pub struct FLVER2 {
}

impl SoulsFile for FLVER2 {
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