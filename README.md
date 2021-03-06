[![Rust](https://github.com/florianfelix/gitini/actions/workflows/rust.yml/badge.svg)](https://github.com/florianfelix/gitini/actions/workflows/rust.yml)

# gitini
Create repository on Github and upload the current folder
## Install
- have rust <a href="https://www.rust-lang.org/tools/install">installed</a>
- `cargo install --locked --git https://github.com/florianfelix/gitini`

## Usage
- <a href="https://github.com/settings/tokens">get token</a> from Github with repo permissions
- run once or `gitini -t TOKEN` to setup new access token
- run `gitini` in folder to create github repo and upload [.gitignore and README.md]
- run `gitini -c` to create and upload everything (possible empty gitignore! Danger!)


## Help Message
USAGE:

    gitini [OPTIONS]

OPTIONS:

    -c, --complete         Init, commit and push everything
    -h, --help             Print help information
    -n, --no_browser       Do not open the repository in browser
    -p, --public           Create a public repository
    -t, --token <TOKEN>    Store the Github API Token
    -V, --version          Print version information
