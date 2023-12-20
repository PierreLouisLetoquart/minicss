use crate::error::LexError;
use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    current_pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars(),
            current_pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Move to the next character in the input.
    fn advance_char(&mut self) -> Option<char> {
        let next = self.chars.next();
        self.current_pos += 1;
        if let Some('\n') = next {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        next
    }

    /// Peek at the current character without consuming it.
    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Get the next token from the input.
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace_and_comments();
        let start_pos = self.current_pos;
        let current_char = self.advance_char();
        let token_kind = match current_char {
            Some('{') => TokenKind::CurlyBracketOpen,
            Some('}') => TokenKind::CurlyBracketClose,
            Some(':') => TokenKind::Colon,
            Some(';') => TokenKind::Semicolon,
            Some(',') => TokenKind::Comma,
            Some('(') => TokenKind::ParenthesisOpen,
            Some(')') => TokenKind::ParenthesisClose,
            Some(c) if c.is_alphabetic() || c == '_' => {
                return self.lex_identifier(start_pos); // A member function to lex identifiers
            }
            Some(c) if c.is_numeric() || c == '-' => {
                return self.lex_number(start_pos); // A member function to lex numbers
            }
            Some(c) if c == '"' || c == '\'' => {
                return self.lex_string(start_pos); // A member function to lex strings
            }
            Some('#') => {
                return self.lex_hash(start_pos); // A member function to lex hashes
            }
            Some('@') => {
                return self.lex_at_keyword(start_pos); // A member function to lex at-keywords
            }
            None => return Err(LexError::EndOfFile),
            _ => {
                return Err(LexError::unexpected_character(
                    current_char.unwrap(),
                    self.line,
                    self.column,
                ))
            }
        };
        let end_pos = self.current_pos;
        Ok(Token::new(token_kind, (start_pos, end_pos)))
    }

    fn lex_identifier(&mut self, start_pos: usize) -> Result<Token, LexError> {
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '-' {
                self.advance_char();
            } else {
                break;
            }
        }
        let value = self.input[start_pos..self.current_pos].to_string();
        Ok(Token::new(
            TokenKind::Identifier(value),
            (start_pos, self.current_pos),
        ))
    }

    fn lex_number(&mut self, start_pos: usize) -> Result<Token, LexError> {
        while let Some(c) = self.peek_char() {
            if c.is_numeric() || c == '.' {
                self.advance_char();
            } else {
                break;
            }
        }
        let value = self.input[start_pos..self.current_pos].to_string();
        if let Ok(number) = value.parse::<f64>() {
            Ok(Token::new(
                TokenKind::Number(number),
                (start_pos, self.current_pos),
            ))
        } else {
            Err(LexError::GenericError {
                message: "Invalid number".to_string(),
            })
        }
    }

    fn lex_string(&mut self, start_pos: usize) -> Result<Token, LexError> {
        let quote_char = self.advance_char().unwrap(); // We know this is '"' or '\''
        while let Some(c) = self.advance_char() {
            if c == quote_char {
                break;
            }
            // TODO:
            // Handle escape sequences and line breaks as needed...
        }
        if self.peek_char() == Some(quote_char) {
            self.advance_char(); // Consume the closing quote
            let value = self.input[start_pos + 1..self.current_pos - 1].to_string();
            Ok(Token::new(
                TokenKind::StringLiteral(value),
                (start_pos, self.current_pos),
            ))
        } else {
            Err(LexError::UnclosedString {
                line: self.line,
                column: self.column,
            })
        }
    }

    fn lex_hash(&mut self, start_pos: usize) -> Result<Token, LexError> {
        self.advance_char(); // We know this is '#'
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '-' {
                self.advance_char();
            } else {
                break;
            }
        }
        let value = self.input[start_pos + 1..self.current_pos].to_string();
        Ok(Token::new(
            TokenKind::Hash(value),
            (start_pos, self.current_pos),
        ))
    }

    fn lex_at_keyword(&mut self, start_pos: usize) -> Result<Token, LexError> {
        self.advance_char(); // We know this is '@'
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '-' {
                self.advance_char();
            } else {
                break;
            }
        }
        let value = self.input[start_pos + 1..self.current_pos].to_string();
        Ok(Token::new(
            TokenKind::AtKeyword(value),
            (start_pos, self.current_pos),
        ))
    }

    /// Skip over any whitespace or comments.
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek_char() {
                Some(c) if c.is_whitespace() => {
                    self.advance_char();
                }
                Some('/') if self.peek_ahead_char() == Some('*') => {
                    self.advance_char(); // Skip '/'
                    self.advance_char(); // Skip '*'
                    self.skip_block_comment();
                }
                _ => break,
            }
        }
    }

    fn peek_ahead_char(&mut self) -> Option<char> {
        let mut ahead_chars = self.chars.clone();
        ahead_chars.next();
        ahead_chars.next()
    }

    fn skip_block_comment(&mut self) {
        while let Some(c) = self.advance_char() {
            if c == '*' && self.peek_char() == Some('/') {
                self.advance_char(); // Skip '*'
                self.advance_char(); // Skip '/'
                break;
            }
        }
    }
}
