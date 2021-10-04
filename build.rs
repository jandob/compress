extern crate cc;
extern crate walkdir;

use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let crate_env = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_path = Path::new(&crate_env);

    let entries = WalkDir::new(crate_path.join("src"))
        .into_iter()
        .map(|e| e.expect("error reading file"))
        .filter(|entry| {
            entry.file_name().to_str().unwrap().ends_with(".c")
                || entry.file_name().to_str().unwrap().ends_with(".h")
        });
    for entry in entries {
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    let sources = Vec::from(["src/main.c"]);
    let includes = Vec::from(["src"]);

    cc::Build::new()
        .files(sources)
        .includes(includes)
        .define("main", "__c_main__")
        .compile("ipg");
}
