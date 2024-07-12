use std::io::{Read, Write};

pub fn archive(
    dir_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
) -> anyhow::Result<()> {
    let mut cstream = zstd_seekable::SeekableCStream::new(10, 1024).unwrap();
    let mut input_vec = vec![];

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
            input_vec.write_all(&buffer[..readed])?;

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

    let input = input_vec.as_slice();

    let mut decomp = Vec::new();
    let mut s = zstd_seekable::Seekable::init_file(output_path.clone().to_str().unwrap())?;
    for frame in 0..s.get_num_frames() {
        let size = s.get_frame_decompressed_size(frame);
        let n = decomp.len();
        decomp.extend(std::iter::repeat(0).take(size));
        s.decompress_frame(&mut decomp[n..], frame);
    }
    assert_eq!(&input[..], &decomp[..]);
    decomp.resize(20, 0);
    s.decompress(decomp.as_mut_slice(), 30).unwrap();
    assert_eq!(&input[30..50], &decomp[..]);
    return anyhow::Ok(());
}

pub fn read_wasm(path: std::path::PathBuf) -> anyhow::Result<Vec<u8>> {
    let mut decomp = Vec::new();
    let mut s = zstd_seekable::Seekable::init_file(path.to_str().unwrap())?;
    for frame in 0..s.get_num_frames() {
        let size = s.get_frame_decompressed_size(frame);
        let n = decomp.len();
        decomp.extend(std::iter::repeat(0).take(size));
        s.decompress_frame(&mut decomp[n..], frame);
    }
    return anyhow::Ok(decomp);
}
