use crate::TokenKind;

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Literal(Literal),
    Unary(Operator, Box<Expression>),
    StmtExpr(Vec<Statement>)
}

#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
    },
    Return {
        value: Expression
    }
}

pub enum Node {
    Expression(Expression),
    Statement(Statement),
    Program(Vec<Statement>),
}

#[derive(Debug)]
pub enum Operator {
    NotEq,
    EqEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Minus,
    Plus,
    Slash,
    Star,
}

#[derive(Clone, Debug)]
pub enum Literal {
    /// "12_u8", "0o100", "0b120i99", "1f32".
    // Int { base: Base, empty_int: bool },
    Int(i32),
    /// "12.34f32", "1e3", but not "1f32".
    // Float { base: Base, empty_exponent: bool },
    Float(f32),
    Bool(bool),
    /// "'a'", "'\\'", "'''", "';"
    Char(char),
    // /// "b'a'", "b'\\'", "b'''", "b';"
    // Byte { terminated: bool },
    /// ""abc"", ""abc"
    Str(String),
}

impl Literal {
    pub fn from_token_literal(other: propane_lexer::Literal, text: &str) -> Literal {
        match other {
            propane_lexer::Literal::Int => Literal::Int(text.parse().unwrap()),
            propane_lexer::Literal::Float => Literal::Float(text.parse().unwrap()),
            propane_lexer::Literal::Bool => Literal::Bool(text.parse().unwrap()),
            propane_lexer::Literal::Char { terminated: true } => Literal::Char(text.chars().nth(1).unwrap()),
            propane_lexer::Literal::Str { terminated: true } => Literal::Str(text[1..text.len() - 1].to_string()),
            _ => panic!("Unexpected literal type"),
        }
    }
}


impl Operator {
    pub fn from_token(token: TokenKind) -> Option<Operator> {
        match token {
            TokenKind::BangEq => Some(Operator::NotEq),
            TokenKind::EqEq => Some(Operator::EqEq),
            TokenKind::Gt => Some(Operator::Gt),
            TokenKind::GtEq => Some(Operator::GtEq),
            TokenKind::Lt => Some(Operator::Lt),
            TokenKind::LtEq => Some(Operator::LtEq),
            TokenKind::Minus => Some(Operator::Minus),
            TokenKind::Plus => Some(Operator::Plus),
            TokenKind::Slash => Some(Operator::Slash),
            TokenKind::Star => Some(Operator::Star),
            _ => None,
        }
    }
}