fn get_proc_address(gfx_context: &mut webrogue_gfx::Context, name: &str) -> *const () {
    gfx_context
        .system
        .as_mut()
        .unwrap()
        .gl_get_proc_address(name)
}

webrogue_gl_gen_macro::make_proc_addresses!();
