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
}

// fn iter_files(dir: impl AsRef<Path>) -> impl Iterator<Item=(PathBuf, u64)> {
//     WalkDir::new(dir).into_iter()
//         .filter_map(|e| {
//             let e = e.unwrap();
//             let md = e.metadata().unwrap();
//             if md.file_type().is_file() {
//                 Some((e.path().to_owned(), md.len()))
//             } else {
//                 None
//             }
//         })
// }

// fn iter_file_strings(dir: impl AsRef<Path>) -> impl Iterator<Item=(PathBuf, String)> {
//     iter_files(dir).filter_map(|(file, _size)| {
//         let mut data = Vec::new();
//         File::open(&file).unwrap().read_to_end(&mut data).unwrap();
//         match String::from_utf8(data) {
//             Ok(text) => Some((file, text)),
//             Err(_) => None,
//         }
//     })
// }

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

fn main() {
    let args = ParseArgs::from_args();

    let mut parser = Parser::new(args.lang.0);

    if args.show_kinds {
        for kind in parser.info.kind_names() {
            println!("{}", kind);
        }
    }

    if let Some(file) = args.file {
        let text = read_file(file).unwrap();

        if args.replay {
            let text = text.replace('\n', " ");
            for i in 0..text.len() + 1 {
                let prefix = &text[0..i];
                println!("{} {:?}", prefix, parser.parse(&prefix));
            }
        }

        let tree = parser.parse(&text);

        if let Some(kind) = args.kind {
            let kind = parser.info.kind_from_name(&kind).unwrap();
            for node in tree.nodes() {
                if node.kind() == kind {
                    println!("{}", node.text());
                }
            }
        }

        if let Some(example) = args.example {
            let full = if let Some(context) = args.context {
                context.replace("@@", &example)
            } else {
                example.clone()
            };

            let ex = parser.parse(&full);
            let ex = find_example(ex.root(), &example).unwrap();
            println!("syntax: {:?}", ex);
            let schema = Schema::from(ex);

            for node in tree.nodes() {
                if schema.matches(&node) {
                    println!("{}", node.text());
                }
            }
        }
    }
}