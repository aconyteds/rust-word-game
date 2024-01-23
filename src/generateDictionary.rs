use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("./dictionary.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut words: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.len() == 5 {
            words.push(line.to_string());
        }
    }

    let output = format!("pub const WORDS: [&str; {}] = {:?};", words.len(), words);
    let mut output_file = File::create("./src/game/words.rs")?;
    write!(output_file, "{}", output)?;

    Ok(())
}
