use proc_macro::TokenStream;

#[proc_macro]
pub fn make_rust_ffi(_item: TokenStream) -> TokenStream {
    let parse_results = webrogue_gl_gen_common::parse::parse();
    webrogue_gl_gen_common::rust_ffi::get_as_str(&parse_results)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn make_rust_impl(_item: TokenStream) -> TokenStream {
    let parse_results = webrogue_gl_gen_common::parse::parse();
    webrogue_gl_gen_common::rust_wasm_imports_imps::get_as_str(&parse_results)
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn make_proc_addresses(_item: TokenStream) -> TokenStream {
    let parse_results = webrogue_gl_gen_common::parse::parse();
    webrogue_gl_gen_common::proc_addresses::get_as_str(&parse_results)
        .parse()
        .unwrap()
}
