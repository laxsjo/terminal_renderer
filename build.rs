use std::env;

fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("Building for target os {}", os);

    match os.as_str() {
        "windows" => {}
        "linux" => {
            // source:
            // https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
            // this is apparently bad practice:
            // https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue
            // But I don't care! ;)
            println!("cargo:rustc-env=WGPU_BACKEND=gl");
        }
        _ => {}
    }

    // Tell Cargo that if the given file changes, to rerun this build script.
    // println!("cargo:rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    // cc::Build::new()
    //     .file("src/hello.c")
    //     .compile("hello");
}
