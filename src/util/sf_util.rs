use std::io::{Cursor, Read, Error};
use flate2::read::ZlibDecoder;

use crate::formats::{DCX, CompressionType};
use crate::util::binary_reader::BinaryReader;

pub(crate) struct SFUtil {

}

impl SFUtil {
    pub(crate) fn decompress_if_neccessary(br: &mut BinaryReader, compression: &mut CompressionType) -> Result<(), Error>{
        if DCX::is(br) {
            let bytes = DCX::decompress(br, compression)?;
            br.memory = bytes;
            println!("It is a DCX file!");

        } else {
            println!("It is not a DCX file!");
            *compression = CompressionType::None;
        };

        Ok(())
    }

    pub(crate) fn read_zlib(br: &mut BinaryReader, compression_size: usize) -> Result<Vec<u8>, Error> {
        // Ensure the first two bytes match the Zlib compression header.
        br.assert_byte(&[0x78]);
        br.assert_byte(&[0x01, 0x5E, 0x9C, 0xDA]);
    
        // Read the compressed data.
        let compressed = br.read_bytes(compression_size - 2);
    
        // Create a Cursor from the compressed data.
        let compressed_stream = Cursor::new(compressed);
    
        // Create a Vec to store the decompressed data.
        let mut decompressed_data = Vec::new();
    
        // Create a ZlibDecoder to handle the decompression.
        let mut decoder = ZlibDecoder::new(compressed_stream);
    
        // Use the ZlibDecoder to copy the decompressed data to the Vec.
        decoder
            .read_to_end(&mut decompressed_data)?;
    
        // Return the decompressed data as a Vec<u8>.
        return Ok(decompressed_data);
    }

}