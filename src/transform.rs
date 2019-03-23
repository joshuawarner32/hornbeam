use crate::parse::{Parser, Kind, Language, Node, Child};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Repeat {
    Single,
    Optional,
    AtLeastOne,
    Many,
}

#[derive(Debug)]
pub struct Variadic {
    pattern: Pattern,
    repeat: Repeat,
}

#[derive(Debug)]
pub enum Pattern {
    Node {
        kind: Kind,
        children: Vec<Variadic>,
    },
    TextLiteral(String),
    TextVar,
    NodeVar,
}

#[derive(Debug)]
pub struct Rule {
    pattern: Pattern,
    output: Pattern,
}

#[derive(Debug)]
pub struct Program {
    from_lang: Language,
    to_lang: Language,
    rules: Vec<Rule>,
}

impl Variadic {
    fn parse_children<'a>(node: &Node<'a>, vars: &[&str]) -> Vec<Variadic> {
        node.children().map(|ch| {
            let pattern = match ch {
                Child::Node(n) => Pattern::parse(&n, vars),
                Child::Text(text) => {
                    if vars.iter().any(|v| *v == text) {
                        Pattern::TextVar
                    } else {
                        Pattern::TextLiteral(text.to_string())
                    }
                }
            };

            Variadic {
                repeat: Repeat::Single,
                pattern,
            }
        }).collect()
    }

    fn replace(&self, vars: &Option<&str>) -> String {
        assert!(self.repeat == Repeat::Single);
        self.pattern.replace(vars)
    }
}

fn has_var<'a>(node: &Node<'a>, vars: &[&str]) -> bool {
    vars.iter().any(|v| node.text().contains(v))
}

fn has_single_var<'a>(node: &Node<'a>, vars: &[&str]) -> bool {
    if has_var(node, vars) {
        if node.nodes().any(|n| has_var(&n, vars)) {
            false
        } else {
            true
        }
    } else {
        false
    }
}

// fn is_text_var<'a>(node: &Node<'a>, vars: &[&str]) -> bool {
//     false
// }

impl Pattern {
    fn parse<'a>(node: &Node<'a>, vars: &[&str]) -> Pattern {
        if has_single_var(node, vars) {
            // if is_text_var(node, vars) {
            //     Pattern::TextVar
            // } else {
                Pattern::NodeVar
            // }
        } else {
            Pattern::Node {
                kind: node.kind(),
                children: Variadic::parse_children(node, vars),
            }
        }
    }

    fn check<'a>(&self, node: &Node<'a>, vars: &mut Option<&'a str>) -> bool {
        println!("checking {:?} vs {:?}", self, node);
        match self {
            Pattern::Node { kind, children, } => {
                if node.kind() == *kind {
                    let mut it = node.children();

                    for ch in children {
                        assert!(ch.repeat == Repeat::Single);
                        if let Some(nch) = it.next() {
                            match nch {
                                Child::Node(nch) => {
                                    if !ch.pattern.check(&nch, vars) {
                                        dbg!("case");
                                        return false;
                                    }
                                }
                                Child::Text(text) => {
                                    if let Pattern::TextLiteral(lit) = &ch.pattern {
                                        if text != lit {
                                            return false;
                                        }
                                    }
                                    // TODO: TextVar
                                }
                            }
                        } else {
                            dbg!("case");
                            return false;
                        }
                    }

                    if it.next().is_some() {
                        dbg!("case");
                        return false;
                    }

                    true
                } else {
                    dbg!("case");
                    false
                }
            }
            Pattern::TextLiteral(text) => node.text() == text,
            Pattern::TextVar => {
                panic!();
            }
            Pattern::NodeVar => {
                if vars.is_some() {
                    panic!();
                } else {
                    *vars = Some(node.text());
                }
                true
            }
        }
    }

    fn replace(&self, vars: &Option<&str>) -> String {
        match self {
            Pattern::Node { kind: _, children, } => {
                let mut text = String::new();
                for ch in children {
                    text.push_str(&ch.replace(vars));
                }
                text
            }
            Pattern::TextLiteral(text) => text.to_string(),
            Pattern::TextVar => {
                panic!();
            }
            Pattern::NodeVar => vars.unwrap().to_string(),
        }
    }
}

impl Rule {
    fn parse<'a>(from: &Node<'a>, to: &Node<'a>, vars: &[&str]) -> Rule {
        Rule {
            pattern: Pattern::parse(from, vars),
            output: Pattern::parse(to, vars),
        }
    }

    fn check(&self, node: &Node) -> Option<String> {
        let mut vars = None;

        if self.pattern.check(node, &mut vars) {
            Some(self.output.replace(&vars))
        } else {
            None
        }
    }
}

impl Program {
    pub fn parse(from_lang: Language, to_lang: Language, from: &str, to: &str, vars: &[&str]) -> Program {
        let mut from_parser = Parser::new(from_lang);
        let from_tree = from_parser.parse(from);

        let mut to_parser = Parser::new(to_lang);
        let to_tree = to_parser.parse(to);

        let rule = Rule::parse(&from_tree.root(), &to_tree.root(), vars);

        Program {
            from_lang,
            to_lang,
            rules: vec![rule],
        }
    }

    pub fn apply(&self, text: &str) -> Option<String> {
        let mut parser = Parser::new(self.from_lang);
        let tree = parser.parse(text);
        let root = &tree.root();

        for rule in &self.rules {
            if let Some(res) = rule.check(root) {
                // TODO: check that result parses correctly under self.to_lang
                return Some(res)
            }
        }
        None
    }
}

pub struct Transform {
    program: Program,
}

#[derive(Debug)]
pub struct Example {
    label: String,
    group: String,
    lang: Language,
    filename: String,
    text: String,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ExampleId(usize);

#[derive(Debug)]
pub struct Examples {
    list: Vec<Example>,
    by_label: HashMap<String, Vec<ExampleId>>,
    by_group: HashMap<String, Vec<ExampleId>>,
}

impl Transform {
    pub fn load(path: impl AsRef<Path>) -> Transform {
        let files = fs::read_dir(path.as_ref()).unwrap()
            .map(|e| e.unwrap().file_name().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        let mut list = Vec::new();
        let mut by_label = HashMap::new();
        let mut by_group = HashMap::new();

        for file in files {
            let dash = file.rfind('-').unwrap();
            let ext = file.rfind('.').unwrap();
            let label = &file[..dash];
            let group = &file[dash + 1..ext];
            let ext = &file[ext + 1..];

            let mut text = String::new();

            File::open(path.as_ref().join(&file)).unwrap().read_to_string(&mut text).unwrap();

            let ex = Example {
                label: label.to_string(),
                group: group.to_string(),
                lang: Language::from_extension(ext).unwrap(),
                filename: file.clone(),
                text,
            };

            let id = ExampleId(list.len());
            list.push(ex);

            by_label.entry(label.to_string()).or_insert_with(Vec::new)
                .push(id);

            by_group.entry(group.to_string()).or_insert_with(Vec::new)
                .push(id);
        }

        let examples = Examples { list, by_label, by_group };

        for (_label, exes) in &examples.by_label {
            for ex in exes {
                println!("{:?}", examples.list[ex.0]);
            }
        }

        let mut parsers = examples.list.iter().map(|e| e.lang).collect::<HashSet<_>>()
            .into_iter().map(|l| (l, Parser::new(l))).collect::<HashMap<Language, Parser>>();

        for ex in &examples.list {
            let tree = parsers.get_mut(&ex.lang).unwrap().parse(&ex.text);
            dbg!(tree);
        }

        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {
        let p = Program::parse(Language::Rust, Language::Rust, "fn a() {}", "fn a();", &["a"]);
        let r = p.apply("fn abcd() {}");
        assert_eq!(r, Some(String::from("fn abcd();")));
    }

    #[test]
    fn test_pattern_cross() {
        let p = Program::parse(Language::Rust, Language::Javascript, "fn a() {}", "function a() {}", &["a"]);
        let r = p.apply("fn abcd() {}");
        assert_eq!(r, Some(String::from("function abcd() {}")));
    }
}
