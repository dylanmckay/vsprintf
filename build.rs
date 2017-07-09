extern crate gcc;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.c");

    gcc::compile_library("libvsprintf.a", &["src/lib.c"]);
}

