use std::io;
use std::process::ExitCode;


fn main() -> ExitCode {
    let stdin = io::stdin();
    let mut aligner = aligner::Aligner::new("=");

    for (i, maybe_line) in stdin.lines().enumerate() {
        match maybe_line {
            Ok(line) => {
                aligner.add_line(line, i);
            },
            Err(err) => {
                eprintln!("Failed to read stdin at line {}:", i);
                eprintln!("{}", err);
                return ExitCode::FAILURE;
            },
        }
    }

    for line in aligner.aligned_lines() {
        println!("{}", line);
    }

    ExitCode::SUCCESS
}


mod aligner;
