use std::io;


fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut aligner = alignby::Aligner::new("=");

    for (i, maybe_line) in stdin.lines().enumerate() {
        aligner.add_line(maybe_line?, i);
    }

    for line in aligner.aligned_lines() {
        println!("{}", line);
    }

    Ok(())
}


mod alignby;
