use std::io::{Read, Seek};

pub trait SeekableProvider<'a>: Send {
    fn get_num_frames(&self) -> usize;
    fn get_frame_decompressed_size(&self, frame_index: usize) -> usize;
    fn decompress_frame(&mut self, dest: &mut [u8], index: usize) -> usize;
}

pub struct ZSTDSeekableProvider<'a, OverallReader: Read + Seek> {
    seekable: zstd_seekable::Seekable<'a, crate::offsetted_reader::OffsettedReader<OverallReader>>,
}

unsafe impl<OverallReader: Read + Seek> Send for ZSTDSeekableProvider<'_, OverallReader> {}

impl<'a, OverallReader: Read + Seek> ZSTDSeekableProvider<'a, OverallReader> {
    pub fn new(overall_reader: OverallReader, offset: u64) -> anyhow::Result<Self> {
        let offsetted_reader = Box::new(crate::offsetted_reader::OffsettedReader::new(
            overall_reader,
            offset,
        )?);
        Ok(Self {
            seekable: zstd_seekable::Seekable::init(offsetted_reader)?,
        })
    }
}

impl<OverallReader: Read + Seek> SeekableProvider<'_> for ZSTDSeekableProvider<'_, OverallReader> {
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
