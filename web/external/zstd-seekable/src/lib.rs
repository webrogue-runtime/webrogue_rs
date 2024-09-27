#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[macro_use]
extern crate thiserror;
mod bindings;
use bindings::*;

#[cfg(any(not(target_family = "wasm"), target_os = "emscripten"))]
use libc::*;
#[cfg(any(not(target_family = "wasm"), target_os = "emscripten"))]
use std::ffi::CString;
use std::ffi::{c_void, CStr};

#[cfg(target_family = "wasm")]
type c_longlong = i64;

pub fn version() -> &'static CStr {
    unsafe { CStr::from_ptr(ZSTD_versionString()) }
}

pub fn out_size() -> size_t {
    unsafe { ZSTD_DStreamOutSize() }
}

/// The type of seekable compressors.
pub struct SeekableCStream {
    p: *mut ZSTD_seekable_CStream,
}

/// The type of compressors.
pub struct CStream {
    p: *mut ZSTD_CStream,
}

/// The type of compressors.
pub struct FrameLog {
    p: *mut ZSTD_frameLog,
}

/// The type of decompressors.
#[cfg(not(target_family = "wasm"))]
pub struct Seekable<'a, R> {
    p: *mut ZSTD_seekable,
    b: *mut R,
    f: *mut libc::FILE,
    marker: std::marker::PhantomData<&'a R>,
}

#[cfg(target_family = "wasm")]
pub struct Seekable<'a, R> {
    p: *mut ZSTD_seekable,
    b: *mut R,
    f: *mut std::ffi::c_void,
    marker: std::marker::PhantomData<&'a R>,
}

unsafe impl<R> Send for Seekable<'static, R> {}
unsafe impl Send for SeekableCStream {}
unsafe impl Send for FrameLog {}
unsafe impl<D: Dst> Send for CompressedFrame<D> {}

#[derive(Error)]
pub enum Error {
    Null,
    CouldNotOpenFile(String),
    ZSTD(size_t),
    Io(#[from] std::io::Error),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Null => write!(fmt, "Null"),
            Error::Io(ref e) => e.fmt(fmt),
            Error::CouldNotOpenFile(ref f) => write!(fmt, "Could not open {}", f),
            Error::ZSTD(r) => unsafe {
                let error = CStr::from_ptr(ZSTD_getErrorName(r));
                write!(fmt, "ZSTD({:?})", error)
            },
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl Drop for SeekableCStream {
    fn drop(&mut self) {
        unsafe {
            if !self.p.is_null() {
                ZSTD_seekable_freeCStream(self.p);
                self.p = std::ptr::null_mut();
            }
        }
    }
}

impl Drop for CStream {
    fn drop(&mut self) {
        unsafe {
            if !self.p.is_null() {
                ZSTD_freeCStream(self.p);
                self.p = std::ptr::null_mut();
            }
        }
    }
}

impl<'a, R> Drop for Seekable<'a, R> {
    fn drop(&mut self) {
        unsafe {
            if !self.p.is_null() {
                ZSTD_seekable_free(self.p);
                self.p = std::ptr::null_mut();
            }
            if !self.f.is_null() {
                #[cfg(target_family = "wasm")]
                fn fclose(_: *mut c_void) {}
                fclose(self.f);
                self.f = std::ptr::null_mut();
            }
            if !self.b.is_null() {
                let b: Box<R> = Box::from_raw(self.b);
                std::mem::drop(b);
                self.b = std::ptr::null_mut();
            }
        }
    }
}

impl Drop for FrameLog {
    fn drop(&mut self) {
        unsafe {
            if !self.p.is_null() {
                ZSTD_seekable_freeFrameLog(self.p);
                self.p = std::ptr::null_mut()
            }
        }
    }
}

unsafe extern "C" fn zstd_seekable_read<R: std::io::Read + std::io::Seek>(
    opaque: *mut c_void,
    buffer: *mut c_void,
    n: size_t,
) -> c_int {
    let mut b: Box<R> = Box::from_raw(opaque as *mut R);
    let s = std::slice::from_raw_parts_mut(buffer as *mut u8, n as usize);
    let result = if b.read_exact(s).is_ok() { 0 } else { 1 };
    std::mem::forget(b);
    result
}

#[cfg(target_family = "wasm")]
const SEEK_SET: c_int = 0;
#[cfg(target_family = "wasm")]
const SEEK_CUR: c_int = 1;
// #[cfg(target_family = "wasm")]
// const SEEK_END: c_int = 2;

unsafe extern "C" fn zstd_seekable_seek<R: std::io::Read + std::io::Seek>(
    opaque: *mut c_void,
    offset: c_longlong,
    origin: c_int,
) -> c_int {
    let mut b: Box<R> = Box::from_raw(opaque as *mut R);
    use std::io::SeekFrom;
    let origin = if origin == SEEK_SET {
        SeekFrom::Start(offset as u64)
    } else if origin == SEEK_CUR {
        SeekFrom::Current(offset as i64)
    } else {
        SeekFrom::End(offset as i64)
    };
    let result = if b.seek(origin).is_ok() { 0 } else { 1 };
    std::mem::forget(b);
    result
}

impl SeekableCStream {
    /// Create a compressor with the given level and frame size. When seeking in the file, frames are decompressed one by one, so this should be chosen of a size similar to the chunks that will be decompressed.
    pub fn new(level: usize, frame_size: usize) -> Result<Self, Error> {
        unsafe {
            let p = ZSTD_seekable_createCStream();
            if p.is_null() {
                return Err(Error::Null);
            }
            let result = ZSTD_seekable_initCStream(p, level as c_int, 1, frame_size as c_uint);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(SeekableCStream { p })
        }
    }
    /// Compress one chunk of input, and write it into the output. The `output` array must be large enough to hold the result. If successful, this function returns two integers `(out_pos, in_pos)`, where `out_pos` is the number of bytes written in `output`, and `in_pos` is the number of input bytes consumed.
    pub fn compress(&mut self, output: &mut [u8], input: &[u8]) -> Result<(usize, usize), Error> {
        unsafe {
            let mut input = ZSTD_inBuffer {
                src: input.as_ptr() as *const c_void,
                size: input.len() as size_t,
                pos: 0,
            };
            let mut output = ZSTD_outBuffer {
                dst: output.as_mut_ptr() as *mut c_void,
                size: output.len() as size_t,
                pos: 0,
            };
            let result = ZSTD_seekable_compressStream(self.p, &mut output, &mut input);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok((output.pos as usize, input.pos as usize))
        }
    }

    /// Finish writing the message, i.e. write the remaining pending block.
    pub fn end_stream(&mut self, output: &mut [u8]) -> Result<usize, Error> {
        unsafe {
            let mut output = ZSTD_outBuffer {
                dst: output.as_mut_ptr() as *mut c_void,
                size: output.len() as size_t,
                pos: 0,
            };
            let result = ZSTD_seekable_endStream(self.p, &mut output);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(output.pos as usize)
        }
    }
}

impl CStream {
    pub fn new(level: usize) -> Result<Self, Error> {
        unsafe {
            let p = ZSTD_createCStream();
            if p.is_null() {
                return Err(Error::Null);
            }
            let result = ZSTD_initCStream(p, level as c_int);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(CStream { p })
        }
    }

    pub fn in_size() -> usize {
        unsafe { ZSTD_CStreamInSize() as usize }
    }

    pub fn out_size() -> usize {
        unsafe { ZSTD_CStreamOutSize() as usize }
    }

    /// Compress one chunk of input, and write it into the output. The `output` array must be large enough to hold the result. If successful, this function returns three integers `(out_pos, in_pos, next_read_size)`, where `out_pos` is the number of bytes written in `output`, `in_pos` is the number of input bytes consumed, and `next_read_size` is a hint for the next read size.
    pub fn compress(
        &mut self,
        output: &mut [u8],
        input: &[u8],
    ) -> Result<(usize, usize, usize), Error> {
        unsafe {
            let mut input = ZSTD_inBuffer {
                src: input.as_ptr() as *const c_void,
                size: input.len() as size_t,
                pos: 0,
            };
            let mut output = ZSTD_outBuffer {
                dst: output.as_mut_ptr() as *mut c_void,
                size: output.len() as size_t,
                pos: 0,
            };
            let result = ZSTD_compressStream(self.p, &mut output, &mut input);
            Ok((output.pos as usize, input.pos as usize, result as usize))
        }
    }

    pub fn compress2(
        &mut self,
        output: &mut [u8],
        input: &[u8],
        op: EndDirective,
    ) -> Result<(usize, usize, usize), Error> {
        unsafe {
            let mut input = ZSTD_inBuffer {
                src: input.as_ptr() as *const c_void,
                size: input.len() as size_t,
                pos: 0,
            };
            let mut output = ZSTD_outBuffer {
                dst: output.as_mut_ptr() as *mut c_void,
                size: output.len() as size_t,
                pos: 0,
            };
            let result =
                ZSTD_compressStream2(self.p, &mut output, &mut input, op as ZSTD_EndDirective);
            Ok((output.pos as usize, input.pos as usize, result as usize))
        }
    }

    pub fn flush(&mut self, output: &mut [u8]) -> Result<(usize, usize), Error> {
        unsafe {
            let mut output = ZSTD_outBuffer {
                dst: output.as_mut_ptr() as *mut c_void,
                size: output.len() as size_t,
                pos: 0,
            };
            let result = ZSTD_flushStream(self.p, &mut output);
            Ok((output.pos as usize, result as usize))
        }
    }

    /// Finish writing the message, i.e. write the remaining pending block.
    pub fn end(&mut self, output: &mut [u8]) -> Result<usize, Error> {
        unsafe {
            let mut output = ZSTD_outBuffer {
                dst: output.as_mut_ptr() as *mut c_void,
                size: output.len() as size_t,
                pos: 0,
            };
            let result = ZSTD_endStream(self.p, &mut output);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(output.pos as usize)
        }
    }
}

pub enum EndDirective {
    Continue = 0,
    Flush = 1,
    End = 2,
}

pub struct DStream {
    p: *mut ZSTD_DStream,
}

unsafe impl Send for DStream {}

impl Drop for DStream {
    fn drop(&mut self) {
        unsafe {
            ZSTD_freeDStream(self.p);
            self.p = std::ptr::null_mut();
        }
    }
}

impl DStream {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let p = ZSTD_createDStream();
            if p.is_null() {
                return Err(Error::Null);
            }
            let result = ZSTD_initDStream(p);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(DStream { p })
        }
    }

    pub fn decompress(&mut self, output: &mut [u8], input: &[u8]) -> Result<(usize, usize), Error> {
        unsafe {
            let mut input = ZSTD_inBuffer {
                src: input.as_ptr() as *const c_void,
                size: input.len() as size_t,
                pos: 0,
            };
            let mut output = ZSTD_outBuffer {
                dst: output.as_mut_ptr() as *mut c_void,
                size: output.len() as size_t,
                pos: 0,
            };
            let _result = ZSTD_decompressStream(self.p, &mut output, &mut input);
            Ok((output.pos as usize, input.pos as usize))
        }
    }
}

impl<'a> Seekable<'a, ()> {
    /// Initialise a decompressor with an input buffer.
    pub fn init_buf(input: &'a [u8]) -> Result<Self, Error> {
        unsafe {
            let p = ZSTD_seekable_create();
            if p.is_null() {
                return Err(Error::Null);
            }
            let result =
                ZSTD_seekable_initBuff(p, input.as_ptr() as *const c_void, input.len() as size_t);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(Seekable {
                p,
                f: std::ptr::null_mut(),
                b: std::ptr::null_mut(),
                marker: std::marker::PhantomData,
            })
        }
    }

    /// Initialise a decompressor with a file. This method opens the file, and dropping the resulting `Seekable` closes the file.
    pub fn init_file(name_: &str) -> Result<Self, Error> {
        unsafe {
            let name = CString::new(name_).unwrap();
            let f: *mut libc::FILE = fopen(name.as_ptr(), "rb\0".as_ptr() as *const c_char);
            if f.is_null() {
                return Err(Error::CouldNotOpenFile(name_.to_string()));
            }
            let p = ZSTD_seekable_create();
            if p.is_null() {
                return Err(Error::Null);
            }
            let result = ZSTD_seekable_initFile(p, f as *mut FILE);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(Seekable {
                p,
                f: f as *mut c_void,
                b: std::ptr::null_mut(),
                marker: std::marker::PhantomData,
            })
        }
    }
}

impl<'a, R: std::io::Read + std::io::Seek> Seekable<'a, R> {
    /// Initialise a decompressor with a file. This method opens the file, and dropping the resulting `Seekable` closes the file.
    pub fn init(r: Box<R>) -> Result<Self, Error> {
        unsafe {
            let p = ZSTD_seekable_create();
            if p.is_null() {
                return Err(Error::Null);
            }
            let opaque = Box::into_raw(r) as *mut R;
            let adv = ZSTD_seekable_customFile {
                opaque: opaque as *mut c_void,
                read: Some(zstd_seekable_read::<R>),
                seek: Some(zstd_seekable_seek::<R>),
            };
            let result = ZSTD_seekable_initAdvanced(p, adv);
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(Seekable {
                p,
                f: std::ptr::null_mut(),
                b: opaque,
                marker: std::marker::PhantomData,
            })
        }
    }

    pub fn into_inner(self) -> Box<R> {
        unsafe { Box::from_raw(self.b) }
    }
}

impl<'a, R> Seekable<'a, R> {
    /// Decompress starting from an offset. The length of `out` must
    /// be at most the size of the decompressed output.
    ///
    /// This function finds the correct frame to start with, and takes
    /// care of decompressing multiple frames in a row.
    pub fn decompress(&mut self, out: &mut [u8], offset: u64) -> Result<usize, Error> {
        unsafe {
            let result = ZSTD_seekable_decompress(
                self.p,
                out.as_mut_ptr() as *mut c_void,
                out.len() as size_t,
                offset,
            );
            if ZSTD_isError(result) != 0 {
                return Err(Error::ZSTD(result));
            }
            Ok(result as usize)
        }
    }

    /// Number of frames in the message.
    pub fn get_num_frames(&self) -> usize {
        unsafe { ZSTD_seekable_getNumFrames(self.p) as usize }
    }
    /// Offset of the frame in the compressed data.
    pub fn get_frame_compressed_offset(&self, frame_index: usize) -> c_ulonglong {
        unsafe { ZSTD_seekable_getFrameCompressedOffset(self.p, frame_index as c_uint) }
    }
    /// Size of the frame in the compressed data.
    pub fn get_frame_compressed_size(&self, frame_index: usize) -> usize {
        unsafe { ZSTD_seekable_getFrameCompressedSize(self.p, frame_index as c_uint) as usize }
    }
    /// Offset of the frame in the decompressed data.
    pub fn get_frame_decompressed_offset(&self, frame_index: usize) -> u64 {
        unsafe { ZSTD_seekable_getFrameDecompressedOffset(self.p, frame_index as c_uint) }
    }
    /// Size of the frame in the decompressed data.
    pub fn get_frame_decompressed_size(&self, frame_index: usize) -> usize {
        unsafe { ZSTD_seekable_getFrameDecompressedSize(self.p, frame_index as c_uint) as usize }
    }
    /// Decompress a single frame. This method internally calls `decompress`, and `dest` must be exactly the size of the uncompressed frame.
    pub fn decompress_frame(&mut self, dest: &mut [u8], index: usize) -> usize {
        unsafe {
            ZSTD_seekable_decompressFrame(
                self.p,
                dest.as_mut_ptr() as *mut c_void,
                dest.len() as size_t,
                index as c_uint,
            ) as usize
        }
    }
    /// Perform a binary search to find the frame containing the offset.
    pub fn seekable_offset_to_frame_index(&mut self, offset: u64) -> usize {
        unsafe { ZSTD_seekable_offsetToFrameIndex(self.p, offset) as usize }
    }
}

pub trait Dst: Send {
    fn as_mut_ptr(&mut self) -> *mut u8;
    fn as_slice(&self) -> &[u8];
    fn len(&self) -> usize;
    fn new() -> Self;
}
impl Dst for [u8; 256] {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut().as_mut_ptr()
    }
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
    fn len(&self) -> usize {
        256
    }
    fn new() -> Self {
        [0; 256]
    }
}
impl Dst for [u8; 512] {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut().as_mut_ptr()
    }
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
    fn len(&self) -> usize {
        512
    }
    fn new() -> Self {
        [0; 512]
    }
}
impl Dst for [u8; 1024] {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut().as_mut_ptr()
    }
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
    fn len(&self) -> usize {
        1024
    }
    fn new() -> Self {
        [0; 1024]
    }
}
impl Dst for [u8; 2048] {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut().as_mut_ptr()
    }
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
    fn len(&self) -> usize {
        2048
    }
    fn new() -> Self {
        [0; 2048]
    }
}
impl Dst for [u8; 4096] {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut().as_mut_ptr()
    }
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
    fn len(&self) -> usize {
        4096
    }
    fn new() -> Self {
        [0; 4096]
    }
}

pub struct CompressedFrame<D: Dst> {
    src_size: size_t,
    dst_size: size_t,
    checksum: c_uint,
    dst: D,
}

impl<D: Dst> CompressedFrame<D> {
    fn as_slice(&self) -> &[u8] {
        &self.dst.as_slice()[..self.dst_size as usize]
    }
}

extern "C" {
    fn xxh64(src: *const u8, len: c_int) -> c_uint;
}

fn compress_frame<D: Dst>(
    src_ptr: *const u8,
    src_len: size_t,
    level: usize,
) -> Result<CompressedFrame<D>, Error> {
    unsafe {
        let mut dst = D::new();
        let checksum = xxh64(src_ptr, src_len as c_int);
        let ret = ZSTD_compress(
            dst.as_mut_ptr() as *mut c_void,
            dst.len() as size_t,
            src_ptr as *const c_void,
            src_len,
            level as c_int,
        );
        if ZSTD_isError(ret) != 0 {
            return Err(Error::ZSTD(ret));
        }
        println!("{:?}", dst.as_slice());
        Ok(CompressedFrame {
            src_size: src_len,
            dst_size: ret,
            checksum,
            dst,
        })
    }
}

impl FrameLog {
    pub fn new() -> Self {
        let p = unsafe { ZSTD_seekable_createFrameLog(1) };
        assert!(!p.is_null());
        FrameLog { p }
    }

    pub fn log_frame<D: Dst>(&mut self, frame: &CompressedFrame<D>) -> usize {
        unsafe {
            ZSTD_seekable_logFrame(
                self.p,
                frame.dst_size as c_uint,
                frame.src_size as c_uint,
                frame.checksum,
            ) as usize
        }
    }

    pub fn write_all<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut output = [0; 1024];
        let mut output_ = ZSTD_outBuffer {
            dst: output.as_mut_ptr() as *mut c_void,
            size: 1024,
            pos: 0,
        };
        unsafe {
            while ZSTD_seekable_writeSeekTable(self.p, &mut output_) != 0 {
                w.write_all(&output[..output_.pos as usize])?;
                output_.pos = 0;
            }
            w.write_all(&output[..output_.pos as usize])?;
        }
        Ok(())
    }
}

struct Chunk {
    p: *const u8,
    len: size_t,
}

unsafe impl Send for Chunk {}

pub fn parallel_compress<W: std::io::Write, D: Dst + 'static>(
    src: &[u8],
    mut output: W,
    level: usize,
    jobs: usize,
) -> Result<(), Error> {
    use std::sync::mpsc::channel;
    use threadpool::ThreadPool;

    let chunk_size = 256;
    let n = src.len() / chunk_size + if src.len() % 256 == 0 { 0 } else { 1 };
    let pool = ThreadPool::new(jobs);

    let (tx, rx) = channel();
    for (i, chunk) in src.chunks(chunk_size).enumerate() {
        let tx = tx.clone();
        let chunk = Chunk {
            p: chunk.as_ptr(),
            len: chunk.len() as size_t,
        };
        pool.execute(move || {
            let frame = compress_frame(chunk.p, chunk.len, level).unwrap();
            tx.send((i, frame))
                .expect("channel will be there waiting for the pool");
        });
    }

    let frames: Vec<CompressedFrame<D>> = unsafe {
        let mut frames = Vec::with_capacity(n);
        frames.set_len(n);
        for (i, frame) in rx.iter().take(n) {
            frames[i] = frame
        }
        frames
    };
    let mut log = FrameLog::new();
    for frame in frames.iter() {
        output.write_all(frame.as_slice())?;
        log.log_frame(frame);
    }
    log.write_all(&mut output)?;
    Ok(())
}

#[test]
fn test_par() {
    let input = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut diam ante, sollicitudin a dolor et, volutpat elementum nulla. Etiam nec efficitur nibh, quis rutrum risus. Maecenas quis lorem malesuada, aliquet mi vel, viverra nunc. Donec et nulla sed velit sagittis varius. Suspendisse vestibulum, neque lobortis ornare vestibulum, orci turpis vulputate nisi, ut sodales neque purus eget magna. Nunc condimentum, diam eu consequat venenatis, est nisl semper lorem, et lobortis velit justo sed nulla. Nunc sit amet tempor nunc, vel posuere ipsum. Cras erat tortor, pulvinar ac pretium eu, auctor ac nibh. Duis iaculis porta magna, eu lobortis elit. Duis vitae massa eros. Nulla non magna accumsan, egestas quam sit amet, laoreet lectus.";
    let mut output = Vec::new();
    parallel_compress::<&mut Vec<u8>, [u8; 256]>(input, &mut output, 10, 4).unwrap();
    let mut decomp = Vec::new();
    let mut s = { Seekable::init_buf(&output).unwrap() };
    for frame in 0..s.get_num_frames() {
        let size = s.get_frame_decompressed_size(frame);
        let n = decomp.len();
        decomp.extend(std::iter::repeat(0).take(size));
        s.decompress_frame(&mut decomp[n..], frame);
    }
    println!("{:?}", std::str::from_utf8(&decomp).unwrap());

    assert_eq!(&input[..], &decomp[..])
}

#[test]
fn test() {
    let mut cstream = SeekableCStream::new(10, 256).unwrap();
    let input = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut diam ante, sollicitudin a dolor et, volutpat elementum nulla. Etiam nec efficitur nibh, quis rutrum risus. Maecenas quis lorem malesuada, aliquet mi vel, viverra nunc. Donec et nulla sed velit sagittis varius. Suspendisse vestibulum, neque lobortis ornare vestibulum, orci turpis vulputate nisi, ut sodales neque purus eget magna. Nunc condimentum, diam eu consequat venenatis, est nisl semper lorem, et lobortis velit justo sed nulla. Nunc sit amet tempor nunc, vel posuere ipsum. Cras erat tortor, pulvinar ac pretium eu, auctor ac nibh. Duis iaculis porta magna, eu lobortis elit. Duis vitae massa eros. Nulla non magna accumsan, egestas quam sit amet, laoreet lectus.";
    let mut input_pos = 0;
    let mut output = vec![0; input.len()];
    let mut output_pos = 0;
    while input_pos < input.len() {
        let (a, b) = cstream
            .compress(&mut output[output_pos..], &input[input_pos..])
            .unwrap();
        output_pos += a;
        input_pos += b;
    }
    while let Ok(n) = cstream.end_stream(&mut output[output_pos..]) {
        if n == 0 {
            break;
        }
        output_pos += n;
    }
    output.truncate(output_pos);
    {
        use std::io::Write;
        let mut file = std::fs::File::create("test").unwrap();
        file.write_all(&output).unwrap();
    }
    println!("input len = {:?}, pos = {:?}", input.len(), output_pos);

    let mut decomp = Vec::new();
    let mut s = { Seekable::init_buf(&output).unwrap() };
    for frame in 0..s.get_num_frames() {
        let size = s.get_frame_decompressed_size(frame);
        println!("{:?} {:?}", size, s.get_frame_decompressed_offset(frame));
        let n = decomp.len();
        decomp.extend(std::iter::repeat(0).take(size));
        s.decompress_frame(&mut decomp[n..], frame);
    }
    println!("{:?}", std::str::from_utf8(&decomp).unwrap());
    assert_eq!(&input[..], &decomp[..])
}
