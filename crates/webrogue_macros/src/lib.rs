use proc_macro::TokenStream;

mod shared;
mod wasm3;
mod wasmtime;

#[proc_macro]
pub fn make_wasm3_link_functions(_item: TokenStream) -> TokenStream {
    wasm3::make_link_functions(_item)
}

#[proc_macro]
pub fn make_wasmtime_link_functions(_item: TokenStream) -> TokenStream {
    wasmtime::make_link_functions(_item)
}
