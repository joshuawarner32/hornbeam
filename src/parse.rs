use std::collections::HashMap;
use std::fmt;
use tree_sitter as ts;

extern "C" {
    #[cfg(feature = "lang_javascript")]
    fn tree_sitter_javascript() -> ts::Language;
    
    #[cfg(feature = "lang_python")]
    fn tree_sitter_python() -> ts::Language;
    
    #[cfg(feature = "lang_rust")]
    fn tree_sitter_rust() -> ts::Language;
    
    #[cfg(feature = "lang_bash")]
    fn tree_sitter_bash() -> ts::Language;
    
    #[cfg(feature = "lang_c")]
    fn tree_sitter_c() -> ts::Language;
    
    #[cfg(feature = "lang_cpp")]
    fn tree_sitter_cpp() -> ts::Language;
    
    #[cfg(feature = "lang_css")]
    fn tree_sitter_css() -> ts::Language;
    
    #[cfg(feature = "lang_go")]
    fn tree_sitter_go() -> ts::Language;
    
    #[cfg(feature = "lang_html")]
    fn tree_sitter_html() -> ts::Language;
    
    #[cfg(feature = "lang_ocaml")]
    fn tree_sitter_ocaml() -> ts::Language;
    
    #[cfg(feature = "lang_php")]
    fn tree_sitter_php() -> ts::Language;
    
    #[cfg(feature = "lang_ruby")]
    fn tree_sitter_ruby() -> ts::Language;
    
    #[cfg(feature = "lang_typescript")]
    fn tree_sitter_typescript() -> ts::Language;
    
    #[cfg(feature = "lang_agda")]
    fn tree_sitter_agda() -> ts::Language;
    
    #[cfg(feature = "lang_csharp")]
    fn tree_sitter_c_sharp() -> ts::Language;
    
    #[cfg(feature = "lang_haskell")]
    fn tree_sitter_haskell() -> ts::Language;
    
    #[cfg(feature = "lang_java")]
    fn tree_sitter_java() -> ts::Language;
    
    #[cfg(feature = "lang_julia")]
    fn tree_sitter_julia() -> ts::Language;
    
    #[cfg(feature = "lang_scala")]
    fn tree_sitter_scala() -> ts::Language;
}

pub enum Language {
    Javascript,
    Python,
    Rust,
    Bash,
    C,
    Cpp,
    Css,
    Go,
    Html,
    Ocaml,
    Php,
    Ruby,
    Typescript,
    Agda,
    CSharp,
    Haskell,
    Java,
    Julia,
    Scala,
}

pub struct LanguageInfo {
    kinds_by_name: HashMap<&'static str, Kind>,
    // kinds_by_id: Vec<&'static str>,
}

pub struct Parser {
    pub info: LanguageInfo,
    inner: ts::Parser,
}

pub struct Tree<'a> {
    inner: ts::Tree,
    text: &'a str,
}

#[derive(Clone)]
pub struct Node<'a> {
    inner: ts::Node<'a>,
    text: &'a str,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Kind(u16);

impl<'a> Tree<'a> {
    pub fn root(&'a self) -> Node<'a> {
        Node {
            inner: self.inner.root_node(),
            text: self.text,
        }
    }

    pub fn nodes(&'a self) -> impl Iterator<Item=Node<'a>> {
        AllWalker {
            walker: self.inner.walk(),
            descend: true,
            text: self.text,
        }
    }
}

impl<'a> fmt::Debug for Tree<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.root_node().to_sexp())
    }
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.to_sexp())
    }
}

impl<'a> Node<'a> {
    pub fn kind(&self) -> Kind {
        Kind(self.inner.kind_id())
    }

    pub fn children(&self) -> impl Iterator<Item=Node<'a>> {
        let text = self.text;
        self.inner.children().map(move |inner| Node {
            inner,
            text,
        })
    }

    pub fn text(&self) -> &'a str {
        self.inner.utf8_text(self.text.as_bytes()).unwrap()
    }
}

struct AllWalker<'a> {
    walker: ts::TreeCursor<'a>,
    descend: bool,
    text: &'a str,
}

impl<'a> Iterator for AllWalker<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = Node {
            inner: self.walker.node(),
            text: self.text,
        };
        if self.descend && self.walker.goto_first_child() {
            Some(n)
        } else {
            loop {
                if self.walker.goto_next_sibling() {
                    self.descend = true;
                    return Some(n);
                }

                if !self.walker.goto_parent() {
                    return None;
                }
            }
        }
    }
}

impl LanguageInfo {
    fn new(lang: ts::Language) -> LanguageInfo {
        let mut kinds_by_name = HashMap::new();
        // let mut kinds_by_id = Vec::new();

        for k in 0..lang.node_kind_count() {
            let name = lang.node_kind_for_id(k as u16);
            // kinds_by_id.push(name);
            if lang.node_kind_is_named(k as u16) {
                // TODO: deal with duplicate names (they exist!?!?!)
                kinds_by_name.insert(name, Kind(k as u16));
            }
        }

        LanguageInfo {
            kinds_by_name,
            // kinds_by_id,
        }
    }

    pub fn kind_from_name(&self, name: &str) -> Option<Kind> {
        self.kinds_by_name.get(name).cloned()
    }

    pub fn kind_names(&self) -> impl Iterator<Item=&str> {
        self.kinds_by_name.keys().cloned()
    }
}

impl Parser {
    pub fn new(lang_id: Language) -> Parser {
        let lang = match lang_id {
            Language::Javascript => {
                #[cfg(feature = "lang_javascript")]
                unsafe { tree_sitter_javascript() }
                #[cfg(not(feature = "lang_javascript"))]
                panic!("support for {} not compiled in", "javascript");
            }
            Language::Python => {
                #[cfg(feature = "lang_python")]
                unsafe { tree_sitter_python() }
                #[cfg(not(feature = "lang_python"))]
                panic!("support for {} not compiled in", "python");
            }
            Language::Rust => {
                #[cfg(feature = "lang_rust")]
                unsafe { tree_sitter_rust() }
                #[cfg(not(feature = "lang_rust"))]
                panic!("support for {} not compiled in", "rust");
            }
            Language::Bash => {
                #[cfg(feature = "lang_bash")]
                unsafe { tree_sitter_bash() }
                #[cfg(not(feature = "lang_bash"))]
                panic!("support for {} not compiled in", "bash");
            }
            Language::C => {
                #[cfg(feature = "lang_c")]
                unsafe { tree_sitter_c() }
                #[cfg(not(feature = "lang_c"))]
                panic!("support for {} not compiled in", "c");
            }
            Language::Cpp => {
                #[cfg(feature = "lang_cpp")]
                unsafe { tree_sitter_cpp() }
                #[cfg(not(feature = "lang_cpp"))]
                panic!("support for {} not compiled in", "cpp");
            }
            Language::Css => {
                #[cfg(feature = "lang_css")]
                unsafe { tree_sitter_css() }
                #[cfg(not(feature = "lang_css"))]
                panic!("support for {} not compiled in", "css");
            }
            Language::Go => {
                #[cfg(feature = "lang_go")]
                unsafe { tree_sitter_go() }
                #[cfg(not(feature = "lang_go"))]
                panic!("support for {} not compiled in", "go");
            }
            Language::Html => {
                #[cfg(feature = "lang_html")]
                unsafe { tree_sitter_html() }
                #[cfg(not(feature = "lang_html"))]
                panic!("support for {} not compiled in", "html");
            }
            Language::Ocaml => {
                #[cfg(feature = "lang_ocaml")]
                unsafe { tree_sitter_ocaml() }
                #[cfg(not(feature = "lang_ocaml"))]
                panic!("support for {} not compiled in", "ocaml");
            }
            Language::Php => {
                #[cfg(feature = "lang_php")]
                unsafe { tree_sitter_php() }
                #[cfg(not(feature = "lang_php"))]
                panic!("support for {} not compiled in", "php");
            }
            Language::Ruby => {
                #[cfg(feature = "lang_ruby")]
                unsafe { tree_sitter_ruby() }
                #[cfg(not(feature = "lang_ruby"))]
                panic!("support for {} not compiled in", "ruby");
            }
            Language::Typescript => {
                #[cfg(feature = "lang_typescript")]
                unsafe { tree_sitter_typescript() }
                #[cfg(not(feature = "lang_typescript"))]
                panic!("support for {} not compiled in", "typescript");
            }
            Language::Agda => {
                #[cfg(feature = "lang_agda")]
                unsafe { tree_sitter_agda() }
                #[cfg(not(feature = "lang_agda"))]
                panic!("support for {} not compiled in", "agda");
            }
            Language::CSharp => {
                #[cfg(feature = "lang_csharp")]
                unsafe { tree_sitter_c_sharp() }
                #[cfg(not(feature = "lang_csharp"))]
                panic!("support for {} not compiled in", "csharp");
            }
            Language::Haskell => {
                #[cfg(feature = "lang_haskell")]
                unsafe { tree_sitter_haskell() }
                #[cfg(not(feature = "lang_haskell"))]
                panic!("support for {} not compiled in", "haskell");
            }
            Language::Java => {
                #[cfg(feature = "lang_java")]
                unsafe { tree_sitter_java() }
                #[cfg(not(feature = "lang_java"))]
                panic!("support for {} not compiled in", "java");
            }
            Language::Julia => {
                #[cfg(feature = "lang_julia")]
                unsafe { tree_sitter_julia() }
                #[cfg(not(feature = "lang_julia"))]
                panic!("support for {} not compiled in", "julia");
            }
            Language::Scala => {
                #[cfg(feature = "lang_scala")]
                unsafe { tree_sitter_scala() }
                #[cfg(not(feature = "lang_scala"))]
                panic!("support for {} not compiled in", "scala");
            }
        };

        let mut inner = ts::Parser::new();
        inner.set_language(lang).unwrap();

        Parser {
            info: LanguageInfo::new(lang),
            inner,
        }
    }

    pub fn parse<'a>(&mut self, text: &'a str) -> Tree<'a> {
        Tree {
            inner: self.inner.parse(text, None).unwrap(),
            text,
        }
    }
}