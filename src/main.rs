mod structure;
mod parse;

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        println!("{}", line?);
    }

    Ok(())
}
