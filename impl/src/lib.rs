extern crate glsl_to_spirv;
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/*
static foo: &'static [u32] = {
    struct Dummy;
    static DATA: [u32; 1] = [1];
    &DATA as &'static [u32]
};
fn bar() {
    //let foo =
}*/

/*#[macro_export]
macro_rules! glsl {
    ($source:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[src=$source]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}*/

#[proc_macro_derive(GLSLEmbedImpl, attributes(src, path, ty))]
pub fn macro_impl(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).ok().expect("Invalid shader macro arguments - you must specify a single string literal.");
    let mut source_string = None;
    let mut path_string = None;
    let mut ty_string = None;
    for attr in &ast.attrs {
        match attr.value {
            syn::MetaItem::NameValue(ref ident, syn::Lit::Str(ref value, _)) => {
                if ident == "src" {
                    source_string = Some(value.clone());
                } else if ident == "path" {
                    path_string = Some(value.clone());
                } else if ident == "ty" {
                    ty_string = Some(value.clone());
                } else {
                    panic!("Invalid shader macro arguments - you must specify a single string literal.")
                }
            }
            syn::MetaItem::List(ref ident, _) if ident == "allow" => {}
            _ => panic!("Invalid shader macro arguments - you must specify a single string literal.")
        }
    };
    let source_string = if let Some(source_string) = source_string {
        source_string
    } else {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
        let path = path_string.unwrap();
        let full_path = Path::new(&root).join(&path);
        let mut s = String::new();
        File::open(full_path)
            .and_then(|mut file| file.read_to_string(&mut s))
            .expect(&format!("Error reading shader source file: {:?}", path));
        s
    };
    let ty = match &*ty_string.unwrap() {
        "vs" => glsl_to_spirv::ShaderType::Vertex,
        "fs" => glsl_to_spirv::ShaderType::Fragment,
        "gs" => glsl_to_spirv::ShaderType::Geometry,
        "tcs" => glsl_to_spirv::ShaderType::TessellationControl,
        "tes" => glsl_to_spirv::ShaderType::TessellationEvaluation,
        "cs" => glsl_to_spirv::ShaderType::Compute,
        _ => panic!()
    };
    let mut spirv_file: File = match glsl_to_spirv::compile(&source_string, ty) {
        Ok(compiled) => compiled,
        Err(message) => panic!("{}\nfailed to compile shader", message),
    };
    let mut spirv_data = Vec::new();
    spirv_file.read_to_end(&mut spirv_data).unwrap();
    let count = spirv_data.len();
    let expanded = quote! {
        static DATA: [u8; #count] = #spirv_data;
    };

    expanded.parse().unwrap()
}