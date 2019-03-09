
fn main() {
    let mut c = cc::Build::new();
    c
        .include("tree-sitter/include")
        .include("tree-sitter/utf8proc")
        .flag("-Wno-unused-parameter");

    let mut cpp = cc::Build::new();
    cpp
        .include("tree-sitter/include")
        .include("tree-sitter/utf8proc")
        .flag("-Wno-unused-parameter")
        .cpp(true)
        .flag("-std=c++11");

    #[cfg(feature = "lang_rust")]
    {
        c.file("parsers/rust/parser.c");
        c.file("parsers/rust/scanner.c");
    }

    #[cfg(feature = "lang_javascript")]
    {
        c.file("parsers/javascript/parser.c");
        c.file("parsers/javascript/scanner.c");
    }

    #[cfg(feature = "lang_python")]
    {
        c.file("parsers/python/parser.c");
        cpp.file("parsers/python/scanner.cc");
    }

    #[cfg(feature = "lang_bash")]
    {
        c.file("parsers/bash/parser.c");
        cpp.file("parsers/bash/scanner.cc");
    }

    #[cfg(feature = "lang_c")]
    {
        c.file("parsers/c/parser.c");
    }

    #[cfg(feature = "lang_cpp")]
    {
        c.file("parsers/cpp/parser.c");
        cpp.file("parsers/cpp/scanner.cc");
    }

    #[cfg(feature = "lang_css")]
    {
        c.file("parsers/css/parser.c");
        c.file("parsers/css/scanner.c");
    }

    #[cfg(feature = "lang_go")]
    {
        c.file("parsers/go/parser.c");
    }

    #[cfg(feature = "lang_html")]
    {
        c.file("parsers/html/parser.c");
        cpp.file("parsers/html/scanner.cc");
    }

    #[cfg(feature = "lang_ocaml")]
    {
        c.file("parsers/ocaml/parser.c");
        cpp.file("parsers/ocaml/scanner.cc");
    }

    #[cfg(feature = "lang_php")]
    {
        c.file("parsers/php/parser.c");
        cpp.file("parsers/php/scanner.cc");
    }

    #[cfg(feature = "lang_ruby")]
    {
        c.file("parsers/ruby/parser.c");
        cpp.file("parsers/ruby/scanner.cc");
    }

    #[cfg(feature = "lang_typescript")]
    {
        c.file("parsers/typescript/parser.c");
        c.file("parsers/typescript/scanner.c");
    }

    #[cfg(feature = "lang_agda")]
    {
        c.file("parsers/agda/parser.c");
        cpp.file("parsers/agda/scanner.cc");
    }

    #[cfg(feature = "lang_csharp")]
    {
        c.file("parsers/c-sharp/parser.c");
    }

    #[cfg(feature = "lang_haskell")]
    {
        c.file("parsers/haskell/parser.c");
        cpp.file("parsers/haskell/scanner.cc");
    }

    #[cfg(feature = "lang_java")]
    {
        c.file("parsers/java/parser.c");
    }

    #[cfg(feature = "lang_julia")]
    {
        c.file("parsers/julia/parser.c");
    }

    #[cfg(feature = "lang_scala")]
    {
        c.file("parsers/scala/parser.c");
        c.file("parsers/scala/scanner.c");
    }

    c.compile("parsers_c");

    #[cfg(any(feature="lang_python", feature="lang_bash", feature="lang_cpp",
        feature="lang_html", feature="lang_ocaml", feature="lang_php",
        feature="lang_ruby", feature="lang_agda", feature="lang_haskell"))]
    {
        cpp.compile("parsers_cpp");
    }
}
