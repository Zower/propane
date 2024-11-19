use codespan::Span;

mod scanner;
pub fn tokenize(src: &str) -> Vec<Token> {
    scanner::scan(&src).collect()
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    // Multi-char tokens:
    /// "// comment"
    LineComment,

    /// `/* block comment */`
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment,

    /// Any whitespace character sequence.
    Whitespace,

    /// A user defined identifer
    Ident,

    /// Like the above, but containing invalid unicode codepoints.
    InvalidIdent,

    // Keywords:
    Let,
    If,
    Else,
    For,
    Fun,
    Return,
    While,


    /// Examples: `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
    /// suffix, but may be present here on string and float literals. Users of
    /// this type will need to check for and reject that case.
    ///
    /// See [Literal] for more details.
    Literal {
        kind: Literal,
    },

    // Two-char tokens:
    BangEq,
    EqEq,
    GtEq,
    LtEq,

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
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,
    /// "@"
    At,
    /// "#"
    Pound,
    /// "~"
    Tilde,
    /// "?"
    Question,
    /// ":"
    Colon,
    /// "$"
    Dollar,
    /// "="
    Eq,
    /// "!"
    Bang,
    /// "<"
    Lt,
    /// ">"
    Gt,
    /// "-"
    Minus,
    /// "&"
    And,
    /// "|"
    Or,
    /// "+"
    Plus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "^"
    Caret,
    /// "%"
    Percent,

    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,

    /// End of input.
    Eof,
}

/// Enum representing the literal types supported by the lexer.
///
/// Note that the suffix is *not* considered when deciding the `LiteralKind` in
/// this type. This means that float literals like `1f32` are classified by this
/// type as `Int`. (Compare against `rustc_ast::token::LitKind` and
/// `rustc_ast::ast::LitKind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    /// "12_u8", "0o100", "0b120i99", "1f32".
    // Int { base: Base, empty_int: bool },
    Int,
    /// "12.34f32", "1e3", but not "1f32".
    // Float { base: Base, empty_exponent: bool },
    Float,
    Bool,
    /// "'a'", "'\\'", "'''", "';"
    Char { terminated: bool },
    // /// "b'a'", "b'\\'", "b'''", "b';"
    // Byte { terminated: bool },
    /// ""abc"", ""abc"
    Str { terminated: bool },
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
    }
}
