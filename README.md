# minigrep

A tiny grep clone written in Rust, built for learning. Searches files and
directories for matching lines, parallelized with rayon for directory-wide
searches.

## Building

```bash
cargo build --release
```

The release binary is produced at `target/release/mgrep`. Copy it into the
tracked `release/` directory when you want to distribute it:

```bash
cargo build --release
cp target/release/mgrep release/mgrep
```

## Usage

```
mgrep [OPTIONS]
```

### Options

| Flag       | Description                                                                 |
|------------|-----------------------------------------------------------------------------|
| `-f PATH`  | File or directory to search. Optional — defaults to the current directory.  |
| `-c TERMS` | "Or" search terms, `\|`-separated. Lines matching at least one are shown.    |
| `-eq TERMS`| "And" search terms, `\|`-separated. Lines matching all are shown.            |
| `-ne TERMS`| Excluded terms, `\|`-separated. Matching lines are hidden.                   |
| `-i`       | Case-insensitive search.                                                    |
| `-h`       | Print help.                                                                |

At least one of `-c`, `-eq`, or `-ne` is required.

If `-f` is omitted and stdin is not a terminal, input is read from stdin
instead.

### Examples

```bash
# Case-insensitive search for "nobody" in a single file
mgrep -c nobody -i -f poem.txt

# "Or" search: lines containing "fn" or "pub" in all files under src/
mgrep -c fn|pub -f src

# "And" search: lines containing both "fn" and "search"
mgrep -eq fn|search -f src

# Exclude lines with a term
mgrep -c fn -ne deprecated -f src

# Search piped stdin
cat poem.txt | mgrep -c nobody -i
```

## Installation

Install the release binary to `~/.local/bin` and ensure it is on your PATH:

```bash
./install.sh
```

This copies `release/mgrep` to `~/.local/bin/mgrep` and appends the
`~/.local/bin` PATH export to your shell configuration (`.bashrc`,
`.bash_profile`, or `.zshrc`) if it is not already present. It is safe to
re-run after updating the binary — it overwrites the installed copy in place.

> Requires a release binary in `release/mgrep` first. After rebuilding, copy
> it there with `cp target/release/mgrep release/mgrep`.

## Uninstall

```bash
./uninstall.sh
```

Removes the binary from `~/.local/bin` and strips the PATH export line that
`install.sh` added. Your shell configuration is otherwise left untouched.

## How it works

- `walk` recursively collects files, skipping symlinks.
- The file list is searched in parallel using rayon's thread pool.
- Results are grouped per file and printed as they complete; files with no
  matches are omitted.
- Searching is memory-bounded: each file is streamed line-by-line, never
  loaded wholly into memory.
