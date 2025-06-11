pub struct Aligner {
    delimiter: String,
    lines: Vec<Line>,
    max_index: usize,
}

impl Aligner {
    pub fn new(delimiter: &str) -> Self {
        Self {
            delimiter: String::from(delimiter),
            lines: Vec::new(),
            max_index: 0,
        }
    }

    pub fn add_line(&mut self, line: String) {
        let maybe_index = line.find(&self.delimiter);
        if let Some(index) = maybe_index {
            if index > self.max_index {
                self.max_index = index;
            }
            
        }
        self.lines.push(Line {
            text: line,
            delimiter_index: maybe_index,
        });
    }

    pub fn aligned_lines(self) -> impl Iterator<Item = String> {
        self.lines.into_iter().map(move |line| {
            if let Some(delimiter_index) = line.delimiter_index {
                let padding_length = self.max_index - delimiter_index;
                let first_half     = &line.text[..delimiter_index];
                let second_half    = &line.text[delimiter_index..];
                format!("{}{}{}", first_half, " ".repeat(padding_length), second_half)
            }
            else {
                line.text
            }
        })
    }
}


struct Line {
    text: String,
    delimiter_index: Option<usize>,
}


#[cfg(test)]
mod tests;
