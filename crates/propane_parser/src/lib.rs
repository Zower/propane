use propane_lexer::{tokenize, Token};
use crate::expression::Expression;

mod expression;
mod parser;

pub fn parse(src: &str, tokens: &[Token]) -> Expression {
    parser::parse(src, &tokens)
}

#[cfg(test)]
mod tests {
    use codespan::Files;

    use super::*;

    #[test]
    fn it_works() {
        let mut files = Files::new();

        let main = files.add(
            "main",
            r#"
            "A string!"
// this is a comment
(( )){} // grouping stuff
!*+-/=<> <= == // operators
let x = 3;
        "#,
        );

        let tokens = tokenize(files.source(main));

        for token in &tokens {
            let _ = dbg!(&token.kind, files.source_slice(main, token.span).unwrap());
        }

        let expression = parse(files.source(main), &tokens);
    }
}
