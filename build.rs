extern crate gcc;

fn main() {
    gcc::compile_library("libvsprintf.a", &["src/lib.c"]);
}

