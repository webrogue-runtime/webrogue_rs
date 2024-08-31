pub struct Preamble {
    pub config: crate::config::Config,
    pub offset: u64,
}

pub fn parse_preamble(readable: &mut impl std::io::Read) -> anyhow::Result<Preamble> {
    let mut magic = [0u8; 6];
    readable.read_exact(&mut magic)?;
    if magic != *b"WRAPP\0" {
        anyhow::bail!("Magic number mismatch while reading WRAPP archive");
    }
    let mut preamble_content: Vec<u8> = Vec::new();
    // without zero byte
    let mut read_total = 0;
    'read_loop: loop {
        let to_read = 128;
        preamble_content
            .extend(std::iter::repeat(0).take(read_total + to_read - preamble_content.len()));
        let read = readable.read(&mut preamble_content.as_mut_slice()[read_total..])?;
        read_total += read;
        for (offset, byte) in preamble_content[(read_total - read)..].iter().enumerate() {
            if *byte == 0 {
                read_total = read_total - read + offset;
                break 'read_loop;
            }
        }
    }
    preamble_content.truncate(read_total);
    let config: crate::config::Config = serde_json::from_slice(&preamble_content)?;
    Ok(Preamble {
        config,
        offset: 6 + (read_total as u64) + 1,
    })
}
