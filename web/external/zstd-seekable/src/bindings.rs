pub use libc::{c_char, c_int, c_uint, c_ulonglong, c_void, size_t};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_seekable_CStream {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_CStream {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_frameLog {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_seekable {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_inBuffer {
    pub src: *const c_void,
    pub size: size_t,
    pub pos: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_outBuffer {
    pub dst: *mut c_void,
    pub size: size_t,
    pub pos: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_DStream {
    _unused: [u8; 0],
}

pub type ZSTD_seekable_read = ::std::option::Option<
    unsafe extern "C" fn(
        opaque: *mut ::std::os::raw::c_void,
        buffer: *mut ::std::os::raw::c_void,
        n: size_t,
    ) -> ::std::os::raw::c_int,
>;
pub type ZSTD_seekable_seek = ::std::option::Option<
    unsafe extern "C" fn(
        opaque: *mut ::std::os::raw::c_void,
        offset: ::std::os::raw::c_longlong,
        origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_seekable_customFile {
    pub opaque: *mut ::std::os::raw::c_void,
    pub read: ZSTD_seekable_read,
    pub seek: ZSTD_seekable_seek,
}

pub type ZSTD_EndDirective = c_uint;

extern "C" {
    pub fn ZSTD_versionString() -> *const c_char;

    pub fn ZSTD_compress(
        dst: *mut c_void,
        dstCapacity: size_t,
        src: *const c_void,
        srcSize: size_t,
        compressionLevel: c_int,
    ) -> size_t;
    pub fn ZSTD_isError(code: size_t) -> c_uint;
    pub fn ZSTD_getErrorName(code: size_t) -> *const c_char;
    pub fn ZSTD_DStreamOutSize() -> size_t;
    pub fn ZSTD_createCStream() -> *mut ZSTD_CStream;
    pub fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t;
    pub fn ZSTD_seekable_createCStream() -> *mut ZSTD_seekable_CStream;
    pub fn ZSTD_seekable_freeCStream(zcs: *mut ZSTD_seekable_CStream) -> size_t;
    pub fn ZSTD_seekable_initCStream(
        zcs: *mut ZSTD_seekable_CStream,
        compressionLevel: c_int,
        checksumFlag: c_int,
        maxFrameSize: c_uint,
    ) -> size_t;
    pub fn ZSTD_seekable_compressStream(
        zcs: *mut ZSTD_seekable_CStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;
    pub fn ZSTD_seekable_endStream(
        zcs: *mut ZSTD_seekable_CStream,
        output: *mut ZSTD_outBuffer,
    ) -> size_t;
    pub fn ZSTD_seekable_createFrameLog(checksumFlag: c_int) -> *mut ZSTD_frameLog;
    pub fn ZSTD_seekable_freeFrameLog(fl: *mut ZSTD_frameLog) -> size_t;
    pub fn ZSTD_seekable_logFrame(
        fl: *mut ZSTD_frameLog,
        compressedSize: c_uint,
        decompressedSize: c_uint,
        checksum: c_uint,
    ) -> size_t;
    pub fn ZSTD_seekable_writeSeekTable(
        fl: *mut ZSTD_frameLog,
        output: *mut ZSTD_outBuffer,
    ) -> size_t;
    pub fn ZSTD_seekable_create() -> *mut ZSTD_seekable;
    pub fn ZSTD_seekable_free(zs: *mut ZSTD_seekable) -> size_t;
    pub fn ZSTD_seekable_initBuff(
        zs: *mut ZSTD_seekable,
        src: *const c_void,
        srcSize: size_t,
    ) -> size_t;
    pub fn ZSTD_seekable_initFile(zs: *mut ZSTD_seekable, src: *mut libc::FILE) -> size_t;
    pub fn ZSTD_seekable_decompress(
        zs: *mut ZSTD_seekable,
        dst: *mut c_void,
        dstSize: size_t,
        offset: c_ulonglong,
    ) -> size_t;
    pub fn ZSTD_seekable_decompressFrame(
        zs: *mut ZSTD_seekable,
        dst: *mut c_void,
        dstSize: size_t,
        frameIndex: c_uint,
    ) -> size_t;
    pub fn ZSTD_seekable_getNumFrames(zs: *mut ZSTD_seekable) -> c_uint;
    pub fn ZSTD_seekable_getFrameCompressedOffset(
        zs: *mut ZSTD_seekable,
        frameIndex: c_uint,
    ) -> c_ulonglong;
    pub fn ZSTD_seekable_getFrameDecompressedOffset(
        zs: *mut ZSTD_seekable,
        frameIndex: c_uint,
    ) -> c_ulonglong;
    pub fn ZSTD_seekable_getFrameCompressedSize(
        zs: *mut ZSTD_seekable,
        frameIndex: c_uint,
    ) -> size_t;
    pub fn ZSTD_seekable_getFrameDecompressedSize(
        zs: *mut ZSTD_seekable,
        frameIndex: c_uint,
    ) -> size_t;
    pub fn ZSTD_seekable_offsetToFrameIndex(zs: *mut ZSTD_seekable, offset: c_ulonglong) -> c_uint;
    pub fn ZSTD_compressStream2(
        cctx: *mut ZSTD_CStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
        endOp: ZSTD_EndDirective,
    ) -> size_t;
    pub fn ZSTD_CStreamInSize() -> size_t;
    pub fn ZSTD_CStreamOutSize() -> size_t;
    pub fn ZSTD_compressStream(
        zcs: *mut ZSTD_CStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;
    pub fn ZSTD_flushStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer) -> size_t;
    pub fn ZSTD_endStream(zcs: *mut ZSTD_CStream, output: *mut ZSTD_outBuffer) -> size_t;

    pub fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    pub fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
    pub fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> size_t;
    pub fn ZSTD_decompressStream(
        zds: *mut ZSTD_DStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;
    pub fn ZSTD_seekable_initAdvanced(
        zs: *mut ZSTD_seekable,
        src: ZSTD_seekable_customFile,
    ) -> size_t;
    pub fn ZSTD_initCStream(zcs: *mut ZSTD_CStream, compressionLevel: c_int) -> size_t;
}
/*
#[doc = "  Explicit context"]
pub type ZSTD_CCtx = ZSTD_CCtx_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZSTD_DCtx_s {
    _unused: [u8; 0],
}
pub type ZSTD_DCtx = ZSTD_DCtx_s;
#[doc = "  Streaming"]
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type ZSTD_CStream = ZSTD_CCtx;
extern "C" {
}
pub type ZSTD_DStream = ZSTD_DCtx;
extern "C" {
}
extern "C" {
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub type ZSTD_seekable = ZSTD_seekable_s;
extern "C" {
}
extern "C" {
}
extern "C" {
}
*/
