use std::io::{Read, Seek};

pub struct SubReader<OverallReader: Read + Seek> {
    overall_reader: OverallReader,
}

impl<OverallReader: Read + Seek> SubReader<OverallReader> {
    pub fn new(overall_reader: OverallReader) -> anyhow::Result<Self> {
        Ok(Self { overall_reader })
    }
}

impl<OverallReader: Read + Seek> Read for SubReader<OverallReader> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.overall_reader.read(buf)
    }
}

impl<OverallReader: Read + Seek> Seek for SubReader<OverallReader> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.overall_reader.seek(pos)
    }
}
