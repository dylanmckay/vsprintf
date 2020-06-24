fn main() {
    println!("cargo:rerun-if-changed=src/lib.c");

    cc::Build::new()
        .file("src/lib.c")
        .compile("libvsprintf.a");
}

