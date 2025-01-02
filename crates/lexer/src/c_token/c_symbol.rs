use tokengen::{token::Token, Token};

pub type Operator<'a> = Symbol<'a>;
pub type Punctuator<'a> = Symbol<'a>;

tokengen::symbol!(
    [OpenParenthesis, '('],
    [CloseParenthesis, ')'],
    [OpenCurlyBrace, '{'],
    [CloseCurlyBrace, '}'],
    [Semicolon, ';']
);
