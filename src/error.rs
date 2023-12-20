#[derive(Debug)]
pub enum LexError {
    /// Represents an encountered unexpected character.
    UnexpectedCharacter {
        /// The unexpected character that was encountered.
        character: char,
        /// The line where the unexpected character was found.
        line: usize,
        /// The column where the unexpected character was found.
        column: usize,
    },
    /// Represents a generic lexing error with a custom message.
    GenericError {
        /// The message describing the error.
        message: String,
    },
    /// Represents an error due to reaching the end of the file unexpectedly.
    EndOfFile,
    /// Represents an error due to an unclosed string literal.
    UnclosedString {
        /// The line where the unclosed string starts.
        line: usize,
        /// The column where the unclosed string starts.
        column: usize,
    },
    /// Represents an error due to an unclosed comment.
    UnclosedComment {
        /// The line where the unclosed comment starts.
        line: usize,
        /// The column where the unclosed comment starts.
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
