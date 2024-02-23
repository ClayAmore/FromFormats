use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use crate::util::oodle::*;

use std::io::Error;
use libc::{c_ulong, c_int, c_uint};

#[derive(WrapperApi)]
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

pub struct Oodle28 {
    container: Container<OodleApi>
}

impl OodleCompressor for Oodle28 {
    fn decompress(&mut self, source: &[u8], uncompressed_size: usize) -> Result<Vec<u8>, Error> {
        todo!()
    }
}

impl Oodle28 {
    pub fn new() -> Self {
        let mut cont: Container<OodleApi> =
        unsafe { Container::load("oo2core_8_win64.dll") }.expect("Could not open oo2core_8_win64 or load symbols");

        Self {
            container: cont
        }
        
    }
}