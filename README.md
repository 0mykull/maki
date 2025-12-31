# maki

`maki` is a tiny CLI that combines the ergonomics of `touch` and `mkdir`. Pass any mix of files and directories (use a trailing `/` for directories) and it will create them, optionally applying POSIX permissions and kicking off project initializers like `git init`.

## Build

```bash
cargo install --path .
```

## Usage

```bash
maki src/main.rs app/ --git --mode 755
```

- Files are created or touched in place; parents are created automatically.
- Directories end with `/`; permissions are applied when `--mode <octal>` is provided.
- Optional initializers: `--git`, `--npm`, `--cargo`, `--go` (directories only).
- `-v` prints verbose logs; `-h`/`--help` and `-V`/`--version` show docs/meta.

See `maki --help` for the full flag list.
