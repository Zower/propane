use codespan::{FileId, Span};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use crate::expression::{Expression, Literal, Statement};
use crate::{ParserToken, TokenKind};
use crate::expression::Expression::StmtExpr;

struct Parser<'src> {
    tokens: &'src [ParserToken],
    src: &'src str,
    file_id: FileId,
    current: usize,
    errors: Vec<Diagnostic<FileId>>
}

pub type ParseResult = Result<Expression, Vec<Diagnostic<FileId>>>;

impl Parser<'_> {
    fn parse(mut self) -> ParseResult {
        let mut statements = vec![];

        loop {
            if self.tokens.get(self.current).unwrap().kind == TokenKind::Eof {
                break;
            }

            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            } else {
                break;
            }
        }

        if self.errors.is_empty() {
            Ok(StmtExpr(statements))
        } else {
            Err(self.errors)
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token().kind {
            TokenKind::Let => {
                self.parse_let_statement()
            }
            TokenKind::Return => {
                self.parse_return_statement()
            }
            _ => {
                self.parse_expression_statement()
            }
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let Some(ident_token) = self.peek_expect_and_advance(TokenKind::Ident) else {
            return None;
        };

        let name = self.src[ident_token.span.start().0 as usize..ident_token.span.end().0 as usize].to_string();

        if self.peek_expect_and_advance(TokenKind::Eq).is_none() {
            return None;
        }

        if !self.skip_expression_to_semi_temp() {
            return None;
        }

        Some(Statement::Let { name, value: Expression::Literal(Literal::Int(1)) } )
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        if !self.skip_expression_to_semi_temp() {
            return None;
        }

        Some(Statement::Return { value: Expression::Literal(Literal::Int(1)) } )
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if !self.skip_expression_to_semi_temp() {
            return None;
        }

        Some(Statement::Return { value: Expression::Literal(Literal::Int(1)) } )
    }

    fn skip_expression_to_semi_temp(&mut self) -> bool {
        loop {
            match self.current_token().kind {
                TokenKind::Semi => {
                    self.advance();
                    break;
                },
                TokenKind::Eof => {
                    let diagnostic = self.expected_token_error(TokenKind::Eof, TokenKind::Semi, self.current_token().span);

                    self.errors.push(diagnostic);

                    return false;
                }
                _ => {
                    self.advance();
                },
            }
        }

        true
    }

    fn current_token(&self) -> ParserToken {
        self.tokens.get(self.current).cloned().unwrap_or(ParserToken { kind: TokenKind::Eof, span: Span::new(self.src.len() as u32, self.src.len() as u32) })
    }

    fn peek_token(&self) -> Option<ParserToken> {
        self.tokens.get(self.current + 1).cloned()
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek_expect_and_advance(&mut self, kind: TokenKind) -> Option<ParserToken> {
        if let Some(token) = self.peek_token() {
            if token.kind == kind {
                self.advance();

                Some(token)
            } else {
                let diagnostic = self.expected_token_error(token.kind, kind, token.span);

                self.errors.push(diagnostic);

                None
            }
        } else {
            None
        }
    }

    fn expected_token_error(&self, found: TokenKind, expected: TokenKind, span: Span) -> Diagnostic<FileId> {
        Diagnostic::error()
            .with_message("Unexpected token found")
            // .with_code("")
            .with_labels(vec![
                Label::primary(self.file_id, span.start().0 as usize..span.end().0 as usize).with_message(format!("expected `{:?}`, found `{:?}`", expected, found)),
                // Label::secondary(file_id, 211..331).with_message("`case` clauses have incompatible types"),
            ])
        //     .with_notes(vec![unindent::unindent(
        //         "
        //     expected type `String`
        //         found type `Nat`
        // ",
        //     )]
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

pub fn parse(file_id: FileId, src: &str, tokens: &[ParserToken]) -> ParseResult {
    let mut parser = Parser {
        tokens,
        src,
        file_id,
        current: 0,
        errors: Vec::new(),
    };

    parser.parse()
}
