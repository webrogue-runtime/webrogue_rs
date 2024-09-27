fn main() {
    println!("cargo:rerun-if-changed=zstd/contrib/seekable_format/zstdseek_compress.c");
    println!("cargo:rerun-if-changed=zstd/contrib/seekable_format/zstdseek_decompress.c");

    if pkg_config::Config::new()
        .statik(true)
        .probe("libzstd")
        .is_err()
    {
        let mut cc = cc::Build::new();
        if !cfg!(target_arch = "x86_64") || !cfg!(target_family="unix") {
            cc.define("ZSTD_DISABLE_ASM", "1");
        }
        for &dir in &[
            "zstd/lib/common",
            "zstd/lib/compress",
            "zstd/lib/decompress",
        ] {
            cc.include(dir);
            for entry in std::fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let name = path.file_name();
                if let Some(f) = name {
                    let ext = std::path::Path::new(f)
                        .extension()
                        .map(|v| v.to_str().unwrap());
                    if ext == Some("c") {
                        cc.file(path);
                    } else if ext == Some("S") && cfg!(target_arch = "x86_64") && cfg!(target_family="unix") {
                        cc.file(path);
                    }
                }
            }
        }
        cc.opt_level(3).compile("zstd");
    }
    cc::Build::new()
        .include("zstd/lib/common")
        .include("zstd/lib")
        .file("zstd/contrib/seekable_format/zstdseek_compress.c")
        .file("zstd/contrib/seekable_format/zstdseek_decompress.c")
        .file("zstd/lib/common/xxhash.c")
        .file("xxh64.c")
        // .flag("-nostdlib")
        .opt_level(3)
        .compile("zstdseek");
}
