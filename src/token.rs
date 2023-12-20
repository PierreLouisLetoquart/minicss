#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Whitespace,
    CurlyBracketOpen,
    CurlyBracketClose,
    Colon,
    Semicolon,
    Comma,
    ParenthesisOpen,
    ParenthesisClose,
    Identifier(String),
    Function(String),
    AtKeyword(String),
    Hash(String),
    StringLiteral(String),
    BadString,
    Percentage(f64),
    Dimension(f64, String),
    Number(f64),
    Uri(String),
    UnicodeRange(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    /// The range within the input corresponding to this token.
    /// These are byte indices into the original string.
    pub range: (usize, usize),
}

impl Token {
    pub fn new(kind: TokenKind, range: (usize, usize)) -> Self {
        Token { kind, range }
    }
}
