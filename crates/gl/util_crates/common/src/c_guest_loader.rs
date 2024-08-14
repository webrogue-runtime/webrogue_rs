use crate::types::*;
use std::io::Write;

pub fn write_to_file(file: &mut std::fs::File, parse_results: &ParseResults) {
    file.write(
        r#"#include <GLES2/gl2.h>
#include <stddef.h>
#include <string.h>

// clang-format off

void* webrogueGLLoader(const char* procname) {
"#
        .as_bytes(),
    )
    .unwrap();

    for command in parse_results.commands.clone() {
        file.write(
            format!(
                r#"  if (strcmp(procname, "{}") == 0)
    return (void *){};
"#,
                command.name,
                command.name,
            )
            .as_bytes(),
        )
        .unwrap();
    }
    file.write(
        r#"  return NULL;
}
        "#
        .as_bytes(),
    )
    .unwrap();
}
