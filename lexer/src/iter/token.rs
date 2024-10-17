use crate::iter::char::CharIter;
use crate::model::token::Token;
use std::fmt::Debug;
use std::fs::File;
use std::iter::Peekable;
use std::mem::discriminant;
use std::vec::IntoIter;

pub trait BaseTokenIter: Iterator {
    fn next_token(&mut self) -> Option<Token>;
    fn peek(&mut self) -> Option<&Token>;
    fn get_last_pos(&self) -> (u32, u32);

    fn peek_eq(&mut self, expected: &Token) -> bool {
        match self.peek() {
            Some(t) => discriminant(expected) == discriminant(t),
            _ => false,
        }
    }

    fn next_if_eq(&mut self, expected: &Token) -> Option<Token> {
        if self.peek_eq(expected) {
            self.next_token()
        } else {
            None
        }
    }

    fn consume_next_if_eq(&mut self, expected: &Token) {
        if self.peek_eq(expected) {
            let _ = self.next_token();
        }
    }

    /// Returns if there are more tokens to be consumed
    fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }

    fn get_required(&mut self, expected: &Token) -> Result<Token, String> {
        match self.next_token() {
            Some(token) if token == *expected => Ok(token),
            None if *expected == Token::EOF => Ok(Token::EOF),

            Some(token) => Err(format!(
                "expected {} around {:#?}, found {token}",
                expected.get_key(),
                self.get_last_pos()
            )),
            None => Err(format!(
                "expected {} around {:?} but reached end of stream",
                expected.get_key(),
                self.get_last_pos()
            )),
        }
    }

    /// Returns the result of consuming the next token with [expected]
    fn consume_required(&mut self, expected: &Token) -> Result<(), String> {
        match self.get_required(expected) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Consumes next token without returning it, does not throw any error
    fn consume_next(&mut self) {
        let _ = self.next_token();
    }

    fn collect_till(&mut self, read_till_token: &Token) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut seen_open_curl = 0;
        let mut seen_open_paren = 0;
        let mut seen_start_if = 0;
        let mut seen_start_case = 0;
        let mut seen_start_loop = 0;
        let mut seen_start_let = 0; // matches with `in`

        // read tokens, accounting for matching brackets and matching expressions
        loop {
            if self.peek_eq(read_till_token)
                && seen_open_curl == 0
                && seen_open_paren == 0
                && seen_start_if == 0
                && seen_start_case == 0
                && seen_start_loop == 0
                && seen_start_let == 0
            {
                break; // reached the real end, accounted for all matching brackets
            }

            let token = match self.next_token() {
                None => {
                    assert_eq!(seen_open_curl, 0);
                    assert_eq!(seen_open_paren, 0);
                    break;
                }
                Some(t) => t,
            };

            match token {
                Token::OpenParen { .. } => seen_open_paren += 1,

                Token::CloseParen { .. } => seen_open_paren -= 1,

                Token::OpenCurl { .. } => seen_open_curl += 1,

                Token::CloseCurl { .. } => seen_open_curl -= 1,

                Token::If { .. } => seen_start_if += 1,

                Token::EndIf { .. } => seen_start_if -= 1,

                Token::Case { .. } => seen_start_case += 1,

                Token::EndCase { .. } => seen_start_case -= 1,

                Token::Loop { .. } => seen_start_loop += 1,

                Token::EndLoop { .. } => seen_start_loop -= 1,

                Token::Let { .. } => seen_start_let += 1,

                Token::In { .. } => seen_start_let -= 1,

                Token::Comment { .. } => continue,

                _ => (),
            }

            tokens.push(token);
        }

        if self.has_next() {
            // If there are more tokens, then the next token in iterator must match the `read_till_token`;
            // Otherwise we have reached the end of list, no need to assert
            assert!(self.peek_eq(read_till_token));
        }

        tokens
    }
}

pub struct BufferedTokenIter {
    buffer: Peekable<IntoIter<Token>>,
    last_line_num: u32,
    last_line_pos: u32,
}

impl From<Vec<Token>> for BufferedTokenIter {
    fn from(value: Vec<Token>) -> Self {
        BufferedTokenIter {
            buffer: value.into_iter().peekable(),
            last_line_num: 0,
            last_line_pos: 0,
        }
    }
}

impl From<TokenIter> for BufferedTokenIter {
    fn from(iter: TokenIter) -> Self {
        let tokens: Vec<Token> = iter.collect();
        tokens.into()
    }
}

impl Iterator for BufferedTokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.buffer.next();
        if let Some(ref token) = t {
            (self.last_line_num, self.last_line_pos) = token.get_pos();
        }

        t
    }
}

impl BaseTokenIter for BufferedTokenIter {
    fn next_token(&mut self) -> Option<Token> {
        self.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.buffer.peek()
    }

    fn get_last_pos(&self) -> (u32, u32) {
        (self.last_line_num, self.last_line_pos)
    }
}

impl BufferedTokenIter {
    #[must_use]
    pub fn gen_iter_till(&mut self, read_till_token: &Token) -> Self {
        let tokens = self.collect_till(read_till_token);
        Self::from(tokens)
    }
}

#[derive(Debug)]
pub struct TokenIter {
    char_iter: Peekable<CharIter>,
    last_line_num: u32,
    last_line_pos: u32,
}

impl From<String> for TokenIter {
    fn from(value: String) -> Self {
        TokenIter {
            char_iter: CharIter::from(value).peekable(),
            last_line_num: 0,
            last_line_pos: 0,
        }
    }
}

impl From<File> for TokenIter {
    fn from(value: File) -> Self {
        TokenIter {
            char_iter: CharIter::from(value).peekable(),
            last_line_num: 0,
            last_line_pos: 0,
        }
    }
}

impl Iterator for TokenIter {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.char_iter.next();
        if let Some(ref token) = t {
            (self.last_line_num, self.last_line_pos) = token.get_pos();
        }

        t
    }
}

impl BaseTokenIter for TokenIter {
    fn next_token(&mut self) -> Option<Token> {
        self.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.char_iter.peek()
    }

    fn get_last_pos(&self) -> (u32, u32) {
        (self.last_line_num, self.last_line_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_iterator() {
        let file1 = File::open("../test_resources/programs/cool.cl").unwrap();
        let file2 = File::open("../test_resources/programs/arith.cl").unwrap();
        let files = vec![file1, file2];

        for file in files {
            let iter = TokenIter::from(file);

            for t in iter {
                println!("{t}");
                let is_error_token = matches!(t, Token::Error { .. });
                assert!(!is_error_token);
            }
        }
    }

    #[test]
    fn test_token_printer() {
        let file = File::open("../test_resources/programs/primes.cl").expect("Cannot open file");

        let iter = TokenIter::from(file);

        for t in iter {
            println!("{t}");
            let is_error_token = matches!(t, Token::Error { .. });
            assert!(!is_error_token);
        }
    }
}
