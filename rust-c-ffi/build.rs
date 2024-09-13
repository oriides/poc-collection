fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/include/my_struct_lib.c");
    println!("cargo:rerun-if-changed=src/include/my_struct_lib.h");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=static=my_struct");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/include/my_struct_lib.c")
        .compile("my_struct");
}
