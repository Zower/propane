use codespan::FileId;
use propane_lexer::{tokenize, Literal, Token};
use crate::expression::Expression;
use crate::parser::ParseResult;

mod expression;
mod parser;

type ParserToken = Token<TokenKind>;

pub fn parse(file_id: FileId, src: &str, tokens: &[Token<propane_lexer::TokenKind>]) -> ParseResult {
    let tokens = tokens.iter().filter_map(|token|
        TokenKind::from_lexer(token.kind).map(|kind|
            Token {
                kind,
                span: token.span,
            }
        )
    ).collect::<Vec<_>>();

    parser::parse(file_id, src, &tokens)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    /// A user defined identifer
    Ident,

    // Keywords:
    Let,
    Fun,
    Return,

    /// Examples: `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [Literal] for more details.
    Literal {
        kind: Literal,
    },

    // One-char tokens:
    /// ";"
    Semi,
    /// ","
    Comma,
    /// "."
    Dot,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,

    Eq,
    Bang,
    BangEq,
    EqEq,

    Gt,
    GtEq,
    Lt,
    LtEq,

    Minus,
    Plus,
    Slash,
    Star,

    /// End of input.
    Eof,
}

impl TokenKind {
    fn from_lexer(kind: propane_lexer::TokenKind) -> Option<TokenKind> {
        match kind {
            propane_lexer::TokenKind::Ident => Some(TokenKind::Ident),
            propane_lexer::TokenKind::Let => Some(TokenKind::Let),
            propane_lexer::TokenKind::Fun => Some(TokenKind::Fun),
            propane_lexer::TokenKind::Return => Some(TokenKind::Return),
            propane_lexer::TokenKind::Literal { kind } => Some(TokenKind::Literal { kind }),
            propane_lexer::TokenKind::Semi => Some(TokenKind::Semi),
            propane_lexer::TokenKind::Comma => Some(TokenKind::Comma),
            propane_lexer::TokenKind::Dot => Some(TokenKind::Dot),
            propane_lexer::TokenKind::OpenParen => Some(TokenKind::OpenParen),
            propane_lexer::TokenKind::CloseParen => Some(TokenKind::CloseParen),
            propane_lexer::TokenKind::Eq => Some(TokenKind::Eq),
            propane_lexer::TokenKind::Plus => Some(TokenKind::Plus),
            propane_lexer::TokenKind::Eof => Some(TokenKind::Eof),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;
    use codespan::Files;
    use propane_lexer::LexerToken;
    use super::*;

    #[test]
    fn it_works() {
//         let mut files = Files::new();
//
//         let main = files.add(
//             "main",
//             r#"
// // this is a comment
// (( )){} // grouping stuff
// let x = 3;
//         "#,
//         );
//
//         let tokens = tokenize(files.source(main));
//
//         for token in &tokens {
//             let _ = dbg!(&token.kind, files.source_slice(main, token.span).unwrap());
//         }
//
//         let expression = parse(files.source(main), &tokens);
    }

    #[test]
    fn parse_let() {
        let src = r#"
        let main = 3 + 3;
        let the_end = 14 * 2 \ (8 / 2) - 14;

        let another = main * the_end;
        "#;

        let mut files = Files::new();

        let main = files.add(
            "main",
            src,
        );

        let tokens = tokenize(files.source(main));

        let expression = parse(main, src, &tokens);

        test_print(&expression)
    }

    #[test]
    fn parse_let_error() {
        let src = r#"
        let main 3 + 3;
        "#;

        let mut files = Files::new();

        let main = files.add(
            "main",
            src,
        );

        let tokens = tokenize(files.source(main));

        let Err(expression) = parse(main, src, &tokens) else {
            panic!("Expected err when parsing invalid let statement")
        };

        test_print(&expression)
    }

    fn test_print(str: &impl fmt::Debug) {
        assert!(false, "{:?}", str)
    }
}
