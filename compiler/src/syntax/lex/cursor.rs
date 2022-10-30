use std::{str::Chars, io::Error, io::ErrorKind};

pub const END_OF_FILE: char = '\0';

/// A struct that handles a stream of chars
pub struct Cursor<'a> {
    ilen: usize,
    chars: Chars<'a>,
    prev: char,
    /// the current index in the chars buffer.
    index: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            ilen: input.len(),
            chars: input.chars(),
            prev: END_OF_FILE,
            index: 0,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.prev = c;

                if self.is_eof() {
                    return None;
                }

                self.index += 1;

                Some(c)
            }
            None => None,
        }
    }

    pub fn unpeek(&mut self) -> char {
        self.chars.nth(self.eaten() - 1).unwrap_or(END_OF_FILE)
    }

    /// Is End of file?
    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    // Grabs the next char without consuming it.
    pub fn first(&self) -> Result<char, Error> {
        self.nth_char(0)
    }

    // Grabs the second char without consuming it.
    pub fn second(&self) -> Result<char, Error> {
        self.nth_char(1)
    }

    /// Returns the `nth_char` releative to the current cursor pos
    /// If the position given doesn't exist, `END_OF_FILE` is returned.
    pub fn nth_char(&self, amt: usize) -> Result<char, Error> {
        self.chars().nth(amt).ok_or_else(|| Error::from(ErrorKind::UnexpectedEof))
    }

    /// Copies the current chars in the cursor.
    pub fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    pub fn get_pos(&self) -> usize {
        self.index.clone()
    }

    pub fn get_prev(&self) -> char {
        self.prev.clone()
    }

    pub fn peek_get_pos(&mut self) -> usize {
        self.peek();
        self.index.clone()
    }

    /// Increment current position by `x` and return the new position.
    ///
    /// ! DOES NOT PEEK, FOR PEEK INCREMENTING USE `peek_inc`
    pub fn ipeek_get_pos(&mut self, x: usize) -> usize {
        self.peek_inc(x);
        self.index.clone()
    }

    /// Increments the current buffer with the given one.
    /// Peeks `x` times
    pub fn peek_inc(&mut self, x: usize) {
        let mut i = 0;
        while !self.is_eof() && i <= x {
            self.peek();
            i += 1;
        }
    }

    /// Shows how many chars have been consumed by the cursor.
    pub fn eaten(&self) -> usize {
        self.ilen - self.chars.as_str().len()
    }

    pub fn eat_while(&mut self, mut pred: impl FnMut(char) -> bool) -> Result<String, Error> {
        let mut segment = String::new();
        while !self.is_eof() && pred(self.first()?) == true {
            segment.push(self.peek().unwrap_or(END_OF_FILE));
        }
        Ok(segment)
    }

    pub fn eat_while_cursor(
        &mut self,
        mut pred: impl FnMut(&mut Cursor<'a>, char) -> bool,
    ) -> Result<String, Error> {
        let mut segment = String::new();
        while !self.is_eof() && pred(self, self.first()?) == true {
            segment.push(self.peek().unwrap_or(END_OF_FILE));
        }
        Ok(segment)
    }
}

pub fn is_line_ending(c: char) -> bool {
    c == '\n'
}


