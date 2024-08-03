use crate::types::*;
use std::io::Write;

pub fn write_to_file(file: &mut std::fs::File, parse_results: &ParseResults) {
    file.write(
        r#"
#include <GLES2/gl2.h>

// clang-format off

__attribute__((import_name("present")))
__attribute__((import_module("wr_gl")))
void imported_wr_gl_present();

void wr_gl_present() { imported_wr_gl_present(); }
"#
        .as_bytes(),
    )
    .unwrap();

    for command in parse_results.commands.clone() {
        let params_with_types = command
            .params
            .iter()
            .map(|param| param.ty.to_c_type() + " " + param.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let params_without_types = command
            .params
            .iter()
            .map(|param| param.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        file.write(
            format!(
                r#"
__attribute__((import_name("{}")))
__attribute__((import_module("wr_gl")))
{} imported_{}({});

{} {}({}) {{
    {}imported_{}({});
}}
        "#,
                command.name,
                command.ret.to_c_type(),
                command.name,
                params_with_types,
                command.ret.to_c_type(),
                command.name,
                params_with_types,
                if command.ret == GLType::Void {
                    ""
                } else {
                    "return "
                },
                command.name,
                params_without_types
            )
            .as_bytes(),
        )
        .unwrap();
    }
}
