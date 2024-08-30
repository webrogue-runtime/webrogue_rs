use std::io::Write;

fn compress(input: Vec<u8>, output: &mut impl Write) {
    let mut out_buffer = [0; 10];
    let mut writed = 0;

    let mut cstream = zstd_seekable::SeekableCStream::new(5, 64 * 1024).unwrap();

    while writed < input.len() {
        let (out_pos, in_pos) = cstream
            .compress(&mut out_buffer, &input[writed..input.len()])
            .unwrap();
        output.write_all(&out_buffer[..out_pos]).unwrap();
        writed += in_pos;
    }
    while let Ok(n) = cstream.end_stream(&mut out_buffer) {
        if n == 0 {
            break;
        }
        output.write_all(&out_buffer[..n]).unwrap();
    }
}

pub fn archive(
    dir_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
) -> anyhow::Result<()> {
    let mut output_file = std::fs::File::create(output_path.clone())?;

    let mut filenames_to_archive: Vec<String> = Vec::new();

    filenames_to_archive.push("main.wasm".to_owned());

    let mut packed_data: Vec<u8> = Vec::new();

    for file_rel_path in filenames_to_archive {
        let mut file = std::fs::File::open(dir_path.join(file_rel_path))?;
        std::io::copy(&mut file, &mut packed_data)?;
    }

    compress(packed_data, &mut output_file);
    return anyhow::Ok(());
}
