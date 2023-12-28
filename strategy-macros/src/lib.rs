#![crate_type = "proc-macro"]

use proc_macro::TokenStream;
use syn::__private::ToTokens;

#[proc_macro_attribute]
pub fn main(_: TokenStream, mut ts: TokenStream) -> TokenStream {
    let main_fn = syn::parse::<syn::ItemFn>(ts.clone())
        .expect("`#[strategy::main]` can only be applied to fn item");

    let trait_impl = format!(
        r#"
        const _: () = {{
            #[export_name = "main"]
            unsafe extern "C" fn main() {{
                {}()
            }}

            #[export_name = "alloc"]
            unsafe extern "C" fn alloc(size: i32, align: i32) -> i32 {{
                extern crate alloc;
                use alloc::alloc::alloc;
                use core::alloc::Layout;

                let layout = Layout::from_size_align(size as _, align as _).unwrap();
                let ptr = unsafe {{ alloc(layout) }};
                ptr as _
            }}
        }};
        "#,
        main_fn.sig.ident.to_string()
    );

    ts.extend(trait_impl.parse::<TokenStream>().unwrap());
    ts
}
