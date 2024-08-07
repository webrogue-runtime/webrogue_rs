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
        "../../../../example_apps/libs/wr_gl/wr_gl.c",
        &parse_results,
    );

    run_flavor(
        wr_defs::write_to_file,
        "../../../../crates/gl/defs.in",
        &parse_results,
    );
}
