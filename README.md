# OpalLang
A hobby programming language very similar to Rust (hmm, I wonder why)

## Building

Requires `nasm` and `ld` on a Linux operating system.

```bash
git clone https://github.com/imTyp0/OpalLang
cd OpalLang
cargo build --release
```

Executable will be `refine` in the `target/release/` directory.
Assembly code, object file and executable of compiled code will be in the build/ directory (prone to change/be a switch on the compiler)

Heavy inspiration from:
series "[Creating a Compiler](https://www.youtube.com/playlist?list=PLUDlas_Zy_qC7c5tCgTMYq2idyyT241qs)" by Pixeled
