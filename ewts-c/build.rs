extern crate cbindgen;

use cbindgen::Language;

use std::env;

fn main() {
    gen(Language::C);
    gen(Language::Cxx);
    gen(Language::Cython);
}

fn gen(lang: Language) {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let fname = format!("ewts-{}.h", lang_as_str(lang));

    let bindings = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(lang)
        .generate()
        .unwrap_or_else(|_| panic!("Unable to generate bindings for {:?}", lang));

    bindings.write_to_file(&fname);
}

fn lang_as_str(lang: Language) -> &'static str {
    match lang {
        Language::C => "c",
        Language::Cxx => "cpp",
        Language::Cython => "cython",
    }
}
