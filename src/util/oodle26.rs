use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use crate::util::oodle::*;

use std::io::Error;
use libc::{c_ulong, c_int, c_uint};

#[derive(WrapperApi)]
#[allow(non_snake_case)]
struct OodleApi {
    OodleLZ_Decompress: unsafe extern "C" fn(
        compBuf: *const u8,
        compBufSize: usize,
        rawBuf: *mut u8,
        rawLen: usize,
        fuzzSafe: FuzzSafe,
        checkCRC: OodleLZ_CheckCRC,
        verbosity: OodleLZ_Verbosity,
        decBufBase: usize,
        decBufSize: usize,
        fpCallback: usize,
        callbackUserData: usize,
        decoderMemory: usize,
        decoderMemorySize: usize,
        threadPhase: OodleLZ_Decode_ThreadPhase,
    ) -> usize,

    OodleLZ_GetDecodeBufferSize: unsafe extern "C" fn(rawSize: c_ulong, corruptionPossible: c_int ) -> c_uint,
}

pub struct Oodle26 {
    container: Container<OodleApi>
}

impl OodleCompressor for Oodle26 {
    fn decompress(&mut self, source: &[u8], uncompressed_size: usize) -> Result<Vec<u8>, Error> {
        
        let decoded_buffer_size = unsafe {
            self.container.OodleLZ_GetDecodeBufferSize(uncompressed_size as c_ulong, true as c_int)
        };

        // Allocate a destination buffer
        let mut rawBuf: Vec<u8> = vec![0; decoded_buffer_size as usize];

        // Decompress the data
        let result = unsafe {
            self.container.OodleLZ_Decompress(
                source.as_ptr(),
                source.len(),
                rawBuf.as_mut_ptr(),
                uncompressed_size ,
                FuzzSafe::OodleLZ_FuzzSafe_Yes,
                OodleLZ_CheckCRC::OodleLZ_CheckCRC_No ,
                OodleLZ_Verbosity::OodleLZ_Verbosity_None ,
                0,
                0 ,
                0,
                0,
                0,
                0 ,
                OodleLZ_Decode_ThreadPhase::OodleLZ_Decode_ThreadPhaseAll ,
            )
        };

        Ok(rawBuf)
    }
}

impl Oodle26 {
    pub fn new() -> Self {
        let mut cont: Container<OodleApi> =
        unsafe { Container::load("oo2core_6_win64.dll") }.expect("Could not open oo2core_6_win64 or load symbols");

        Self {
            container: cont
        }
        
    }
}