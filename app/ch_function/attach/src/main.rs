pub mod format;

use std::io::{self, BufRead, Write};
use crate::format::format;

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };


        let result = match format(line.clone()) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("e:{}, row:{}", e, line.clone());
                break
            },
        };

        if let Err(err) = writeln!(out, "{}", result) {
            panic!("Error: {}", err);
        }
    }
    out.flush()?;

    Ok(())
}