use crate::settings::Settings;


pub struct Aligner {
    delimiter: String,
    after: bool,
    lines: Vec<Line>,
    max_length: usize,
}

impl Aligner {
    pub fn new(settings: Settings) -> Self {
        if settings.delimiter.len() == 0 {
            // This should be checked when parsing the command line arguments.
            panic!("delimiter cannot be empty");
        }
        Self {
            delimiter: settings.delimiter,
            after: settings.after,
            lines: Vec::new(),
            max_length: 0,
        }
    }

    pub fn add_line(&mut self, line: String) {
        let maybe_index = 
        line.find(&self.delimiter).and_then(|index| {
            if self.after {
                let s = self.delimiter.len();
                line[(index+s)..].find(|c: char| !c.is_whitespace())
                                 .map(|i| index + i + s)
                // example (delimiter is '::', first non-whitespace after it is 'd'):
                // 
                // abc::  def
                // 0123456789
                //      01234
                // 3 + 2 + 2 = 7
            }
            else {
                Some(index)
            }
        });

        let maybe_length = maybe_index.and_then(|i| num_chars_before_index(&line, i));

        let delimiter =
        if let Some(index) = maybe_index {
            Some(Delimiter {
                index,
                length: maybe_length.expect("if index is not None then length should not be None either"),
            })
        }
        else {
            None
        };

        self.lines.push(Line {
            text: line,
            delimiter,
        });

        if let Some(length) = maybe_length {
            if length > self.max_length {
                self.max_length = length;
            }
        }
    }

    pub fn aligned_lines(self) -> impl Iterator<Item = String> {
        self.lines.into_iter().map(move |line| {
            if let Some(delimiter) = line.delimiter {
                let padding_length = self.max_length - delimiter.length;
                let first_half     = &line.text[..delimiter.index];
                let second_half    = &line.text[delimiter.index..];
                format!("{}{}{}", first_half, " ".repeat(padding_length), second_half)
            }
            else {
                line.text
            }
        })
    }
}


fn num_chars_before_index(string: &str, index: usize) -> Option<usize> {
    let mut n = 0;
    for (byte, _char) in string.char_indices() {
        if byte == index {
            return Some(n);
        }
        n += 1;
    }
    return None;
}


struct Delimiter {
    index: usize,
    length: usize,
}


struct Line {
    text: String,
    delimiter: Option<Delimiter>,
}


#[cfg(test)]
mod tests;
