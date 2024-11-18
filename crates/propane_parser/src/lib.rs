use propane_lexer::Token;
use crate::expression::Expression;

mod expression;
mod parser;

pub fn parse(tokens: &[Token]) -> Expression {
    parser::parse(&tokens).collect()
}
