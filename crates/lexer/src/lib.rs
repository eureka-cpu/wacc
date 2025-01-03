use c_token::{
    c_keyword::{Int, Keyword, Return, Void},
    c_symbol::{CloseCurlyBrace, CloseParenthesis, OpenCurlyBrace, OpenParenthesis, Semicolon},
};
use tokengen::{
    span::{SourceSpan, Span},
    token::{Ident, Token, TokenStream},
};

use crate::c_token::CToken;

pub mod c_token;

pub struct ErrorEmitter<E>
where
    E: std::error::Error,
{
    state: Vec<E>,
}
impl<E> ErrorEmitter<E>
where
    E: std::error::Error,
{
    pub fn new() -> Self {
        Self { state: Vec::new() }
    }
    pub fn push(&mut self, err: E) {
        self.state.push(err);
    }
    pub fn report_errors(self) {
        if !self.state.is_empty() {
            self.state.iter().for_each(|e| eprintln!("Error: {e:?}\n"));
            std::process::exit(1);
        }
    }
}
impl<E> Default for ErrorEmitter<E>
where
    E: std::error::Error,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Error: {context}:\n{source}", source = span.span())]
pub struct LexError {
    span: SourceSpan,
    context: String,
}
impl LexError {
    pub fn new(src: &str, start: usize, end: usize, context: &str) -> Self {
        Self {
            span: SourceSpan::new(src, start, end),
            context: context.into(),
        }
    }
}

pub trait Lexable: AsRef<str> + Sized {}
impl<T: AsRef<str> + Sized> Lexable for T {}

pub trait Lexer: Lexable {
    /// Takes some source code and a closure that returns `Token`s.
    fn lex<'a, T>(&'a self, f: impl FnOnce(&'a str) -> TokenStream<T>) -> TokenStream<T>
    where
        T: Token,
    {
        f(self.as_ref())
    }
    fn lex_c(src: &str) -> TokenStream<CToken> {
        let mut error_emitter = ErrorEmitter::default();
        let mut pos = 0_usize;
        let mut token_stream = TokenStream::new(src.len());
        while pos < src.len() {
            match_regex!(src, pos, token_stream, error_emitter, {
                r"\s" => {
                    |_, _, _| -> Result<CToken, LexError> {
                        Ok(CToken::Whitespace)
                    }
                },
                r"[a-zA-Z_]\w*\b" => {
                    |src: &str, start: usize, end: usize| -> Result<CToken, LexError> {
                        let raw = SourceSpan::new(src, start, end);
                        Ok(match raw.span() {
                            "int" => CToken::Keyword(Keyword::Int(Int::new(src, start, end))),
                            "void" => CToken::Keyword(Keyword::Void(Void::new(src, start, end))),
                            "return" => CToken::Keyword(Keyword::Return(Return::new(src, start, end))),
                            _ => CToken::Identifier(Ident::new(src, start, end)),
                        })
                    }
                },
                r"[0-9]+\b" => {
                    |src: &str, start: usize, end: usize| -> Result<CToken, LexError> {
                        Ok(CToken::Constant(c_token::Constant::new(src, start, end)))
                    }
                },
                r"\(" => {
                    |src: &str, start: usize, end: usize| -> Result<CToken, LexError> {
                        Ok(CToken::Punctuator(c_token::c_symbol::Symbol::OpenParenthesis(OpenParenthesis::new(src, start, end))))
                    }
                },
                r"\)" => {
                    |src: &str, start: usize, end: usize| -> Result<CToken, LexError> {
                        Ok(CToken::Punctuator(c_token::c_symbol::Symbol::CloseParenthesis(CloseParenthesis::new(src, start, end))))
                    }
                },
                r"\{" => {
                    |src: &str, start: usize, end: usize| -> Result<CToken, LexError> {
                        Ok(CToken::Punctuator(c_token::c_symbol::Symbol::OpenCurlyBrace(OpenCurlyBrace::new(src, start, end))))
                    }
                },
                r"\}" => {
                    |src: &str, start: usize, end: usize| -> Result<CToken, LexError> {
                        Ok(CToken::Punctuator(c_token::c_symbol::Symbol::CloseCurlyBrace(CloseCurlyBrace::new(src, start, end))))
                    }
                },
                r"\;" => {
                    |src: &str, start: usize, end: usize| -> Result<CToken, LexError> {
                        Ok(CToken::Punctuator(c_token::c_symbol::Symbol::Semicolon(Semicolon::new(src, start, end))))
                    }
                },
                r"." => {
                    |src: &str, start: usize, end: usize| -> Result<CToken, LexError> {
                        Err(LexError::new(src, start, end, "Unrecognized token"))
                    }
                },
            });
        }

        if token_stream.is_empty() {
            eprintln!("token stream is empty");
            std::process::exit(1);
        }
        error_emitter.report_errors();

        token_stream
    }
}

pub struct Source<S: Lexable>(S);
impl<S: Lexable> Source<S> {
    pub fn new(src: S) -> Self {
        Self(src)
    }
}
impl<S: Lexable> AsRef<str> for Source<S> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl<T: Lexable> Lexer for T {}

#[cfg(test)]
mod lexer_tests {
    use expect_test::{expect, Expect};
    use tokengen::token::{Token, TokenStream};

    use super::Lexer;
    use crate::c_token::CToken;

    fn check_tokens<T: Token + std::fmt::Debug>(output: TokenStream<T>, expect: Expect) {
        expect.assert_eq(&format!("{output:#?}"));
    }

    #[test]
    fn test_lex_c() {
        let input = r#"
            int main(void) {
              return 2;
            }
        "#;
        check_tokens(
            Lexer::lex::<CToken>(&input, String::lex_c),
            expect![[r#"
                TokenStream(
                    [
                        Keyword(
                            Int(
                                Int {
                                    span: SourceSpan {
                                        src: "int",
                                        start: 13,
                                        end: 16,
                                    },
                                },
                            ),
                        ),
                        Identifier(
                            Ident {
                                span: SourceSpan {
                                    src: "main",
                                    start: 17,
                                    end: 21,
                                },
                            },
                        ),
                        Punctuator(
                            OpenParenthesis(
                                OpenParenthesis {
                                    span: SourceSpan {
                                        src: "(",
                                        start: 21,
                                        end: 22,
                                    },
                                },
                            ),
                        ),
                        Keyword(
                            Void(
                                Void {
                                    span: SourceSpan {
                                        src: "void",
                                        start: 22,
                                        end: 26,
                                    },
                                },
                            ),
                        ),
                        Punctuator(
                            CloseParenthesis(
                                CloseParenthesis {
                                    span: SourceSpan {
                                        src: ")",
                                        start: 26,
                                        end: 27,
                                    },
                                },
                            ),
                        ),
                        Punctuator(
                            OpenCurlyBrace(
                                OpenCurlyBrace {
                                    span: SourceSpan {
                                        src: "{",
                                        start: 28,
                                        end: 29,
                                    },
                                },
                            ),
                        ),
                        Keyword(
                            Return(
                                Return {
                                    span: SourceSpan {
                                        src: "return",
                                        start: 44,
                                        end: 50,
                                    },
                                },
                            ),
                        ),
                        Constant(
                            Constant {
                                span: SourceSpan {
                                    src: "2",
                                    start: 51,
                                    end: 52,
                                },
                            },
                        ),
                        Punctuator(
                            Semicolon(
                                Semicolon {
                                    span: SourceSpan {
                                        src: ";",
                                        start: 52,
                                        end: 53,
                                    },
                                },
                            ),
                        ),
                        Punctuator(
                            CloseCurlyBrace(
                                CloseCurlyBrace {
                                    span: SourceSpan {
                                        src: "}",
                                        start: 66,
                                        end: 67,
                                    },
                                },
                            ),
                        ),
                    ],
                )"#]],
        );
    }
}
