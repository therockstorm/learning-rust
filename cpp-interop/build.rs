fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .flag_if_supported("-std=c++14")
        .compile("cpp-interop");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
