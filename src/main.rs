use minicss::{LexError, Lexer};

fn main() {
    let css_input = r#"main {
        color: red;
        background-color: #fff;
        border: 1px solid black;
        /* A comment */
        font-size: 1.25em;
    }"#;

    let mut lexer = Lexer::new(css_input);

    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("Token: {:?}", token);

                // TODO !!
                // Impl EOF Token for clean breaking loop.

                // if token.kind == TokenKind::EOF {
                //     break;
                // }
            }
            Err(LexError::EndOfFile) => {
                println!("End of file reached.");
                break;
            }
            Err(e) => {
                eprintln!("Error occurred during lexing: {:?}", e);
                break;
            }
        }
    }
}
