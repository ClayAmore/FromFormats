use crate::dcx::{DCX, CompressionType};
use crate::binary_reader::BinaryReader;

pub(crate) struct SFUtil {

}

impl SFUtil {
    pub(crate) fn decompress_if_neccessary(br: &mut BinaryReader, compression: &mut CompressionType){
        if DCX::is(br) {
            let bytes = DCX::decompress(br, compression);
            br.set_memory(bytes);
            println!("It is a DCX file!");

        } else {
            println!("It is not a DCX file!");
            *compression = CompressionType::None;
        };
    }

    pub(crate) fn read_zlib(br: &mut BinaryReader, compression_size: usize) -> Vec<u8> {
        todo!()
    }
}