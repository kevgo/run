<picture>
  <source media="(prefers-color-scheme: dark)" srcset="web/logo_800_dark.png">
  <source media="(prefers-color-scheme: light)" srcset="web/logo_800_light.png">
  <img alt="Atalanta logo" src="documentation/logo_800_light.png">
</picture>

Software development requires housekeeping activities like installing
dependencies, running compilers, linters, automated tests, formatters, etc. If
you work on a larger number of code bases, remembering how to use a variety of
developer tools to run a variety of custom scripts and activities can get
tedious. Atalanta &ndash; named after the Greek godess of running &ndash; knows
a wide range of software stacks and runs these housekeeping activities for you.

Currently supported stacks:

- [Make](https://www.gnu.org/software/make)
- [Node.js](https://nodejs.org) using [npm](https://github.com/npm/cli)
- [Node.js](https://nodejs.org) using [yarn](https://yarnpkg.com)

[![CI](https://github.com/kevgo/atalanta/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/atalanta/actions/workflows/ci.yml)

## Installation

Install [Rust](https://www.rust-lang.org) via [RustUp](https://rustup.rs). Then
install Atalanta:

```
cargo install https://github.com/kevgo/atalanta
```

This installs a binary called `a` into the `~/.cargo/bin` folder. Please add it
to your `$PATH`.

To set up the auto-completions for [Fish shell](https://fishshell.com), add this
to `~/.config/fish/config.fish`:

```
if test -f ~/.cargo/bin/a
  ~/.cargo/bin/a --print-fish-completions | source
end
```

## Usage

Use Atalanta inside a directory containing code. Running it without any
parameters shows all recognized software stacks and available tasks for them:

```
a
```

Use the `--setup` or `-s` command to install dependencies for your codebase:

```
a -s
```

If your codebase contains a file `package.json` that defines a `test` script,
you no longer have to run `npm run test` but can do this now:

```
a test
```

You can abbreviate task names as long as they uniquely match a task. To run the
test script again:

```
a t
```
