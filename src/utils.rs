use std::fs::{File, OpenOptions};
use std::io;
use std::io::{stdin, stdout, Write};
use std::path::Path;

pub fn read_input() -> String {
    let mut s = String::new();
    print!("Enter your Github API Token: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

// A simple implementation of `% touch path` (ignores existing files)
pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
