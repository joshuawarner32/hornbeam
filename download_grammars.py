import os
import shutil
import subprocess
import argparse


parser = argparse.ArgumentParser(description='Download and regenerate parsers')
parser.add_argument('--skip-git', dest='skip_git', action='store_const', const=True, default=False)
args = parser.parse_args()

def cmd(cmd, cwd=None):
    print(repr(cwd) + " > " + ' '.join(repr(w) for w in cmd))
    subprocess.run(cmd, cwd=cwd, check=True)

def clone_or_update(name, recursive=False):
    target = "scratch/" + name
    if args.skip_git:
        return target
    if not os.path.exists(target):
        cmd([
            "git",
            "clone",
            "git://github.com/tree-sitter/" + name,
            target,
        ])
    else:
        cmd(["git", "reset", "--hard", "HEAD"], cwd=target)
        cmd(["git", "pull"], cwd=target)
    if recursive:
        cmd(["git", "submodule", "update", "--init"], cwd=target)
    return target

def include(g, f):
    s = "scratch/tree-sitter-" + g + "/src/" + f
    if os.path.exists(s):
        shutil.copy(s, "parsers/" + g + "/" + f)

grammars = [
    "javascript",
    "python",
    "rust",
    "bash",
    "c",
    "cpp",
    "css",
    "embedded-template",
    "go",
    "html",
    "ocaml",
    "php",
    "ruby",
    "typescript",
    "agda",
    "c-sharp",
    # "haskell",
    # "java",
    "julia",
    "scala",
]

os.makedirs("scratch", exist_ok=True)

tool_target = clone_or_update("tree-sitter", recursive=True)
shutil.rmtree("tree-sitter")
shutil.copytree(tool_target + "/lib", "tree-sitter")

cmd(['cargo', 'build', '--release'], cwd=tool_target)
tool = os.path.abspath(tool_target + '/target/release/tree-sitter')
assert os.path.isfile(tool)

shutil.rmtree("parsers")
for g in grammars:
    target = clone_or_update("tree-sitter-" + g)
    os.makedirs("parsers/" + g, exist_ok=True)

    # if os.path.exists(target + "/examples"):
    #     shutil.copytree(target + "/examples", "parsers/" + g + "/examples")

    cmd([tool, 'generate', 'src/grammar.json'], cwd=target)

    include(g, "grammar.json")
    include(g, "parser.c")
    include(g, "node-types.json")
    include(g, "scanner.c")
    include(g, "scanner.cc")
    include(g, "tag.h")
