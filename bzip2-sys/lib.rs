extern crate libc;

use libc::{c_int, c_uint, c_void, c_char};

pub const BZ_RUN: c_int = 0;
pub const BZ_FLUSH: c_int = 1;
pub const BZ_FINISH: c_int = 2;

pub const BZ_OK: c_int = 0;
pub const BZ_RUN_OK: c_int = 1;
pub const BZ_FLUSH_OK: c_int = 2;
pub const BZ_FINISH_OK: c_int = 3;
pub const BZ_STREAM_END: c_int = 4;
pub const BZ_SEQUENCE_ERROR: c_int = -1;
pub const BZ_PARAM_ERROR: c_int = -2;
pub const BZ_MEM_ERROR: c_int = -3;
pub const BZ_DATA_ERROR: c_int = -4;
pub const BZ_DATA_ERROR_MAGIC: c_int = -5;
pub const BZ_IO_ERROR: c_int = -6;
pub const BZ_UNEXPECTED_EOF: c_int = -7;
pub const BZ_OUTBUFF_FULL: c_int = -8;
pub const BZ_CONFIG_ERROR: c_int = -9;

#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct bz_stream {
    pub next_in: *mut c_char,
    pub avail_in: c_uint,
    pub total_in_lo32: c_uint,
    pub total_in_hi32: c_uint,

    pub next_out: *mut c_char,
    pub avail_out: c_uint,
    pub total_out_lo32: c_uint,
    pub total_out_hi32: c_uint,

    pub state: *mut c_void,

    pub bzalloc: Option<extern fn(*mut c_void, c_int, c_int) -> *mut c_void>,
    pub bzfree: Option<extern fn(*mut c_void, *mut c_void)>,
    pub opaque: *mut c_void,
}

extern {
    pub fn BZ2_bzCompressInit(stream: *mut bz_stream,
                              blockSize100k: c_int,
                              verbosity: c_int,
                              workFactor: c_int) -> c_int;
    pub fn BZ2_bzCompress(stream: *mut bz_stream, action: c_int) -> c_int;
    pub fn BZ2_bzCompressEnd(stream: *mut bz_stream) -> c_int;
    pub fn BZ2_bzDecompressInit(stream: *mut bz_stream,
                                verbosity: c_int,
                                small: c_int) -> c_int;
    pub fn BZ2_bzDecompress(stream: *mut bz_stream) -> c_int;
    pub fn BZ2_bzDecompressEnd(stream: *mut bz_stream) -> c_int;
}

#[no_mangle]
pub fn bz_internal_error(errcode: c_int) {
    panic!("bz internal error: {}", errcode);
}
