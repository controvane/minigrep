# minigrep

A tiny grep clone written in Rust, built for learning. Searches files and
directories for matching lines, parallelized with rayon for directory-wide
searches.

## Building

```bash
cargo build --release
```

The release binary is produced at `target/release/mgrp`. Copy it into the
tracked `release/` directory when you want to distribute it:

```bash
cargo build --release
cp target/release/mgrp release/mgrp
```

## Usage

```
mgrp [OPTIONS]
```

### Options

| Flag       | Description                                                                 |
|------------|-----------------------------------------------------------------------------|
| `-f PATH`  | File or directory to search. Optional — defaults to the current directory.  |
| `-c TERMS` | "Or" search terms, `\|`-separated. Lines matching at least one are shown.    |
| `-eq TERMS`| "And" search terms, `\|`-separated. Lines matching all are shown.            |
| `-ne TERMS`| Excluded terms, `\|`-separated. Matching lines are hidden.                   |
| `-i`       | Case-insensitive search.                                                    |
| `-a N`     | Print N lines after each match as context.                                  |
| `-b N`     | Print N lines before each match as context.                                 |
| `-h`       | Print help.                                                                |
| `-n`     | Show line number together with the line                                     |
| `-t`     | List of file types to include in search. If passing a file with -f or piping output of other program, this flag is ignored. separated by `\|` |

At least one of `-c`, `-eq`, or `-ne` is required.

If `-f` is omitted and stdin is not a terminal, input is read from stdin
instead.

### Examples

```bash
# Case-insensitive search for "nobody" in a single file
mgrp -c nobody -i -f poem.txt

# "Or" search: lines containing "fn" or "pub" in all files under src/
mgrp -c "fn|pub" -f src

# "And" search: lines containing both "fn" and "search"
mgrp -eq "fn|search" -f src

# Exclude lines with a term
mgrp -c fn -ne deprecated -f src

# Search piped stdin
cat poem.txt | mgrp -c nobody -i

# Show 2 lines of context before and after each match
mgrp -c nobody -b 2 -a 2 -f poem.txt
```

## Installation

Install the release binary to `~/.local/bin` and ensure it is on your PATH:

```bash
./install.sh
```

This copies `release/mgrp` to `~/.local/bin/mgrp` and appends the
`~/.local/bin` PATH export to your shell configuration (`.bashrc`,
`.bash_profile`, or `.zshrc`) if it is not already present. It is safe to
re-run after updating the binary — it overwrites the installed copy in place.

> Requires a release binary in `release/mgrp` first. After rebuilding, copy
> it there with `cp target/release/mgrp release/mgrp`.

The uninstall script also removes a previously installed `mgrep` binary, so it
is safe to run after upgrading from the old name.

## Uninstall

```bash
./uninstall.sh
```

Removes the binary from `~/.local/bin` and strips the PATH export line that
`install.sh` added. Your shell configuration is otherwise left untouched.

## How it works

- `walk` recursively collects files, skipping symlinks.
- it also filters the filetypes if any are given. Else, the list of paths is just kept as is.
- The file list is searched in parallel using rayon's thread pool.
- Results are grouped per file and printed as they complete; files with no
  matches are omitted.
- Searching is memory-bounded: each file is streamed line-by-line, never
  loaded wholly into memory.
- Context lines (`-a`/`-b`) expand each match with the surrounding lines, kept
  in a small sliding buffer; disjoint groups of output are separated by `--`.
