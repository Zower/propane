use propane_lexer::{tokenize, Literal, Token};
use crate::expression::Expression;

mod expression;
mod parser;

type ParserToken = Token<TokenKind>;

pub fn parse(src: &str, tokens: &[Token<propane_lexer::TokenKind>]) -> Expression {
    let tokens = tokens.iter().filter_map(|token|
        TokenKind::from_lexer(token.kind).map(|kind|
            Token {
                kind,
                span: token.span,
            }
        )
    ).collect::<Vec<_>>();

    parser::parse(src, &tokens)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    /// A user defined identifer
    Ident,

    // Keywords:
    Let,
    Fun,

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
            propane_lexer::TokenKind::Literal { kind } => Some(TokenKind::Literal { kind }),
            propane_lexer::TokenKind::Semi => Some(TokenKind::Semi),
            propane_lexer::TokenKind::Comma => Some(TokenKind::Comma),
            propane_lexer::TokenKind::Dot => Some(TokenKind::Dot),
            propane_lexer::TokenKind::OpenParen => Some(TokenKind::OpenParen),
            propane_lexer::TokenKind::CloseParen => Some(TokenKind::CloseParen),
            propane_lexer::TokenKind::Eof => Some(TokenKind::Eof),
            _ => None,
        }
    }
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
// this is a comment
(( )){} // grouping stuff
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
