use std::str::FromStr;
use failure::{Error, format_err};
use std::path::{Path, PathBuf};
// use walkdir::WalkDir;
use std::io::Read;
use std::fs::File;
use structopt::StructOpt as StructOptTrait;
use structopt_derive::StructOpt;

use hornbeam::{Language, Parser, Node, Kind};

struct LangArg(Language);

impl FromStr for LangArg {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Error> {
        Ok(LangArg(match text {
            "javascript" => Language::Javascript,
            "python" => Language::Python,
            "rust" => Language::Rust,
            "bash" => Language::Bash,
            "c" => Language::C,
            "cpp" => Language::Cpp,
            "css" => Language::Css,
            "go" => Language::Go,
            "html" => Language::Html,
            "ocaml" => Language::Ocaml,
            "php" => Language::Php,
            "ruby" => Language::Ruby,
            "typescript" => Language::Typescript,
            "agda" => Language::Agda,
            "c-sharp" => Language::CSharp,
            "haskell" => Language::Haskell,
            "java" => Language::Java,
            "julia" => Language::Julia,
            "scala" => Language::Scala,
            _ => return Err(format_err!("invalid language '{}'", text))
        }))
    }
}

#[derive(StructOpt)]
struct ParseArgs {
    #[structopt(long = "lang")]
    lang: LangArg,

    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,

    #[structopt(short = "s", long = "show-kinds")]
    show_kinds: bool,

    #[structopt(long = "replay")]
    replay: bool,

    #[structopt(short = "e", long = "example")]
    example: Option<String>,

    #[structopt(short = "c", long = "context")]
    context: Option<String>,

    #[structopt(short = "k", long = "kind")]
    kind: Option<String>,

    #[structopt(short = "g", long = "glob")]
    glob: Option<String>,

    #[structopt(short = "t", long = "tree")]
    tree: bool,
}

fn read_file(path: impl AsRef<Path>) -> Result<String, Error> {
    let mut res = String::new();
    File::open(path)?.read_to_string(&mut res)?;
    Ok(res)
}

fn find_example<'a>(node: Node<'a>, ex: &str) -> Option<Node<'a>> {
    if node.text().contains(ex) {
        for ch in node.children() {
            if let Some(n) = find_example(ch, ex) {
                return Some(n);
            }
        }
        Some(node.clone())
    } else {
        None
    }
}

#[derive(Eq, PartialEq)]
struct Schema {
    kind: Kind,
    children: Vec<Schema>,
}

impl Schema {
    fn from(node: Node) -> Schema {
        Schema {
            kind: node.kind(),
            children: node.children().map(Schema::from).collect()
        }
    }

    fn matches(&self, node: &Node) -> bool {
        if self.kind != node.kind() {
            return false;
        }

        let mut mch = self.children.iter();
        let mut nch = node.children();

        loop {
            match (mch.next(), nch.next()) {
                (Some(m), Some(n)) => {
                    if !m.matches(&n) {
                        return false;
                    } else {
                        // fallthrough
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

enum Finder {
    Kind(Kind),
    Schema(Schema),
}

impl Finder {
    fn matches<'a>(&self, node: &Node<'a>) -> bool {
        match self {
            Finder::Kind(k) => node.kind() == *k,
            Finder::Schema(s) => s.matches(node),
        }
    }
    fn from_args(parser: &mut Parser, args: &ParseArgs) -> Finder {
        if let Some(kind) = &args.kind {
            let kind = parser.info.kind_from_name(&kind).unwrap();
            return Finder::Kind(kind);
        }

        if let Some(example) = &args.example {
            let full = if let Some(context) = &args.context {
                context.replace("@@", &example)
            } else {
                example.clone()
            };

            let ex = parser.parse(&full);
            let ex = find_example(ex.root(), &example).unwrap();
            println!("syntax: {:?}", ex);
            let schema = Schema::from(ex);

            return Finder::Schema(schema)
        }

        panic!();
    }
}

enum Action {
    Replay,
    Find(Finder),
    Tree,
}

impl Action {
    fn from_args(parser: &mut Parser, args: &ParseArgs) -> Action {
        if args.replay {
            return Action::Replay;
        }
        if args.tree {
            return Action::Tree;
        }
        Action::Find(Finder::from_args(parser, args))
    }

    fn apply(&self, parser: &mut Parser, text: &str) {
        match self {
            Action::Replay => {
                let text = text.replace('\n', " ");
                for i in 0..text.len() + 1 {
                    let prefix = &text[0..i];
                    println!("{} {:?}", prefix, parser.parse(&prefix));
                }
            }
            Action::Find(finder) => {
                let tree = parser.parse(&text);

                for node in tree.nodes() {
                    if finder.matches(&node) {
                        println!("{}", node.text());
                    }
                }
            }
            Action::Tree => {
                let tree = parser.parse(&text);
                print_node(&tree.root(), 0);
            }
        }
    }
}

fn print_node<'a>(node: &Node<'a>, indent: usize) {
    println!("{:indent$}Begin {:?}", "", node.kind(), indent=indent*2);
    for ch in node.children() {
        print_node(&ch, indent + 1);
    }
    println!("{:indent$}End {:?}", "", node.kind(), indent=indent*2);
}

fn main() {
    let args = ParseArgs::from_args();

    let mut parser = Parser::new(args.lang.0);

    if args.show_kinds {
        for kind in parser.info.kind_names() {
            println!("{}", kind);
        }
    }

    let action = Action::from_args(&mut parser, &args);

    if let Some(file) = &args.file {
        let text = read_file(file).unwrap();
        action.apply(&mut parser, &text);
    }

    if let Some(g) = &args.glob {
        for entry in glob::glob(g).unwrap() {
            let entry = entry.unwrap();
            let text = read_file(&entry).unwrap();
            action.apply(&mut parser, &text);
        }
    }
}
