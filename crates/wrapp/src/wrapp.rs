use std::{
    io::Cursor,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Wrapp<'a> {
    seekable: Arc<Mutex<dyn crate::seekable_provider::SeekableProvider<'a>>>,
    config: Arc<Mutex<crate::config::Config>>,
}

impl Wrapp<'_> {
    pub fn from_file_path(path: std::path::PathBuf) -> anyhow::Result<Self> {
        let mut overall_reader = std::fs::File::open(path).unwrap();
        let preamble = crate::preamble_reader::parse_preamble(&mut overall_reader)?;
        let seekable =
            crate::seekable_provider::ZSTDSeekableProvider::new(overall_reader, preamble.offset)?;
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
            config: Arc::new(Mutex::new(preamble.config)),
        })
    }

    pub fn from_vec(bytes: Vec<u8>) -> anyhow::Result<Self> {
        let mut overall_reader = Cursor::new(bytes);
        let preamble = crate::preamble_reader::parse_preamble(&mut overall_reader)?;
        let seekable =
            crate::seekable_provider::ZSTDSeekableProvider::new(overall_reader, preamble.offset)?;
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
            config: Arc::new(Mutex::new(preamble.config)),
        })
    }

    pub fn from_static_slice(bytes: &'static [u8]) -> anyhow::Result<Self> {
        let mut overall_reader = Cursor::new(bytes);
        let preamble = crate::preamble_reader::parse_preamble(&mut overall_reader)?;
        let seekable =
            crate::seekable_provider::ZSTDSeekableProvider::new(overall_reader, preamble.offset)?;
        anyhow::Ok(Self {
            seekable: Arc::new(Mutex::new(seekable)),
            config: Arc::new(Mutex::new(preamble.config)),
        })
    }
}

impl Wrapp<'_> {
    pub fn read_wasm(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut seekable: std::sync::MutexGuard<dyn crate::seekable_provider::SeekableProvider> =
            self.seekable.lock().unwrap();
        let mut decompressed = vec![];
        for frame in 0..seekable.get_num_frames() {
            let size = seekable.get_frame_decompressed_size(frame);
            let n = decompressed.len();
            decompressed.extend(std::iter::repeat(0).take(size));
            seekable.decompress_frame(&mut decompressed[n..], frame);
        }
        return anyhow::Ok(decompressed);
    }

    pub fn get_config(&self) -> crate::config::Config {
        return self.config.lock().unwrap().clone();
    }
}
