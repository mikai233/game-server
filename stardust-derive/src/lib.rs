use proc_macro::TokenStream;

use syn::{ItemImpl, parse_macro_input};

mod lua_method;

#[proc_macro_attribute]
pub fn lua_helper(meta: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemImpl);
    lua_method::expand(&ast).into()
}

//maker only
#[proc_macro_attribute]
pub fn lua_function(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_function_mut(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_method(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_method_mut(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_meta_method(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_meta_method_mut(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_meta_function(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_meta_function_mut(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_async_function(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_async_method(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_async_meta_method(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn lua_async_meta_function(meta: TokenStream, input: TokenStream) -> TokenStream {
    input
}