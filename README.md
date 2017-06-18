# glsl-to-spirv-macros

[![Documentation](https://docs.rs/glsl-to-spirv-macros/badge.svg)](https://docs.rs/glsl-to-spirv-macros)

```toml
[dependencies]
glsl-to-spirv-macros = "0.1.0"
glsl-to-spirv-macros-impl = "0.1.0"
```

Macros for generating SPIR-V binaries at compile time for use with Vulkan.

To use this crate, you must also use the glsl-to-spirv-macros-impl crate.

### [Documentation](https://docs.rs/glsl-to-spirv-macros)

Use this crate to compile your GLSL shaders at compile time into binaries embedded in your program.

This crate requires you to also use the `glsl-to-spirv-macros-impl` crate. Without it these macros will not work.
Unfortunately it is not yet possible to combine the two crates into one.

Example usage:

```rust
#[macro_use] extern crate glsl_to_spirv_macros;
#[macro_use] extern crate glsl_to_spirv_macros_impl;

static some_shader: &'static [u8] = glsl_vs!{r#"
    // Shader code here
"#};

fn main() {
    let another_shader = include_glsl_fs!("path/to/shader");
}
```

All macros in this crate return `&'static [u8]`, and can be used in the definition of `static` as well as local variables.
Every macro takes a string literal, e.g. `"..."`, `r#"..."#` etc.

These macros generate Vulkan-compatible SPIR-V binaries using the official glslang compiler - they
are not designed for use with other APIs, like OpenCL.