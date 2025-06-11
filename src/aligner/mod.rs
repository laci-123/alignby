pub struct Aligner {
    delimiter: String,
    lines: Vec<Line>,
    max_index: usize,
    max_line_number: usize,
}

impl Aligner {
    pub fn new(delimiter: &str) -> Self {
        Self {
            delimiter: String::from(delimiter),
            lines: Vec::new(),
            max_index: 0,
            max_line_number: 0
        }
    }

    pub fn add_line(&mut self, line: String, line_number: usize) {
        if let Some(index) = line.find(&self.delimiter) {
            if index > self.max_line_number {
                self.max_line_number = line_number;
                self.max_index = index;
            }
            self.lines.push(Line {
                text: line,
                number: line_number,
                delimiter_index: index,
            })
        }
    }

    pub fn aligned_lines<'a>(&'a self) -> AlignedLinesIterator<'a> {
        AlignedLinesIterator {
            aligner: self,
            index: 0
        }
    }
}


pub struct AlignedLinesIterator<'a> {
    aligner: &'a Aligner,
    index: usize,
}

impl<'a> Iterator for AlignedLinesIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.aligner.lines.len() {
            let line           = &self.aligner.lines[self.index];
            let padding_length = self.aligner.max_index - line.delimiter_index;
            let first_half     = &line.text[..line.delimiter_index];
            let second_half    = &line.text[line.delimiter_index..];
            self.index += 1;
            Some(format!("{}{}{}", first_half, " ".repeat(padding_length), second_half))
        }
        else {
            None
        }
    }
}


#[derive(Debug)]
struct Line {
    text: String,
    number: usize,
    delimiter_index: usize,
}


#[cfg(test)]
mod tests;
