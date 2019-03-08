use std::collections::HashMap;
use tree_sitter as ts;

extern "C" {
    // fn tree_sitter_javascript() -> ts::Language;
    // fn tree_sitter_python() -> ts::Language;
    fn tree_sitter_rust() -> ts::Language;
    // fn tree_sitter_bash() -> ts::Language;
    // fn tree_sitter_c() -> ts::Language;
    // fn tree_sitter_cpp() -> ts::Language;
    // fn tree_sitter_css() -> ts::Language;
    // fn tree_sitter_go() -> ts::Language;
    // fn tree_sitter_html() -> ts::Language;
    // fn tree_sitter_ocaml() -> ts::Language;
    // fn tree_sitter_php() -> ts::Language;
    // fn tree_sitter_ruby() -> ts::Language;
    // fn tree_sitter_typescript() -> ts::Language;
    // fn tree_sitter_agda() -> ts::Language;
    // fn tree_sitter_csharp() -> ts::Language;
    // fn tree_sitter_haskell() -> ts::Language;
    // fn tree_sitter_java() -> ts::Language;
    // fn tree_sitter_julia() -> ts::Language;
    // fn tree_sitter_scala() -> ts::Language;
}

pub enum Language {
    // Javascript,
    // Python,
    Rust,
    // Bash,
    // C,
    // Cpp,
    // Css,
    // Go,
    // Html,
    // Ocaml,
    // Php,
    // Ruby,
    // Typescript,
    // Agda,
    // CSharp,
    // Haskell,
    // Java,
    // Julia,
    // Scala,
}

pub struct LanguageInfo {
    lang: ts::Language,
    kinds_by_name: HashMap<&'static str, Kind>,
    kinds_by_id: Vec<&'static str>,
}

pub struct Parser {
    pub info: LanguageInfo,
    inner: ts::Parser,
}

pub struct Tree<'a> {
    inner: ts::Tree,
    text: &'a str,
}

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

impl<'a> Node<'a> {
    pub fn kind(&self) -> Kind {
        Kind(self.inner.kind_id())
    }

    pub fn children(&'a self) -> impl Iterator<Item=Node<'a>> {
        self.inner.children().map(move |inner| Node {
            inner,
            text: self.text,
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
        let mut kinds_by_id = Vec::new();

        for k in 0..lang.node_kind_count() {
            let name = lang.node_kind_for_id(k as u16);
            kinds_by_id.push(name);
            if lang.node_kind_is_named(k as u16) {
                // TODO: deal with duplicate names (they exist!?!?!)
                kinds_by_name.insert(name, Kind(k as u16));
            }
        }

        LanguageInfo {
            lang,
            kinds_by_name,
            kinds_by_id,
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
            // Language::Javascript => unsafe { tree_sitter_javascript() }
            // Language::Python => unsafe { tree_sitter_python() }
            Language::Rust => unsafe { tree_sitter_rust() }
            // Language::Bash => unsafe { tree_sitter_bash() }
            // Language::C => unsafe { tree_sitter_c() }
            // Language::Cpp => unsafe { tree_sitter_cpp() }
            // Language::Css => unsafe { tree_sitter_css() }
            // Language::Go => unsafe { tree_sitter_go() }
            // Language::Html => unsafe { tree_sitter_html() }
            // Language::Ocaml => unsafe { tree_sitter_ocaml() }
            // Language::Php => unsafe { tree_sitter_php() }
            // Language::Ruby => unsafe { tree_sitter_ruby() }
            // Language::Rust => unsafe { tree_sitter_rust() }
            // Language::Typescript => unsafe { tree_sitter_typescript() }
            // Language::Agda => unsafe { tree_sitter_agda() }
            // Language::CSharp => unsafe { tree_sitter_csharp() }
            // Language::Haskell => unsafe { tree_sitter_haskell() }
            // Language::Java => unsafe { tree_sitter_java() }
            // Language::Julia => unsafe { tree_sitter_julia() }
            // Language::Scala => unsafe { tree_sitter_scala() }
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