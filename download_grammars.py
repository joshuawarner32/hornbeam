import os
import shutil
import subprocess

def cmd(cmd, cwd=None):
    print(repr(cwd) + " > " + ' '.join(repr(w) for w in cmd))
    subprocess.run(cmd, cwd=cwd)

def clone_or_update(name, recursive=False):
    target = "scratch/" + name
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
    "haskell",
    "java",
    "julia",
    "scala",
]

os.makedirs("scratch", exist_ok=True)

target = clone_or_update("tree-sitter")
shutil.rmtree("tree-sitter")
shutil.rmtree("parsers")
shutil.copytree(target + "/lib", "tree-sitter")

for g in grammars:
    target = clone_or_update("tree-sitter-" + g)
    os.makedirs("parsers/" + g, exist_ok=True)

    if os.path.exists(target + "/examples"):
        shutil.copytree(target + "/examples", "parsers/" + g + "/examples")

    include(g, "grammar.json")
    include(g, "parser.c")
    include(g, "scanner.c")
    include(g, "scanner.cc")
