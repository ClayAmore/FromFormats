use crate::binary_reader::BinaryReader;
use crate::sf_util::SFUtil;

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

        let b0 = br.get_byte(0);
        let b1 = br.get_byte(1);
        let magic = br.get_ascii(0, 4).expect("Error!");
        return magic == "DCP\0"
            || magic == "DCX\0"
            || b0 == 0x78 && (b1 == 0x01 || b1 == 0x5E || b1 == 0x9C || b1 == 0xDA);
    }

    pub(crate) fn decompress(br: &mut BinaryReader, compression: &mut CompressionType) -> Vec<u8> {
        br.big_endian = true;
        *compression = CompressionType::Unknown;

        let magic = br.get_ascii(0, 4).expect("Error!");

        if magic == "DCP\0" {
            let format = br.get_ascii(4, 4).expect("Error. Couldn't get ascii(4,4)");

            if format == "DFLT" {
                *compression = CompressionType::DCP_DFLT;
            } else if format == "EDGE" {
                *compression = CompressionType::DCP_EDGE;
            }
        } else if magic == "DCX\0" {
            let format = br
                .get_ascii(0x28, 4)
                .expect("Error. Couldn't get ascii(4,4)");
            if format == "EDGE" {
                *compression = CompressionType::DCX_EDGE;
            } else if format == "DFLT" {
                let unk04 = br.get_int32(0x4);
                let unk10 = br.get_int32(0x10);
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

            br.set_position(0);
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
                    return DCX::decompress_dcx_krak(br);
                }
                _ => {
                    panic!("Unknown DCX format.");
                }
            }
        }

        return Vec::new();
    }

    pub(crate) fn decompress_dcp_edge(br: &mut BinaryReader) -> Vec<u8> {
        todo!()
    }

    pub(crate) fn decompress_dcp_dflt(br: &mut BinaryReader) -> Vec<u8> {
        todo!()
    }

    pub(crate) fn decompress_dcx_edge(br: &mut BinaryReader) -> Vec<u8> {
        todo!()
    }

    pub(crate) fn decompress_dcx_dflt(br: &mut BinaryReader, compression: &mut CompressionType) -> Vec<u8> {
        todo!()
    }

    pub(crate) fn decompress_dcx_krak(br: &mut BinaryReader) -> Vec<u8> {
        todo!()
    }
}
