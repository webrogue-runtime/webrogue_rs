use std::io::{Read, Seek};

pub struct OffsettedReader<OverallReader: Read + Seek> {
    overall_reader: OverallReader,
    offset: u64,
}

impl<OverallReader: Read + Seek> OffsettedReader<OverallReader> {
    pub fn new(mut overall_reader: OverallReader, offset: u64) -> anyhow::Result<Self> {
        overall_reader.seek(std::io::SeekFrom::Start(5))?;
        Ok(Self {
            overall_reader,
            offset,
        })
    }
}

impl<OverallReader: Read + Seek> Read for OffsettedReader<OverallReader> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.overall_reader.read(buf)
    }
}

impl<OverallReader: Read + Seek> Seek for OffsettedReader<OverallReader> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let pos = match pos {
            std::io::SeekFrom::Start(offset) => std::io::SeekFrom::Start(offset + self.offset),
            _ => pos,
        };
        self.overall_reader
            .seek(pos)
            .map(|offset| offset - self.offset)
    }
}
