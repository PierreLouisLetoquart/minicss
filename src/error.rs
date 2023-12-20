#[derive(Debug)]
pub enum LexError {
    UnexpectedCharacter {
        character: char,
        line: usize,
        column: usize,
    },
    GenericError {
        message: String,
    },
    EndOfFile,
    UnclosedString {
        line: usize,
        column: usize,
    },
    UnclosedComment {
        line: usize,
        column: usize,
    },
}

impl LexError {
    pub fn unexpected_character(character: char, line: usize, column: usize) -> Self {
        LexError::UnexpectedCharacter {
            character,
            line,
            column,
        }
    }

    pub fn generic_error(message: String) -> Self {
        LexError::GenericError { message }
    }
}
