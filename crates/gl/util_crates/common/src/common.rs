pub const EXCLUDED: [&str; 1] = ["glGetString"];

pub const MANUAL_IMPL: [&str; 8] = [
    "glShaderSource",
    "glGetAttribLocation",
    "glVertexAttribPointer",
    "glBindAttribLocation",
    "glGetUniformLocation",
    "glTexImage2D",
    "glBufferSubData",
    "glDrawElements",
];

pub const C_MANUAL_IMPL: [&str; 0] = [];

pub const EXTRA_EXTENSIONS: [&str; 1] = ["GL_OES_vertex_array_object"];
