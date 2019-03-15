use std::str::FromStr;
use failure::Error;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;
use structopt::StructOpt as StructOptTrait;
use structopt_derive::StructOpt;

use hornbeam::{Language, Parser, Node, Kind, Child};

#[derive(StructOpt)]
struct ParseArgs {
    #[structopt(long = "lang")]
    lang: Language,

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
    Transform(Transform),
}

impl Action {
    fn from_args(parser: &mut Parser, args: &ParseArgs) -> Action {
        if args.replay {
            return Action::Replay;
        }
        if args.tree {
            return Action::Tree;
        }
        if let Some(tform_path) = &args.transform {
            return Action::Transform(parse_transform(&read_file(tform_path).unwrap()));
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
            Action::Transform(_) => {
                panic!()
            }
        }
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

#[derive(Debug)]
struct LangTree {
    lang: Language,
    source: String,
}

#[derive(Debug)]
struct Example {
    from: LangTree,
    to: LangTree,
}

#[derive(Debug)]
struct Transform {
    examples: Vec<Example>,
}

fn parse_transform(text: &str) -> Transform {
    let mut newlines = vec![0];

    newlines.extend(text.chars().enumerate()
        .filter_map(|(i, ch)| if ch == '\n' { Some(i) } else { None }));

    if newlines.last() != Some(&(text.len() - 1)) {
        newlines.push(text.len());
    }

    #[derive(Debug)]
    enum Chunk<'a> {
        From(Language),
        To(Language),
        Text(&'a str),
    }

    let mut last = 0;
    let mut chunks = Vec::new();

    for p in newlines.windows(2) {
        let t = &text[p[0]..p[1]];
        if t.contains("--from") {
            chunks.push(Chunk::Text(&text[last..p[0]]));
            chunks.push(Chunk::From(find_lang(t)));
            last = p[1];
        } else if t.contains("--to") {
            chunks.push(Chunk::Text(&text[last..p[0]]));
            chunks.push(Chunk::To(find_lang(t)));
            last = p[1];
        }
    }
    chunks.push(Chunk::Text(&text[last..]));

    let mut examples = Vec::new();

    for c in chunks.windows(4) {
        match (&c[0], &c[1], &c[2], &c[3]) {
            (Chunk::From(from_lang), Chunk::Text(from_text), Chunk::To(to_lang), Chunk::Text(to_text)) => {
                examples.push(Example {
                    from: LangTree {
                        lang: *from_lang,
                        source: from_text.to_string(),
                    },
                    to: LangTree {
                        lang: *to_lang,
                        source: to_text.to_string(),
                    },
                });
            }
            _ => {}
        }
    }

    for ex in examples {
        println!("{:?}", ex);
    }

    panic!();
}

fn find_lang(text: &str) -> Language {
    let l = text.find("lang:").unwrap();
    let t = text[l + "lang:".len()..].trim();
    Language::from_str(t).unwrap()
}

fn main() {
    let args = ParseArgs::from_args();

    let mut parser = Parser::new(args.lang);

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
