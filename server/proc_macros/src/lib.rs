use std::{fs::File, io::Read};

use css_inline::CSSInliner;
use minify_html_onepass::{in_place_str, Cfg};
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Compiles an email template into the binary as a &str, also inlines the css and minifies it.
/// The file path should be relative to the workspace Cargo.toml.
/// TODO Make it so path can be relative to the file the macro runs in at compile time, basically wait for [this](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.source_file) to become stable.
#[proc_macro]
pub fn include_email_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let filepath = input.value();
    let mut file = File::open(filepath).expect("file could not be found");
    let mut html = String::new();
    file.read_to_string(&mut html)
        .expect("file content could not be read to string");
    let mut html = CSSInliner::compact()
        .inline(&html)
        .expect("css could not be inlined");
    let minify_cfg = Cfg {
        minify_css: true,
        minify_js: true,
    };
    let html = in_place_str(html.as_mut_str(), &minify_cfg).expect("html could not be minified");
    let output = quote! {
        #html
    };
    output.into()
}
