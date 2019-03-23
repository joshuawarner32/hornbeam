use std::str::FromStr;
use failure::Error;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;
use structopt::StructOpt as StructOptTrait;
use structopt_derive::StructOpt;

use hornbeam::{Language, Parser, Node, Kind, Child, Transform};

#[derive(StructOpt)]
struct ParseArgs {
    #[structopt(long = "lang")]
    lang: Option<Language>,

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

    #[structopt(long = "transform")]
    transform: Option<PathBuf>,

}

fn read_file(path: impl AsRef<Path>) -> Result<String, Error> {
    let mut res = String::new();
    File::open(path)?.read_to_string(&mut res)?;
    Ok(res)
}

fn find_example<'a>(node: Node<'a>, ex: &str) -> Option<Node<'a>> {
    if node.text().contains(ex) {
        for ch in node.children() {
            match ch {
                Child::Node(ch) => {
                    if let Some(n) = find_example(ch, ex) {
                        return Some(n);
                    }
                }
                Child::Text(text) => {}
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
            children: node.nodes().map(Schema::from).collect()
        }
    }

    fn matches(&self, node: &Node) -> bool {
        if self.kind != node.kind() {
            return false;
        }

        let mut mch = self.children.iter();
        let mut nch = node.nodes();

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
                print_children(&tree.root(), 0);
            }
        }
    }
}

enum Tool {
    Action(Parser, Action),
    Transform(Transform),
}

impl Tool {
    fn from_args(args: &ParseArgs) -> Tool {
        if let Some(lang) = &args.lang {
            let mut parser = Parser::new(*lang);
            let action = Action::from_args(&mut parser, &args);

            return Tool::Action(parser, action);
        }

        if let Some(transform) = &args.transform {
            return Tool::Transform(Transform::load(transform));
        }
        panic!();
    }
}

fn print_children<'a>(node: &Node<'a>, indent: usize) {
    println!("{:indent$}Begin {:?}", "", node.kind(), indent=indent*2);
    for ch in node.children() {
        match ch {
            Child::Node(ch) => print_children(&ch, indent + 1),
            Child::Text(text) => println!("{:indent$}Text {:?}", "", text, indent=(indent + 1)*2),
        }
    }
    println!("{:indent$}End {:?}", "", node.kind(), indent=indent*2);
}

fn find_lang(text: &str) -> Language {
    let l = text.find("lang:").unwrap();
    let t = text[l + "lang:".len()..].trim();
    Language::from_str(t).unwrap()
}

fn main() {
    let args = ParseArgs::from_args();

    let tool = Tool::from_args(&args);

    match tool {
        Tool::Action(mut parser, action) => {
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
        Tool::Transform(transform) => {
            // TODO
        }
    }
}
