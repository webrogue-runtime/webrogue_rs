use std::{
    io::{Cursor, Read, Write},
    sync::{Arc, Mutex},
};

pub fn archive(
    dir_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
) -> anyhow::Result<()> {
    let mut cstream = zstd_seekable::SeekableCStream::new(5, 64*1024).unwrap();

    let mut output = std::fs::File::create(output_path.clone())?;
    let mut out_buffer = [0; 10];

    for file_rel_path in vec!["main.wasm"] {
        let mut file = std::fs::File::open(dir_path.join(file_rel_path))?;
        let mut buffer = [0; 1024];
        loop {
            let readed = file.read(&mut buffer)?;
            if readed == 0 {
                break;
            };

            let mut writed = 0;

            while writed < readed {
                let (out_pos, in_pos) = cstream
                    .compress(&mut out_buffer, &buffer[writed..readed])
                    .unwrap();
                output.write_all(&out_buffer[..out_pos]).unwrap();
                writed += in_pos;
            }
        }
    }

    while let Ok(n) = cstream.end_stream(&mut out_buffer) {
        if n == 0 {
            break;
        }
        output.write_all(&out_buffer[..n]).unwrap();
    }
    return anyhow::Ok(());
}

trait SeekableProvider<'a>: Send {
    fn get_num_frames(&self) -> usize;
    fn get_frame_decompressed_size(&self, frame_index: usize) -> usize;
    fn decompress_frame(&mut self, dest: &mut [u8], index: usize) -> usize;
}

// #[derive(Send)]
struct ZSTDSeekableProvider<'a, R> {
    seekable: zstd_seekable::Seekable<'a, R>,
}

unsafe impl<R> Send for ZSTDSeekableProvider<'_, R> {}

impl<'a, R> ZSTDSeekableProvider<'a, R> {
    fn new(seekable: zstd_seekable::Seekable<'a, R>) -> Self {
        Self { seekable }
    }
}

impl<R> SeekableProvider<'_> for ZSTDSeekableProvider<'_, R> {
    fn get_num_frames(&self) -> usize {
        self.seekable.get_num_frames()
    }

    fn get_frame_decompressed_size(&self, frame_index: usize) -> usize {
        self.seekable.get_frame_decompressed_size(frame_index)
    }

    fn decompress_frame(&mut self, dest: &mut [u8], index: usize) -> usize {
        self.seekable.decompress_frame(dest, index)
    }
}

#[derive(Clone)]
pub struct Reader<'a> {
    seekable: Arc<Mutex<dyn SeekableProvider<'a>>>,
}

impl Reader<'_> {
    #[cfg(not(target_family = "wasm"))]
    pub fn from_file_path(path: std::path::PathBuf) -> anyhow::Result<Self> {
        let seekable = zstd_seekable::Seekable::init_file(path.to_str().unwrap())?;
        let seekable = ZSTDSeekableProvider::new(seekable);
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
        })
    }

    pub fn from_vec(bytes: Vec<u8>) -> anyhow::Result<Self> {
        let seekable = zstd_seekable::Seekable::init(Box::new(Cursor::new(bytes)))?;
        let seekable = ZSTDSeekableProvider::new(seekable);
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
        })
    }

    pub fn from_static_slice(bytes: &'static [u8]) -> anyhow::Result<Self> {
        let seekable = zstd_seekable::Seekable::init(Box::new(Cursor::new(bytes)))?;
        let seekable = ZSTDSeekableProvider::new(seekable);
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
        })
    }
}

impl Reader<'_> {
    pub fn read_wasm(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut seekable: std::sync::MutexGuard<dyn SeekableProvider> =
            self.seekable.lock().unwrap();
        let mut decomp = vec![];
        for frame in 0..seekable.get_num_frames() {
            let size = seekable.get_frame_decompressed_size(frame);
            let n = decomp.len();
            decomp.extend(std::iter::repeat(0).take(size));
            seekable.decompress_frame(&mut decomp[n..], frame);
        }
        return anyhow::Ok(decomp);
    }
}
