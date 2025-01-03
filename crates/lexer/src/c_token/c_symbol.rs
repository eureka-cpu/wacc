use tokengen::{token::Token, Token};

pub type Operator = Symbol;
pub type Punctuator = Symbol;

tokengen::symbol!(
    [OpenParenthesis, '('],
    [CloseParenthesis, ')'],
    [OpenCurlyBrace, '{'],
    [CloseCurlyBrace, '}'],
    [Semicolon, ';']
);
