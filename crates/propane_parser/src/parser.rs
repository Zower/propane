use propane_lexer::{Token };
use crate::expression::{Expression, Literal, Operator, Statement};
use crate::{ParserToken, TokenKind};

struct Parser<'src> {
    tokens: &'src [ParserToken],
    src: &'src str,
    current: usize,
}

impl Parser<'_> {
    fn parse(&mut self) -> Expression {
        loop {
            let token = self.tokens.get(self.current).unwrap();

            if token.kind == TokenKind::Eof {
                break;
            }


            Self::parseStatement()

            // self.current += 1;
        }

        todo!()
        // self.expression()
    }

    fn parseStatement(tokens: &[ParserToken], current: &mut usize) -> Option<Statement> {

    }
//     fn parse(&mut self) -> Expression {
//         self.expression()
//     }
//
//     fn is_at_end(&self) -> bool {
//         self.current >= self.tokens.len()
//     }
//
//     fn peek(&self) -> &ParserToken{
//         &self.tokens[self.current]
//     }
//
//     fn previous(&self) -> &ParserToken {
//         &self.tokens[self.current - 1]
//     }
//
//     fn advance(&mut self) -> &ParserToken{
//         if !self.is_at_end() {
//             self.current += 1;
//         }
//
//         self.previous()
//     }
//
//     fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
//         if self.is_at_end() {
//             return false;
//         }
//
//         if kinds.contains(&self.peek().kind) {
//             self.current += 1;
//
//             true
//         } else {
//             false
//         }
//     }
//
//     fn expression(&mut self) -> Expression {
//         self.equality()
//     }
//
//     fn equality(&mut self) -> Expression {
//         let mut expr = self.comparison();
//
//         while self.match_token(&[TokenKind::BangEq, TokenKind::EqEq]) {
//             let operator = Operator::from_token(self.previous().kind).unwrap();
//
//             let right = self.comparison();
//             expr = Expression::Binary {
//                 left: Box::new(expr),
//                 operator,
//                 right: Box::new(right),
//             };
//         }
//
//         expr
//     }
//
//     fn comparison(&mut self) -> Expression {
//         let mut expr = self.term();
//
//         while self.match_token(&[TokenKind::Gt, TokenKind::GtEq, TokenKind::Lt, TokenKind::LtEq]) {
//             let operator = Operator::from_token(self.previous().kind).unwrap();
//
//             let right = self.term();
//             expr = Expression::Binary {
//                 left: Box::new(expr),
//                 operator,
//                 right: Box::new(right),
//             };
//         }
//
//         expr
//     }
//
//     fn term(&mut self) -> Expression {
//         let mut expr = self.factor();
//
//         while self.match_token(&[TokenKind::Minus, TokenKind::Plus]) {
//             let operator = Operator::from_token(self.previous().kind).unwrap();
//
//             let right = self.factor();
//             expr = Expression::Binary {
//                 left: Box::new(expr),
//                 operator,
//                 right: Box::new(right),
//             };
//         }
//
//         expr
//     }
//
//     fn factor(&mut self) -> Expression {
//         let mut expr = self.unary();
//
//         while self.match_token(&[TokenKind::Slash, TokenKind::Star]) {
//             let operator = Operator::from_token(self.previous().kind).unwrap();
//
//             let right = self.unary();
//             expr = Expression::Binary {
//                 left: Box::new(expr),
//                 operator,
//                 right: Box::new(right),
//             };
//         }
//
//         expr
//     }
//
//     fn unary(&mut self) -> Expression {
//         if self.match_token(&[TokenKind::Bang, TokenKind::Minus]) {
//             let operator = Operator::from_token(self.previous().kind).unwrap();
//
//             let right = self.unary();
//             Expression::Unary(operator, Box::new(right))
//         } else {
//             self.primary()
//         }
//     }
//
//     fn primary(&mut self) -> Expression {
//         if let Token { kind: TokenKind::Literal { kind }, span } = *self.peek() {
//             self.advance();
//
//             Expression::Literal(Literal::from_token_literal(kind, &self.src[span.start().0 as usize..span.end().0 as usize]))
//         } else if self.match_token(&[TokenKind::OpenParen]) {
//             let expr = self.expression();
//             if !self.match_token(&[TokenKind::CloseParen]) {
//                 panic!("Expected ')' after expression.");
//             }
//
//             Expression::Grouping(Box::new(expr))
//         } else {
//             dbg!(self.peek());
//             panic!("Expected expression.");
//         }
//     }
}

pub fn parse(src: &str, tokens: &[ParserToken]) -> Expression {
    let mut parser = Parser {
        tokens,
        src,
        current: 0,
    };

    parser.parse()
}
