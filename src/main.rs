use std::io;


#[derive(Debug)]
struct Line {
    text: String,
    number: usize,
    delimiter_index: usize,
}


fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    let mut max_line_number = 0;
    let mut max_index = 0;

    for (i, maybe_line) in stdin.lines().enumerate() {
        let line = maybe_line?;
        if let Some(index) = line.find("=") {
            if index > max_line_number {
                max_line_number = i;
                max_index = index;
            }
            lines.push(Line {
                text: line,
                number: i,
                delimiter_index: index,
            })
        }
    }

    for line in lines {
        let padding_length = max_index - line.delimiter_index;
        let first_half = &line.text[..line.delimiter_index];
        let second_half = &line.text[line.delimiter_index..];
        println!("{}{}{}", first_half, " ".repeat(padding_length), second_half);
    }

    Ok(())
}
