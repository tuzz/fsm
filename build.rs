extern crate webgl_generator;

use std::env;
use std::fs::File;
use std::path::Path;
use webgl_generator::*;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("webgl.rs");
    let mut file = File::create(&dest_path).unwrap();

    Registry::new(Api::WebGl2, Exts::NONE)
        .write_bindings(StdwebGenerator, &mut file)
        .unwrap();
}
