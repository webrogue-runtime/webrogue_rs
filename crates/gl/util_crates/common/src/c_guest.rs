use crate::types::*;
use std::io::Write;

pub fn write_to_file(file: &mut std::fs::File, parse_results: &ParseResults) {
    file.write(
        r#"#include <GLES2/gl2.h>
#include <stddef.h>
#include <stdlib.h>

// clang-format off

"#
        .as_bytes(),
    )
    .unwrap();

    for command in parse_results.commands.clone() {
        if crate::common::EXCLUDED.contains(&command.name.as_str()) {
            continue;
        }
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
__attribute__((import_module("webrogue_gl")))
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

    file.write(
        br#"

__attribute__((import_name("glGetStringData")))
__attribute__((import_module("webrogue_gl")))
void imported_glGetStringData(unsigned int name, unsigned char * data_ptr);

__attribute__((import_name("glGetStringLen")))
__attribute__((import_module("webrogue_gl")))
unsigned int imported_glGetStringLen(unsigned int name);

// TODO fix memory leak here
const GLubyte * glGetString (GLenum name) {
    unsigned int len = imported_glGetStringLen(name);
    if(!len) return NULL;
    GLubyte * data = malloc(len);
    imported_glGetStringData(name, data);
    return data;
}
"#,
    )
    .unwrap();
}
