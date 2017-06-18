//! Use this crate to compile your GLSL shaders at compile time into binaries embedded in your program.
//!
//! This crate requires you to also use the `glsl-to-spirv-macros-impl` crate. Without it these macros will not work.
//! Unfortunately it is not yet possible to combine the two crates into one.
//!
//! Example usage:
//!
//! ```ignore
//! #[macro_use] extern crate glsl_to_spirv_macros;
//! #[macro_use] extern crate glsl_to_spirv_macros_impl;
//!
//! static some_shader: &'static [u8] = glsl_vs!{r#"
//!     // Shader code here
//! "#};
//!
//! fn main() {
//!     let another_shader = include_glsl_fs!("path/to/shader");
//! }
//! ```
//!
//! All macros in this crate return `&'static [u8]`, and can be used in the definition of `static` as well as local variables.
//! Every macro takes a string literal, e.g. `"..."`, `r#"..."#` etc.
//!
//! These macros generate Vulkan-compatible SPIR-V binaries using the official glslang compiler - they
//! are not designed for use with other APIs, like OpenCL.

/// Compiles a string containing a GLSL vertex shader into SPIR-V binary data.
#[macro_export]
macro_rules! glsl_vs {
    ($source:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[src=$source]
        #[ty="vs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles a string containing a GLSL fragment shader into SPIR-V binary data.
#[macro_export]
macro_rules! glsl_fs {
    ($source:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[src=$source]
        #[ty="fs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles a string containing a GLSL geometry shader into SPIR-V binary data.
#[macro_export]
macro_rules! glsl_gs {
    ($source:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[src=$source]
        #[ty="gs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles a string containing a GLSL tessellation control shader into SPIR-V binary data.
#[macro_export]
macro_rules! glsl_tcs {
    ($source:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[src=$source]
        #[ty="tcs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles a string containing a GLSL tessellation evaluation shader into SPIR-V binary data.
#[macro_export]
macro_rules! glsl_tes {
    ($source:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[src=$source]
        #[ty="tes"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles a string containing a GLSL compute shader into SPIR-V binary data.
#[macro_export]
macro_rules! glsl_cs {
    ($source:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[src=$source]
        #[ty="cs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles the specified file (path relative to the crate root) containing a GLSL vertex shader into SPIR-V binary data.
#[macro_export]
macro_rules! include_glsl_vs {
    ($path:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[path=$path]
        #[ty="vs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles the specified file (path relative to the crate root) containing a GLSL fragment shader into SPIR-V binary data.
#[macro_export]
macro_rules! include_glsl_fs {
    ($path:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[path=$path]
        #[ty="fs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles the specified file (path relative to the crate root) containing a GLSL geometry shader into SPIR-V binary data.
#[macro_export]
macro_rules! include_glsl_gs {
    ($path:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[path=$path]
        #[ty="gs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles the specified file (path relative to the crate root) containing a GLSL tessellation control shader into SPIR-V binary data.
#[macro_export]
macro_rules! include_glsl_tcs {
    ($path:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[path=$path]
        #[ty="tcs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles the specified file (path relative to the crate root) containing a GLSL tessellation evaluation shader into SPIR-V binary data.
#[macro_export]
macro_rules! include_glsl_tes {
    ($path:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[path=$path]
        #[ty="tes"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}

/// Compiles the specified file (path relative to the crate root) containing a GLSL compute shader into SPIR-V binary data.
#[macro_export]
macro_rules! include_glsl_cs {
    ($path:tt) => {{
        #[derive(GLSLEmbedImpl)]
        #[path=$path]
        #[ty="cs"]
        struct Dummy;
        &DATA as &'static [u8]
    }};
}