use std::fs::File;
use std::io;
use std::path::Path;
use io::BufReader;
use io::Result;
use std::io::BufRead;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_input(filename: impl AsRef<Path>) -> Result<impl Iterator<Item=String>> {
    let file = File::open(Path::new("src/bin/inputs").join(filename).with_extension("txt"))?;
    Ok(BufReader::new(file).lines().map(|line| line.unwrap()))
}
