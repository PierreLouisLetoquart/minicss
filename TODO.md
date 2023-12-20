## TODOs

- [ ] move the main code into examples/lexer.rs --> cargo run --example lexer
- [ ] add unit tests #[cfg(test)] in all src/ files
- [ ] add missing tokens
- [ ] find alternatives for .clone() --> lighter one

## Notes

After implementing Lexer/Tokenizer -> AST (Abstract syntax tree)

For CSS, the AST seems to be CSSOM. [Here](https://www.w3.org/TR/cssom-1/) is the w3c doc for CSSOM.
