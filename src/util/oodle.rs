use std::env;
use std::io::Error;
use crate::util::oodle26::Oodle26;
use crate::util::oodle28::Oodle28;


pub trait OodleCompressor {
    fn decompress(&mut self, source: &[u8], uncompressed_size: usize) -> Result<Vec<u8>, Error>;
}

static mut  OODLE6_EXISTS: bool = false;
static mut  OODLE8_EXISTS: bool = false;

pub struct Oodle {
    
}

impl Oodle {
    pub fn get_oodle_compressor(compression_level: i32) -> Result<Box<dyn OodleCompressor>, Error> {
        
        if compression_level != -1 {

            if compression_level == 9 {

                if can_use_oodle8() {
                    return Ok(Box::new(Oodle28::new()));
                }
                
                if can_use_oodle6() {
                    return Ok(Box::new(Oodle26::new()));
                }

            }
            else if compression_level == 6 {

                if can_use_oodle6() {
                    return Ok(Box::new(Oodle26::new()));
                }
                
                if can_use_oodle8() {
                    return Ok(Box::new(Oodle28::new()));
                }
            }
        }
        else {
            if can_use_oodle6() {
                return Ok(Box::new(Oodle26::new()));
            }
            if can_use_oodle8() {
                return Ok(Box::new(Oodle28::new()));
            }
        }
        
        return Err(Error::new(std::io::ErrorKind::NotFound, "Could not find a supported version of oo2core.\n \
            Please copy oo2core_6_win64.dll or oo2core_8_win64.dll into the program directory"))
    }
}

fn can_use_oodle6() -> bool {
    unsafe {
        if OODLE6_EXISTS  {
            return true;
        }
        
        if let Ok(mut current_dir) = env::current_exe() {
            current_dir.pop();
            let current_dir_string = current_dir.to_str();
            println!("{:?}",current_dir_string);
            let oodle6 = current_dir.join("oo2core_6_win64.dll");
            if oodle6.exists()  {
                OODLE6_EXISTS = true;
                return true;
            }
        }
        return false;
    }
}

fn can_use_oodle8() -> bool {
    unsafe {
        if OODLE8_EXISTS {
            return true;
        }
        
        if let Ok(mut current_dir) = env::current_exe() {
            current_dir.pop();
            let oodle8 = current_dir.join("oo2core_8_win64.dll");
            if oodle8.exists()  {
                OODLE8_EXISTS = true;
                return true;
            }
        }
        return false;
    }
}


#[repr(u32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum FuzzSafe {
    OodleLZ_FuzzSafe_No = 0,
    OodleLZ_FuzzSafe_Yes = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub(crate) enum OodleLZ_CompressionLevel {
    OodleLZ_CompressionLevel_None = 0,
    OodleLZ_CompressionLevel_SuperFast = 1,
    OodleLZ_CompressionLevel_VeryFast = 2,
    OodleLZ_CompressionLevel_Fast = 3,
    OodleLZ_CompressionLevel_Normal = 4,

    OodleLZ_CompressionLevel_Optimal1 = 5,
    OodleLZ_CompressionLevel_Optimal2 = 6,
    OodleLZ_CompressionLevel_Optimal3 = 7,
    OodleLZ_CompressionLevel_Optimal4 = 8,
    OodleLZ_CompressionLevel_Optimal5 = 9,

    OodleLZ_CompressionLevel_HyperFast1 = -1,
    OodleLZ_CompressionLevel_HyperFast2 = -2,
    OodleLZ_CompressionLevel_HyperFast3 = -3,
    OodleLZ_CompressionLevel_HyperFast4 = -4,

    OodleLZ_CompressionLevel_Force32 = 0x40000000,
    OodleLZ_CompressionLevel_Invalid,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub(crate) enum OodleLZ_Compressor {
    OodleLZ_Compressor_Invalid = -1,
    OodleLZ_Compressor_None = 3,

    OodleLZ_Compressor_Kraken = 8,
    OodleLZ_Compressor_Leviathan = 13,
    OodleLZ_Compressor_Mermaid = 9,
    OodleLZ_Compressor_Selkie = 11,
    OodleLZ_Compressor_Hydra = 12,

    OodleLZ_Compressor_BitKnit = 10,
    OodleLZ_Compressor_LZB16 = 4,
    OodleLZ_Compressor_LZNA = 7,
    OodleLZ_Compressor_LZH = 0,
    OodleLZ_Compressor_LZHLW = 1,
    OodleLZ_Compressor_LZNIB = 2,
    OodleLZ_Compressor_LZBLW = 5,
    OodleLZ_Compressor_LZA = 6,

    OodleLZ_Compressor_Count = 14,
    OodleLZ_Compressor_Force32 = 0x40000000,
}

#[repr(u32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum OodleLZ_CheckCRC {
    OodleLZ_CheckCRC_No = 0,
    OodleLZ_CheckCRC_Yes = 1,
    OodleLZ_CheckCRC_Force32 = 0x40000000,
}

#[repr(i32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum OodleLZ_Decode_ThreadPhase {
    OodleLZ_Decode_ThreadPhase1 = 1,
    OodleLZ_Decode_ThreadPhase2 = 2,
    OodleLZ_Decode_ThreadPhaseAll = 3,
}

#[repr(u32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum OodleLZ_FuzzSafe {
    OodleLZ_FuzzSafe_No = 0,
    OodleLZ_FuzzSafe_Yes = 1,
}

#[repr(u32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum OodleLZ_Profile {
    OodleLZ_Profile_Main = 0,
    OodleLZ_Profile_Reduced = 1,
    OodleLZ_Profile_Force32 = 0x40000000,
}

#[repr(u32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum OodleLZ_Verbosity {
    OodleLZ_Verbosity_None = 0,
    OodleLZ_Verbosity_Minimal = 1,
    OodleLZ_Verbosity_Some = 2,
    OodleLZ_Verbosity_Lots = 3,
    OodleLZ_Verbosity_Force32 = 0x40000000,
}