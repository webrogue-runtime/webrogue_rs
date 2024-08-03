use crate::types::*;
use std::io::Write;

pub fn write_to_file(file: &mut std::fs::File, parse_results: &ParseResults) {
    file.write(
        "#![allow(dead_code)]
#![allow(non_upper_case_globals)]

extern \"C\" {\n"
            .as_bytes(),
    )
    .unwrap();

    let mut keywords = std::collections::BTreeSet::new();
    keywords.insert("type");
    keywords.insert("ref");

    for command in parse_results.commands.clone() {
        file.write(
            format!(
                "    pub fn {}({}) -> {};\n",
                command.name,
                command
                    .params
                    .iter()
                    .map(|param| {
                        let mut mapped_name = param.name.clone();
                        if keywords.contains(mapped_name.as_str()) {
                            mapped_name = format!("_{}", mapped_name)
                        }
                        format!("{}: {}", mapped_name, param.ty.to_rust_type())
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                command.ret.to_rust_type()
            )
            .as_bytes(),
        )
        .unwrap();
    }
    file.write("}\n".as_bytes()).unwrap();
    let mut seen_enums = std::collections::BTreeSet::new();
    for enum_case in parse_results.enums.clone() {
        if seen_enums.contains(&enum_case.name) {
            file.write("// DUPLICATE ".as_bytes()).unwrap();
        }
        file.write(
            format!(
                "pub const {}: {} = {};\n",
                enum_case.name, enum_case.ty, enum_case.value
            )
            .as_bytes(),
        )
        .unwrap();
        seen_enums.insert(enum_case.name);
    }
}
