[![Crates.io](https://img.shields.io/crates/v/rvi?style=flat-square)](https://crates.io/crates/rvi)
[![rvi](https://github.com/bujnlc8/rvi/actions/workflows/rvi.yml/badge.svg?branch=master)](https://github.com/bujnlc8/rvi/actions/workflows/rvi.yml)

A small tool to let you write rust with `vim` and `rust-analyzer` not using `cargo` to create a project, we just want to play with a single rust file!

- Suppose you use [vim](https://github.com/vim/vim) and [rust analyzer](https://github.com/rust-lang/rust-analyzer).
- When you use vim and rust-analyzer to edit single rust file, you will find rust-analyzer do not work well, code completion and code check are broken.
- Then, you must add `rust-project.json` in your file's directory, as the [manual](https://rust-analyzer.github.io/manual.html#non-cargo-based-projects) shows. At this time, code completion will work, but code check is still broken. it just complains **could not find Cargo.toml in / or any parent directory**, so you must add a `Cargo.toml` in your file's directory, like this:

<!---->

    [package]
    name = "test"
    version = "0.1.0"
    edition = "2021"

    [lib]
    path= "test.rs"

    [dependencies]
    ..

- Then you will find, every time you edit a diffrent rust file, you must replace the field `root_module` in `rust-project.json` and the field `path` in `Cargo.toml` to your editing rust file's name, it's so boring and waste of time.

- This tool will help you, you just need `rvi your_file_name.rs`!

## Install

Download from [release](https://github.com/bujnlc8/rvi/releases) page

OR

```shell
cargo install rvi
```

## Configure

1.Copy [`rvi.toml`](https://github.com/bujnlc8/rvi/blob/master/rvi.toml) into `$HOME/.config/`, you can edit it according to your actual environment.

2.If you use `Youcompleteme`, you must fill the `ycm_extra_conf` field in above file, the config_file is like [`.ycm_extra_conf.py.example`](https://github.com/bujnlc8/rvi/blob/master/.ycm_extra_conf.py.example), you should place it in your directory and rename it to `.ycm_extra_conf.py`. The tool will copy it to your
project_dir automatically.

3.If you do not use `Youcompleteme`, you should transfer the `rust-analyzer.checkOnSave.overrideCommand` configuration item to `rust-analyzer` lsp as your client explain. The following is official description:

> Note that calls to cargo check are disabled when using rust-project.json by default, so compilation errors and warnings will no longer be sent to your LSP client. To enable these compilation errors you will need to specify explicitly what command rust-analyzer should run to perform the checks using the checkOnSave.overrideCommand configuration. As an example, the following configuration explicitly sets cargo check as the checkOnSave command.

```json
{
  "rust-analyzer.checkOnSave.overrideCommand": [
    "cargo",
    "check",
    "--message-format=json"
  ]
}
```

> The checkOnSave.overrideCommand requires the command specified to output json error messages for rust-analyzer to consume. The --message-format=json flag does this for cargo check so whichever command you use must also output errors in this format. See the [Configuration](https://rust-analyzer.github.io/manual.html#configuration) section for more information.

## Usage

```
rvi your_file.rs
```
