use proc_macro::TokenStream;

mod shared;
mod v8;
mod wasm3;
mod wasmer;
mod wasmtime;

#[proc_macro]
pub fn make_v8_link_functions(item: TokenStream) -> TokenStream {
    v8::make_link_functions(item)
}

#[proc_macro]
pub fn make_wasm3_link_functions(item: TokenStream) -> TokenStream {
    wasm3::make_link_functions(item)
}

#[proc_macro]
pub fn make_wasmer_link_functions(item: TokenStream) -> TokenStream {
    wasmer::make_link_functions(item)
}

#[proc_macro]
pub fn make_wasmtime_link_functions(item: TokenStream) -> TokenStream {
    wasmtime::make_link_functions(item)
}
