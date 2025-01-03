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
    fn lex_c<'a>(src: &'a str) -> TokenStream<CToken<'a>> {
        let mut pos = 0_usize;
        let mut token_stream = TokenStream::new(src.len());
        while pos < src.len() {
            lex_ctokens!(src, pos, token_stream, {
                Whitespace => r"\s" => {
                    |_src: &'a str, _start: usize, _end: usize| -> CToken<'a> {
                        CToken::Whitespace
                    }
                },
                Identifier => r"[a-zA-Z_]\w*\b" => {
                    |src: &'a str, start: usize, end: usize| -> CToken<'a> {
                        let raw = SourceSpan::new(src, start, end);
                        match raw.span() {
                            "int" => CToken::Keyword(Keyword::Int(Int::new(src, start, end))),
                            "void" => CToken::Keyword(Keyword::Void(Void::new(src, start, end))),
                            "return" => CToken::Keyword(Keyword::Return(Return::new(src, start, end))),
                            _ => CToken::Identifier(Ident::new(src, start, end)),
                        }
                    }
                },
                Constant => r"[0-9]+\b" => {
                    |src: &'a str, start: usize, end: usize| -> CToken<'a> {
                        CToken::Constant(c_token::Constant::new(src, start, end))
                    }
                },
                Punctuator => r"\(" => {
                    |src: &'a str, start: usize, end: usize| -> CToken<'a> {
                        CToken::Punctuator(c_token::c_symbol::Symbol::OpenParenthesis(OpenParenthesis::new(src, start, end)))
                    }
                },
                Punctuator => r"\)" => {
                    |src: &'a str, start: usize, end: usize| -> CToken<'a> {
                        CToken::Punctuator(c_token::c_symbol::Symbol::CloseParenthesis(CloseParenthesis::new(src, start, end)))
                    }
                },
                Punctuator => r"\{" => {
                    |src: &'a str, start: usize, end: usize| -> CToken<'a> {
                        CToken::Punctuator(c_token::c_symbol::Symbol::OpenCurlyBrace(OpenCurlyBrace::new(src, start, end)))
                    }
                },
                Punctuator => r"\}" => {
                    |src: &'a str, start: usize, end: usize| -> CToken<'a> {
                        CToken::Punctuator(c_token::c_symbol::Symbol::CloseCurlyBrace(CloseCurlyBrace::new(src, start, end)))
                    }
                },
                Punctuator => r"\;" => {
                    |src: &'a str, start: usize, end: usize| -> CToken<'a> {
                        CToken::Punctuator(c_token::c_symbol::Symbol::Semicolon(Semicolon::new(src, start, end)))
                    }
                }
            });
        }

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
                                        src: "\n            int main(void) {\n              return 2;\n            }\n        ",
                                        start: 13,
                                        end: 16,
                                    },
                                },
                            ),
                        ),
                        Identifier(
                            Ident {
                                span: SourceSpan {
                                    src: "\n            int main(void) {\n              return 2;\n            }\n        ",
                                    start: 17,
                                    end: 21,
                                },
                            },
                        ),
                        Punctuator(
                            OpenParenthesis(
                                OpenParenthesis {
                                    span: SourceSpan {
                                        src: "\n            int main(void) {\n              return 2;\n            }\n        ",
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
                                        src: "\n            int main(void) {\n              return 2;\n            }\n        ",
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
                                        src: "\n            int main(void) {\n              return 2;\n            }\n        ",
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
                                        src: "\n            int main(void) {\n              return 2;\n            }\n        ",
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
                                        src: "\n            int main(void) {\n              return 2;\n            }\n        ",
                                        start: 44,
                                        end: 50,
                                    },
                                },
                            ),
                        ),
                        Constant(
                            Constant {
                                span: SourceSpan {
                                    src: "\n            int main(void) {\n              return 2;\n            }\n        ",
                                    start: 51,
                                    end: 52,
                                },
                            },
                        ),
                        Punctuator(
                            Semicolon(
                                Semicolon {
                                    span: SourceSpan {
                                        src: "\n            int main(void) {\n              return 2;\n            }\n        ",
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
                                        src: "\n            int main(void) {\n              return 2;\n            }\n        ",
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
