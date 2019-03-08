
fn main() {
    let mut b = cc::Build::new();

    b
        .include("tree-sitter/include")
        .include("tree-sitter/utf8proc")
        .flag("-Wno-unused-parameter");

    let files: &[&str] = &[
        // "parsers/javascript/parser.c",
        // "parsers/javascript/scanner.c",
        // "parsers/python/parser.c",
        // "parsers/python/scanner.cc",
        "parsers/rust/parser.c",
        "parsers/rust/scanner.c",
        // "parsers/bash/parser.c",
        // "parsers/bash/scanner.cc",
        // "parsers/c/parser.c",
        // // "parsers/c/scanner.c",
        // "parsers/cpp/parser.c",
        // "parsers/cpp/scanner.cc",
        // "parsers/css/parser.c",
        // "parsers/css/scanner.c",
        // // "parsers/embedded-template/parser.c",
        // // "parsers/embedded-template/scanner.c",
        // "parsers/go/parser.c",
        // // "parsers/go/scanner.c",
        // // "parsers/html/parser.c",
        // // "parsers/html/scanner.cc",
        // "parsers/ocaml/parser.c",
        // "parsers/ocaml/scanner.cc",
        // "parsers/php/parser.c",
        // "parsers/php/scanner.cc",
        // "parsers/ruby/parser.c",
        // "parsers/ruby/scanner.cc",
        // "parsers/typescript/parser.c",
        // "parsers/typescript/scanner.c",
        // "parsers/agda/parser.c",
        // "parsers/agda/scanner.cc",
        // "parsers/c-sharp/parser.c",
        // // "parsers/c-sharp/scanner.c",
        // "parsers/haskell/parser.c",
        // "parsers/haskell/scanner.cc",
        // "parsers/java/parser.c",
        // // "parsers/java/scanner.c",
        // "parsers/julia/parser.c",
        // // "parsers/julia/scanner.c",
        // "parsers/scala/parser.c",
        // "parsers/scala/scanner.c",
    ];

    for file in files {
        b.file(file);
    }

    b.compile("parsers");
}
