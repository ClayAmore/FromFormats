use crate::util::binary_reader::BinaryReader;
use crate::util::sf_util::SFUtil;
use crate::util::oodle::Oodle;
use std::io::{Error, ErrorKind, Cursor, Write};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CompressionType {
    Unknown,
    None,
    Zlib,
    DCP_EDGE,
    DCP_DFLT,
    DCX_EDGE,
    DCX_DFLT_10000_24_9,
    DCX_DFLT_10000_44_9,
    DCX_DFLT_11000_44_8,
    DCX_DFLT_11000_44_9,
    DCX_DFLT_11000_44_9_15,
    DCX_KRAK,
}

pub(crate) struct DCX {}

impl DCX {
    pub(crate) fn is(br: &mut BinaryReader) -> bool {
        if br.len() < 4 {
            return false;
        }

        let magic = br.get_ascii(0, 4).expect("Error!");
        return magic == "DCP\0" || magic == "DCX\0";
    }

    pub(crate) fn decompress(br: &mut BinaryReader, compression: &mut CompressionType) -> Result<Vec<u8>, Error> {
        br.big_endian = true;
        *compression = CompressionType::Unknown;

        let magic = br.get_ascii(0, 4)?;

        if magic == "DCP\0" {
            let format = br.get_ascii(4, 4)?;

            if format == "DFLT" {
                *compression = CompressionType::DCP_DFLT;
            } else if format == "EDGE" {
                *compression = CompressionType::DCP_EDGE;
            }
        } else if magic == "DCX\0" {
            let format = br
                .get_ascii(0x28, 4)?;
            if format == "EDGE" {
                *compression = CompressionType::DCX_EDGE;
            } else if format == "DFLT" {
                let unk04 = br.get_i32(0x4);
                let unk10 = br.get_i32(0x10);
                let unk30 = br.get_byte(0x30);
                let unk38 = br.get_byte(0x38);

                if unk04 == 0x10000 && unk10 == 0x24 && unk30 == 9 && unk38 == 0 {
                    *compression = CompressionType::DCX_DFLT_10000_24_9;
                } else if unk04 == 0x10000 && unk10 == 0x44 && unk30 == 9 && unk38 == 0 {
                    *compression = CompressionType::DCX_DFLT_10000_44_9;
                } else if unk04 == 0x11000 && unk10 == 0x44 && unk30 == 8 && unk38 == 0 {
                    *compression = CompressionType::DCX_DFLT_11000_44_8;
                } else if unk04 == 0x11000 && unk10 == 0x44 && unk30 == 9 && unk38 == 0 {
                    *compression = CompressionType::DCX_DFLT_11000_44_9;
                } else if unk04 == 0x11000 && unk10 == 0x44 && unk30 == 9 && unk38 == 15 {
                    *compression = CompressionType::DCX_DFLT_11000_44_9_15;
                }
            }
            else if format == "KRAK" {
                *compression = CompressionType::DCX_KRAK;
            }
            else {
                let b0 = br.get_byte(0);
                let b1 = br.get_byte(1);

                if b0 == 0x78 && (b1 == 0x01 || b1 == 0x5E || b1 == 0x9C || b1 == 0xDA)
                {
                    *compression = CompressionType::Zlib;
                }
            }

            br.position = 0;
            match *compression {
                CompressionType::Zlib => {
                    let compression_size = br.len();
                    return SFUtil::read_zlib(br, compression_size);
                }
                CompressionType::DCP_EDGE => {
                    return DCX::decompress_dcp_edge(br);
                }
                CompressionType::DCP_DFLT => {
                    return DCX::decompress_dcp_dflt(br);
                }
                CompressionType::DCX_EDGE => {
                    return DCX::decompress_dcx_edge(br);
                }
                CompressionType::DCX_DFLT_10000_24_9
                | CompressionType::DCX_DFLT_10000_44_9
                | CompressionType::DCX_DFLT_11000_44_8
                | CompressionType::DCX_DFLT_11000_44_9
                | CompressionType::DCX_DFLT_11000_44_9_15 => {
                    return DCX::decompress_dcx_dflt(br, compression);
                }
                CompressionType::DCX_KRAK => {
                    return DCX::decompress_dcx_krak(br, None);
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Unknown DCX format.",
                    ));
                }
            }
        }

        return Ok(Vec::new());
    }
    
    fn decompress_dcp_edge(br: &mut BinaryReader) -> Result<Vec<u8>, Error> {
        println!("decompress_dcp_edge");
        br.assert_ascii(&["DCP\0"])?;
        br.assert_ascii(&["EDGE"])?;
        br.assert_i32(&[0x20]);
        br.assert_i32(&[0x9000000]);
        br.assert_i32(&[0x10000]);
        br.assert_i32(&[0x0]);
        br.assert_i32(&[0x0]);
        br.assert_i32(&[0x00100100]);

        br.assert_ascii(&["DCS\0"])?;

        let uncompressed_size = br.read_i32();
        let compressed_size = br.read_i32();
        br.assert_i32(&[0]);
        let data_start = br.position;
        br.skip(compressed_size as usize);

        br.assert_ascii(&["DCA\0"])?;

        let dca_size = br.read_i32(); // Todo: check if unused variable
        br.assert_ascii(&["EgdT"])?;
        br.assert_i32(&[0x00010000]);
        br.assert_i32(&[0x20]);
        br.assert_i32(&[0x10]);
        br.assert_i32(&[0x10000]);
        let egdt_size = br.read_i32();
        let chunk_count = br.read_i32();
        br.assert_i32(&[0x100000]);

        if egdt_size != 0x20 + chunk_count * 0x10 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unexpected EgdT size in EDGE DCX.",
            ));
        }

        // Create a vector to store the decompressed data with the size of uncompressed_size
        let decompressed = vec![0u8; uncompressed_size as usize];

        // Create a cursor to write the decompressed data
        let mut dcmp_stream: Cursor<Vec<u8>> = Cursor::new(decompressed);

        // Loop through each data chunk
        for _ in 0..chunk_count {
            // Verify that the next 4 bytes are all zeros
            br.assert_i32(&[0]);

            // Read the offset (4 bytes) from the binary reader
            let offset = br.read_i32();

            // Read the size (4 bytes) from the binary reader
            let size = br.read_i32();

            // Check if the chunk is compressed (next 4 bytes should be 1 if compressed)
            let compressed = br.assert_i32(&[0, 1]) == 1;

            // Get the chunk data based on the offset and size
            let chunk = br.get_bytes(data_start + offset as usize, size as usize);

            // If the chunk is compressed, decompress it
            if compressed {
                // Create a decompressor for zlib (flate2) with "raw" format
                let mut decompresser = flate2::Decompress::new(false);

                // Create a temporary vector to store the decompressed data
                let mut temp_stream: Vec<u8> = Vec::new();

                // Decompress the chunk data and store it in temp_stream
                decompresser.decompress_vec(&chunk, &mut temp_stream, flate2::FlushDecompress::None)?;

                // Write the decompressed data to the dcmp_stream
                dcmp_stream.write_all(&temp_stream)?;
            } else {
                // If the chunk is not compressed, write it directly to the dcmp_stream
                dcmp_stream.write_all(&chunk)?;
            }
        }

        // Return the decompressed data as a result of the function
        Ok(dcmp_stream.into_inner())
    }

    fn decompress_dcp_dflt(br: &mut BinaryReader) -> Result<Vec<u8>, Error> {
        println!("decompress_dcp_dflt");
        br.assert_ascii(&["DCP\0"])?;
        br.assert_ascii(&["DFLT"])?;
        br.assert_i32(&[0x20]);
        br.assert_i32(&[0x9000000]);
        br.assert_i32(&[0]);
        br.assert_i32(&[0]);
        br.assert_i32(&[0]);
        br.assert_i32(&[0x00010100]);
        
        br.assert_ascii(&["DCS\0"])?;
        let uncompressed_size = br.read_i32();  // Todo: check if unused variable
        let compressed_size = br.read_i32();

        let decompressed: Vec<u8> = SFUtil::read_zlib(br, compressed_size as usize)?;


        br.assert_ascii(&["DCA\0"])?;
        br.assert_i32(&[8]);

        return Ok(decompressed);
    }

    fn decompress_dcx_edge(br: &mut BinaryReader) -> Result<Vec<u8>, Error> {
        println!("decompress_dcx_edge");
        br.assert_ascii(&["DCX\0"])?;
        br.assert_i32(&[0x10000]);
        br.assert_i32(&[0x18]);
        br.assert_i32(&[0x24]);
        br.assert_i32(&[0x24]);
        let unk1 = br.read_i32();

        br.assert_ascii(&["DCS\0"])?;
        let uncompressed_size = br.read_i32();
        let compressed_size = br.read_i32();

        br.assert_ascii(&["DCP\0"])?;
        br.assert_ascii(&["EDGE"])?;
        br.assert_i32(&[0x20]);
        br.assert_i32(&[0x9000000]);
        br.assert_i32(&[0x10000]);
        br.assert_i32(&[0x0]);
        br.assert_i32(&[0x0]);
        br.assert_i32(&[0x00100100]);

        let dca_start = br.position;
        br.assert_ascii(&["DCA\0"])?;
        let dca_size = br.read_i32();
        br.assert_ascii(&["EgdT"])?;
        br.assert_i32(&[0x00010100]);
        br.assert_i32(&[0x24]);
        br.assert_i32(&[0x10]);
        br.assert_i32(&[0x10000]);

        let trailing_uncompressed_size = br.assert_i32(&[uncompressed_size % 0x10000, 0x10000]);
        let egdt_size = br.read_i32();
        let chunk_count = br.read_i32();
        br.assert_i32(&[0x100000]);

        if unk1 != 0x50 + chunk_count * 0x10 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unexpected unk1 value in EDGE DCX."));
        }

        if egdt_size != 0x24 + chunk_count * 0x10 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unexpected EgdT size in EDGE DCX."));
        }

        // Create a vector to store the decompressed data with the size of uncompressed_size
        let decompressed = vec![0u8; uncompressed_size as usize];

        // Create a cursor to write the decompressed data
        let mut dcmp_stream: Cursor<Vec<u8>> = Cursor::new(decompressed);

        // Loop through each data chunk
        for _ in 0..chunk_count {
            // Verify that the next 4 bytes are all zeros
            br.assert_i32(&[0]);

            // Read the offset (4 bytes) from the binary reader
            let offset = br.read_i32();

            // Read the size (4 bytes) from the binary reader
            let size = br.read_i32();

            // Check if the chunk is compressed (next 4 bytes should be 1 if compressed)
            let compressed = br.assert_i32(&[0, 1]) == 1;

            // Get the chunk data based on the offset and size
            let chunk = br.get_bytes(dca_start + dca_size as usize + offset as usize, size as usize);

            // If the chunk is compressed, decompress it
            if compressed {
                // Create a decompressor for zlib (flate2) with "raw" format
                let mut decompresser = flate2::Decompress::new(false);

                // Create a temporary vector to store the decompressed data
                let mut temp_stream: Vec<u8> = Vec::new();

                // Decompress the chunk data and store it in temp_stream
                decompresser.decompress_vec(&chunk, &mut temp_stream, flate2::FlushDecompress::None)?;

                // Write the decompressed data to the dcmp_stream
                dcmp_stream.write_all(&temp_stream)?;
            } else {
                // If the chunk is not compressed, write it directly to the dcmp_stream
                dcmp_stream.write_all(&chunk)?;
            }
        }

        // Return the decompressed data as a result of the function
        Ok(dcmp_stream.into_inner())

    }

    fn decompress_dcx_dflt(br: &mut BinaryReader, compression: &mut CompressionType) -> Result<Vec<u8>, Error> {
        println!("decompress_dcx_dflt");
        let unk04 = if *compression == CompressionType::DCX_DFLT_10000_24_9 || *compression == CompressionType::DCX_DFLT_11000_44_9 {  0x10000  } else { 0x11000 };
        let unk10 = if *compression == CompressionType::DCX_DFLT_10000_24_9 { 0x24 } else { 0x44 };
        let unk14 = if *compression == CompressionType::DCX_DFLT_10000_24_9 { 0x2C } else { 0x4C };
        let unk30 = if *compression == CompressionType::DCX_DFLT_11000_44_8 { 8 as u8 } else { 9 as u8 };
        let unk38 = if *compression == CompressionType::DCX_DFLT_11000_44_9_15 { 15 as u8 } else { 0 as u8 };

        br.assert_ascii(&["DCX\0"])?;
        br.assert_i32(&[unk04]);
        br.assert_i32(&[0x18]);
        br.assert_i32(&[0x24]);
        br.assert_i32(&[unk10]);
        br.assert_i32(&[unk14]);

        br.assert_ascii(&["DCS\0"])?;
        let uncompressed_size = br.read_i32();
        let compressed_size = br.read_i32();

        br.assert_ascii(&["DCP\0"])?;
        br.assert_ascii(&["DFLT"])?;
        br.assert_i32(&[0x20]);
        br.assert_byte(&[unk30]);
        br.assert_byte(&[0]);
        br.assert_byte(&[0]);
        br.assert_byte(&[0]);
        br.assert_i32(&[0x0]);
        br.assert_byte(&[unk38]);
        br.assert_byte(&[0]);
        br.assert_byte(&[0]);
        br.assert_byte(&[0]);
        br.assert_i32(&[0x0]);
        br.assert_i32(&[0x00010100]);

        br.assert_ascii(&["DCA\0"])?;

        let compressed_header_length = br.read_i32();

        return SFUtil::read_zlib(br, compressed_size as usize);

    }

    fn decompress_dcx_krak(br: &mut BinaryReader, compression_level: Option<u8>) -> Result<Vec<u8>, Error> {

        // Default value for compression_level 6 if no value specified in the params
        let compression_level_result = match compression_level {
            Some(cl) => cl,
            None => 6
        };

        println!("decompress_dcx_krak");
        br.assert_ascii(&["DCX\0"])?;
        br.assert_i32(&[0x11000]);
        br.assert_i32(&[0x18]);
        br.assert_i32(&[0x24]);
        br.assert_i32(&[0x44]);
        br.assert_i32(&[0x4C]);
        br.assert_ascii(&["DCS\0"])?;
        let uncompressed_size = br.read_i32();
        let compressed_size = br.read_i32();
        br.assert_ascii(&["DCP\0"])?;
        br.assert_ascii(&["KRAK"])?;
        br.assert_i32(&[0x20]);
        br.assert_byte(&[compression_level_result]);
        br.assert_byte(&[0]);
        br.assert_byte(&[0]);
        br.assert_byte(&[0]);
        br.assert_i32(&[0]);
        br.assert_i32(&[0]);
        br.assert_i32(&[0]);
        br.assert_i32(&[0x10100]);
        br.assert_ascii(&["DCA\0"])?;
        br.assert_i32(&[8]);

        
        let compressed: &[u8] = br.read_span_view(compressed_size as usize)?;
        let mut compressor = Oodle::get_oodle_compressor(compression_level_result as i32)?;
        return compressor.decompress(compressed, uncompressed_size as usize);

    }
}
