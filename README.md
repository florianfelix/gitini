# gitify
Create repository on Github and upload the current folder
## Install
- have rust <a href="https://www.rust-lang.org/tools/install">installed</a>
- `cargo install --locked --git https://github.com/florianfelix/gitify`

## Usage
- get token from <a href="https://github.com/settings/tokens">Github</a> with repo permissions
- run once or `gitify -t TOKEN` to setup new access token
- run gitify in folder to create github repo and upload

## Help Message
USAGE:

    gitify [OPTIONS]

OPTIONS:

    -c, --complete         init, commit and push everything
    -h, --help             Print help information
    -p, --public           Create a public repository
    -t, --token <TOKEN>    Store the Github API Token
    -V, --version          Print version information
