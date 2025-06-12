use std::io;
use std::process::ExitCode;


fn main() -> ExitCode {
    // Calls process:exit() on incorrect args.
    let settings = settings::parse_from_cmd_line_args();

    let mut aligner = aligner::Aligner::new(settings);
    let stdin = io::stdin();

    for (i, maybe_line) in stdin.lines().enumerate() {
        match maybe_line {
            Ok(line) => {
                aligner.add_line(line);
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
mod settings;
