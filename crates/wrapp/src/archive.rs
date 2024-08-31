use std::io::Write;

fn compress(input: Vec<u8>, output: &mut impl Write) -> anyhow::Result<()> {
    let mut out_buffer = [0; 10];
    let mut written = 0;

    let mut cstream = zstd_seekable::SeekableCStream::new(5, 64 * 1024)?;

    while written < input.len() {
        let (out_pos, in_pos) = cstream.compress(&mut out_buffer, &input[written..input.len()])?;
        output.write_all(&out_buffer[..out_pos])?;
        written += in_pos;
    }
    while let Ok(n) = cstream.end_stream(&mut out_buffer) {
        if n == 0 {
            break;
        }
        output.write_all(&out_buffer[..n])?;
    }
    Ok(())
}

pub fn archive(
    dir_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
) -> anyhow::Result<()> {
    let mut output_file = std::fs::File::create(output_path.clone())?;

    let mut filenames_to_archive: Vec<String> = Vec::new();

    filenames_to_archive.push("main.wasm".to_owned());

    let mut packed_data: Vec<u8> = Vec::new();
    let mut preamble_data: Vec<u8> = Vec::new();

    preamble_data.write_all(b"WRAPP\0")?;

    let file = std::fs::File::open(dir_path.join("webrogue.json"))?;
    let config: crate::config::Config = serde_json::from_reader(file)?;
    let json_content = serde_json::to_vec(&config)?;
    preamble_data.write_all(&json_content)?;

    for file_rel_path in filenames_to_archive {
        let mut file = std::fs::File::open(dir_path.join(file_rel_path))?;
        std::io::copy(&mut file, &mut packed_data)?;
    }
    output_file.write_all(&preamble_data)?;
    output_file.write_all(b"\0")?;
    compress(packed_data, &mut output_file)?;

    return anyhow::Ok(());
}
