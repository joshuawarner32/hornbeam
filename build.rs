
struct Compiler {
    c: cc::Build,
    cpp: cc::Build,
    saw_c_file: bool,
    saw_cpp_file: bool,
}

impl Compiler {
    fn new() -> Compiler {
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

        Compiler {
            c,
            cpp,
            saw_c_file: false,
            saw_cpp_file: false,
        }
    }

    fn c_file(&mut self, path: &str) {
        self.saw_c_file = true;
        self.c.file(path);
        println!("cargo:rerun-if-changed={}", path);
    }

    fn cpp_file(&mut self, path: &str) {
        self.saw_cpp_file = true;
        self.cpp.file(path);
        println!("cargo:rerun-if-changed={}", path);
    }

    fn finish(self) {
        if self.saw_c_file {
            self.c.compile("parsers_c");
        }

        if self.saw_cpp_file {
            self.cpp.compile("parsers_cpp");
        }
    }
}


fn main() {

    let mut compile = Compiler::new();

    #[cfg(feature = "lang_rust")]
    {
        compile.c_file("parsers/rust/parser.c");
        compile.c_file("parsers/rust/scanner.c");
    }

    #[cfg(feature = "lang_javascript")]
    {
        compile.c_file("parsers/javascript/parser.c");
        compile.c_file("parsers/javascript/scanner.c");
    }

    #[cfg(feature = "lang_python")]
    {
        compile.c_file("parsers/python/parser.c");
        compile.cpp_file("parsers/python/scanner.cc");
    }

    #[cfg(feature = "lang_bash")]
    {
        compile.c_file("parsers/bash/parser.c");
        compile.cpp_file("parsers/bash/scanner.cc");
    }

    #[cfg(feature = "lang_c")]
    {
        compile.c_file("parsers/c/parser.c");
    }

    #[cfg(feature = "lang_cpp")]
    {
        compile.c_file("parsers/cpp/parser.c");
        compile.cpp_file("parsers/cpp/scanner.cc");
    }

    #[cfg(feature = "lang_css")]
    {
        compile.c_file("parsers/css/parser.c");
        compile.c_file("parsers/css/scanner.c");
    }

    #[cfg(feature = "lang_go")]
    {
        compile.c_file("parsers/go/parser.c");
    }

    #[cfg(feature = "lang_html")]
    {
        compile.c_file("parsers/html/parser.c");
        compile.cpp_file("parsers/html/scanner.cc");
    }

    #[cfg(feature = "lang_ocaml")]
    {
        compile.c_file("parsers/ocaml/parser.c");
        compile.cpp_file("parsers/ocaml/scanner.cc");
    }

    #[cfg(feature = "lang_php")]
    {
        compile.c_file("parsers/php/parser.c");
        compile.cpp_file("parsers/php/scanner.cc");
    }

    #[cfg(feature = "lang_ruby")]
    {
        compile.c_file("parsers/ruby/parser.c");
        compile.cpp_file("parsers/ruby/scanner.cc");
    }

    #[cfg(feature = "lang_typescript")]
    {
        compile.c_file("parsers/typescript/parser.c");
        compile.c_file("parsers/typescript/scanner.c");
    }

    #[cfg(feature = "lang_agda")]
    {
        compile.c_file("parsers/agda/parser.c");
        compile.cpp_file("parsers/agda/scanner.cc");
    }

    #[cfg(feature = "lang_csharp")]
    {
        compile.c_file("parsers/c-sharp/parser.c");
    }

    #[cfg(feature = "lang_haskell")]
    {
        compile.c_file("parsers/haskell/parser.c");
        compile.cpp_file("parsers/haskell/scanner.cc");
    }

    #[cfg(feature = "lang_java")]
    {
        compile.c_file("parsers/java/parser.c");
    }

    #[cfg(feature = "lang_julia")]
    {
        compile.c_file("parsers/julia/parser.c");
    }

    #[cfg(feature = "lang_scala")]
    {
        compile.c_file("parsers/scala/parser.c");
        compile.c_file("parsers/scala/scanner.c");
    }

    compile.finish();
}
