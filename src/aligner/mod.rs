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
        let maybe_index = line.find(&self.delimiter);
        if let Some(index) = maybe_index {
            if index > self.max_index {
                self.max_line_number = line_number;
                self.max_index = index;
            }
            
        }
        self.lines.push(Line {
            text: line,
            delimiter_index: maybe_index,
        });
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
            let line = &self.aligner.lines[self.index];
            self.index += 1;
            if let Some(delimiter_index) = line.delimiter_index {
                let padding_length = self.aligner.max_index - delimiter_index;
                let first_half     = &line.text[..delimiter_index];
                let second_half    = &line.text[delimiter_index..];
                Some(format!("{}{}{}", first_half, " ".repeat(padding_length), second_half))
            }
            else {
                Some(line.text.clone())
            }
        }
        else {
            None
        }
    }
}


struct Line {
    text: String,
    delimiter_index: Option<usize>,
}


#[cfg(test)]
mod tests;
