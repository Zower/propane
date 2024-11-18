use std::iter;

use propane_lexer::Token;
use crate::expression::Expression;

struct Parser<'src> {
    tokens: &'src [Token],
    length: u32,
    char_index: u32,
}

impl Parser<'_> {

    // fn match_advance_or(&mut self, ch: char, is: TokenKind, or: TokenKind) -> TokenKind {
    //     if self.peek() == ch {
    //         self.discard();
    //         is
    //     } else {
    //         or
    //     }
    // }
    //
    // fn peek(&self) -> char {
    //     self.source.clone().next().unwrap_or('\0')
    // }
    //
    // fn discard(&mut self) {
    //     self.source.next();
    // }
    //
    // pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    //     while predicate(self.peek()) && self.peek() != '\0' {
    //         self.discard();
    //     }
    // }
}

pub fn parse(src: &[Token]) -> Expression {
    let mut parser = Parser {
        tokens: src,
        char_index: 0,
        length: src.len() as u32,
    };

    parser.parse()
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
