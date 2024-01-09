#![crate_type = "proc-macro"]

use proc_macro::TokenStream;
use syn::__private::ToTokens;

#[proc_macro_attribute]
pub fn main(_: TokenStream, ts: TokenStream) -> TokenStream {
    let main_fn = syn::parse::<syn::ItemFn>(ts.clone())
        .expect("`#[profiling::main]` can only be applied to fn item");

    let trait_impl = format!(
        r#"
        const _: () = {{
            #[export_name = "main"]
            unsafe extern "C" fn main() {{
                {}
            }}
        }};
        "#,
        main_fn.block.to_token_stream()
    );

    trait_impl.parse::<TokenStream>().unwrap()
}
