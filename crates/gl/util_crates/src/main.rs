use webrogue_gl_gen_common::*;

fn run_flavor(
    f: impl Fn(&mut std::fs::File, &types::ParseResults),
    path: &str,
    commands: &types::ParseResults,
) {
    let mut file: std::fs::File = std::fs::File::create(path).unwrap();
    f(&mut file, commands);
}

fn main() {
    let parse_results = parse::parse();

    run_flavor(
        c_guest::write_to_file,
        "../../../examples/libs/GLES2/gl2.inc",
        &parse_results,
    );

    run_flavor(
        wr_defs::write_to_file,
        "../../../crates/gl/defs.in",
        &parse_results,
    );

    run_flavor(
        c_guest_loader::write_to_file,
        "../../../examples/libs/webrogue_gfx/webrogue_gl_loader.c",
        &parse_results,
    );
}
