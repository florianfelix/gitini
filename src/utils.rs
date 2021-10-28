use std::io::{stdin, stdout, Write};

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
