use tokengen::token::{Token, TokenStream};

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

// #[cfg(test)]
// mod lexer_tests {
//     use super::Lexer;
//     use crate::token::{Token, TokenStream};
//     use expect_test::{expect, Expect};

//     fn check_tokens<T: Token + std::fmt::Debug>(output: TokenStream<T>, expect: Expect) {
//         expect.assert_eq(&format!("{output:#?}"));
//     }

//     #[test]
//     fn lex_bash() {
//         let input = r#"
//             <SOME CODE SNIPPET>
//         "#;
//         check_tokens(Lexer::lex::<Token>(&input, String::lex_bash), expect![[]]);
//     }
// }
