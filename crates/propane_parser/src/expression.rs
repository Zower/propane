use propane_lexer::Literal;

pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Literal),
    Unary(Operator, Box<Expression>),
}

enum Operator {
}