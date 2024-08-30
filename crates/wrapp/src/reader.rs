use std::{
    io::Cursor,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Reader<'a> {
    seekable: Arc<Mutex<dyn crate::seekable_provider::SeekableProvider<'a>>>,
}

impl Reader<'_> {
    pub fn from_file_path(path: std::path::PathBuf) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path).unwrap();
        let seekable = crate::seekable_provider::ZSTDSeekableProvider::new(file)?;
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
        })
    }

    pub fn from_vec(bytes: Vec<u8>) -> anyhow::Result<Self> {
        let seekable = crate::seekable_provider::ZSTDSeekableProvider::new(Cursor::new(bytes))?;
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
        })
    }

    pub fn from_static_slice(bytes: &'static [u8]) -> anyhow::Result<Self> {
        let seekable = crate::seekable_provider::ZSTDSeekableProvider::new(Cursor::new(bytes))?;
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
        })
    }
}

impl Reader<'_> {
    pub fn read_wasm(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut seekable: std::sync::MutexGuard<dyn crate::seekable_provider::SeekableProvider> =
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
