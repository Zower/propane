use std::iter;
use std::str::Chars;

use codespan::Span;

use crate::TokenKind::*;
use crate::{LexerToken, Token, TokenKind};

struct Scanner<'src> {
    text: &'src str,
    source: Chars<'src>,
    length: u32,
    char_index: u32,
}

impl Scanner<'_> {
    fn next_token(&mut self) -> Option<LexerToken> {
        let char = self.source.next()?;

        let start = self.char_index;

        let kind = match char {
            c if is_whitespace(c) => {
                self.eat_while(is_whitespace);

                Whitespace
            }
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '@' => At,
            '#' => Pound,
            '~' => Tilde,
            '?' => Question,
            ':' => Colon,
            '$' => Dollar,
            '-' => Minus,
            '&' => And,
            '|' => Or,
            '+' => Plus,
            '*' => Star,
            '^' => Caret,
            '%' => Percent,
            '!' => self.match_advance_or('=', BangEq, Bang),
            '=' => self.match_advance_or('=', EqEq, Eq),
            '<' => self.match_advance_or('=', LtEq, Lt),
            '>' => self.match_advance_or('=', GtEq, Gt),
            '/' => match self.peek() {
                '/' => {
                    self.eat_while(|ch| ch != '\n');

                    LineComment
                }
                // '*' => self.block_comment(),
                _ => Slash,
            },
            // Numeric literal.
            '0'..='9' => TokenKind::Literal {
                kind: crate::Literal::Float,
            },
            '"' => {
                self.eat_while(|ch| ch != '"');

                let terminated = self.peek() == '"';

                let lit = Literal {
                    kind: crate::Literal::Str { terminated },
                };

                if terminated {
                    self.discard();
                }

                lit
            }
            c if is_identifier(c) => {
                self.eat_while(|c| is_identifier(c) || c.is_numeric() );

                // todo common func
                let end = self.length - self.source.as_str().len() as u32;

                let ident_text = &self.text[start as usize..end as usize];

                match ident_text {
                    "let" => TokenKind::Let,
                    // "if" => TokenKind::If,
                    // "else" => TokenKind::Else,
                    // "for" => TokenKind::For,
                    "fun" => TokenKind::Fun,
                    // "return" => TokenKind::Return,
                    "true" | "false" => TokenKind::Literal { kind: crate::Literal::Bool },
                    // "while" => TokenKind::While,
                    _ => TokenKind::Ident
                }
            }
            _ => Unknown,
        };

        let end = self.length - self.source.as_str().len() as u32;

        let span = Span::new(start, end);

        self.char_index = end;

        Some(Token { kind, span })
    }

    fn match_advance_or(&mut self, ch: char, is: TokenKind, or: TokenKind) -> TokenKind {
        if self.peek() == ch {
            self.discard();
            is
        } else {
            or
        }
    }

    fn peek(&self) -> char {
        self.source.clone().next().unwrap_or('\0')
    }

    fn discard(&mut self) {
        self.source.next();
    }

    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && self.peek() != '\0' {
            self.discard();
        }
    }
}

pub fn scan(src: &str) -> impl Iterator<Item = LexerToken> + '_ {
    let mut scanner = Scanner {
        text: src,
        source: src.chars(),
        char_index: 0,
        length: src.len() as u32,
    };

    iter::from_fn(move || scanner.next_token()).chain(iter::once(Token {
        kind: TokenKind::Eof,
        span: Span::new(src.len() as u32, src.len() as u32),
    }))
}

fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

fn is_identifier(c: char) -> bool {
    // TODO: Use xid_unicode
    c.is_alphabetic() || c == '_'
}
