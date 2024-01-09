#![crate_type = "proc-macro"]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(_: TokenStream, mut ts: TokenStream) -> TokenStream {
    let main_fn = syn::parse::<syn::ItemFn>(ts.clone())
        .expect("`#[profiling::main]` can only be applied to fn item");

    let export_main = format!(
        r#"
        const _: () = {{
            #[export_name = "main"]
            unsafe extern "C" fn __export_main() {{
                {}()
            }}
        }};
        "#,
        main_fn.sig.ident
    );
    let export_main = export_main.parse::<TokenStream>().unwrap();

    ts.extend(export_main);
    ts
}
