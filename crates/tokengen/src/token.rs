use crate::span::Span;
use derive_token::Token;

pub trait Token: Copy {}

// TODO: Maybe make this a recursive data structure?
#[derive(Debug)]
pub struct TokenStream<T: Token>(Vec<T>);
impl<T: Token> TokenStream<T> {
    pub fn new() -> Self {
        Self(Vec::with_capacity(50))
    }
    pub fn push(&mut self, token: T) {
        self.0.push(token)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}
impl<T: Token> Default for TokenStream<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Basic single character symbols that can later be used in crafting tokens,
/// or in match arms when lexing. This macro allows for common aliases to
/// be passed as a list for convenience, but is subject to change.
/// Additional derive traits can optionally be added at the end to extend
/// functionality without the need of explicit impl blocks.
#[macro_export]
macro_rules! symbol {
    ( $([$name:ident, $char:literal $(,[$($alias:ident),*]),* $(,{$($trait:ident),*})* ]),+ ) => {
        $(
            #[allow(dead_code)] // Ignore warnings if alias is never used
            $($(pub type $alias<'a> = $name<'a>;)*)*

            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord $(,$($trait,)*)*)]
            pub struct $name<'a> {
                span: $crate::span::SourceSpan<'a>,
            }
            impl<'a> $name<'a> {
                #[allow(dead_code)] // Ignore warnings if type is never constructed with `$name::new`
                pub fn new(src: &'a str, start: usize, end: usize) -> Self {
                    Self { span: $crate::span::SourceSpan::new(src, start, end) }
                }
            }
            impl<'a> $crate::token::Span for $name<'a> {
                fn src(&self) -> &str {
                    self.span.src()
                }
                fn start(&self) -> usize {
                    self.span.start()
                }
                fn end(&self) -> usize {
                    self.span.end()
                }
                fn span(&self) -> &str {
                    self.span.span()
                }
                fn len(&self) -> usize {
                    self.span.len()
                }
            }
            impl<'a> AsRef<char> for $name<'a> {
                fn as_ref(&self) -> &char {
                    &$char
                }
            }
            impl<'a> std::fmt::Display for $name<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.as_ref())
                }
            }
        )+
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Symbol {
            $($name,)+
        }
        impl AsRef<char> for Symbol {
            fn as_ref(&self) -> &char {
                match self {
                    $(Self::$name => &$char,)+
                }
            }
        }
        impl std::fmt::Display for Symbol {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name => write!(f, "{}", self.as_ref()),)+
                }
            }
        }
    };
}

/// A cooked symbol is two or more symbols that together form a specific identifier.
#[macro_export]
macro_rules! cooked_symbol {
    ( $symbol:ty, $([$name:ident, [$($variant:ident),+] $(,{$($trait:ident),*})* ]),+ ) => {
        /// More than one symbol concatenated together.
        pub trait CookedSymbol {
            /// The symbols that a cooked symbol is made from.
            const SYMBOLS: &'static [$symbol];

            /// The symbols that a cooked symbol is made from.
            fn symbols() -> &'static [$symbol];
        }
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $name<'a> {
                span: $crate::span::SourceSpan<'a>,
            }
            impl<'a> $name<'a> {
                pub fn new(src: &'a str, start: usize, end: usize) -> Self {
                    Self { span: $crate::span::SourceSpan::new(src, start, end) }
                }
            }
            impl<'a> $crate::span::Span for $name<'a> {
                fn src(&self) -> &str {
                    self.span.src()
                }
                fn start(&self) -> usize {
                    self.span.start()
                }
                fn end(&self) -> usize {
                    self.span.end()
                }
                fn span(&self) -> &str {
                    self.span.span()
                }
                fn len(&self) -> usize {
                    self.span.len()
                }
            }
            impl<'a> CookedSymbol for $name<'a> {
                const SYMBOLS: &'static [$symbol] = &[$(<$symbol>::$variant,)+];
                fn symbols() -> &'static [$symbol] {
                    Self::SYMBOLS
                }
            }
            impl<'a> std::fmt::Display for $name<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Ok(<Self as CookedSymbol>::symbols()
                        .iter()
                        .for_each(|symbol| write!(f, "{symbol}")
                        .expect("failed to format cooked symbol.")))
                }
            }
        )+
    };
}

/// A keyword is some string identifier that is reserved for the language
/// in order to establish patterns that can be used in conjunction with `Symbol`s.
#[macro_export]
macro_rules! keyword {
    ( $([$name:ident, $str:literal]),+ ) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $name<'a> {
                span: $crate::span::SourceSpan<'a>,
            }
            impl<'a> $name<'a> {
                pub fn new(src: &'a str, start: usize, end: usize) -> Self {
                    Self { span: $crate::span::SourceSpan::new(src, start, end) }
                }
            }
            impl<'a> $crate::span::Span for $name<'a> {
                fn src(&self) -> &str {
                    self.span.src()
                }
                fn start(&self) -> usize {
                    self.span.start()
                }
                fn end(&self) -> usize {
                    self.span.end()
                }
                fn span(&self) -> &str {
                    self.span.span()
                }
                fn len(&self) -> usize {
                    self.span.len()
                }
            }
            impl<'a> AsRef<str> for $name<'a> {
                fn as_ref(&self) -> &str {
                    &$str
                }
            }
            impl<'a> std::fmt::Display for $name<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.as_ref())
                }
            }
        )+
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Keyword {
            $($name,)+
        }
        impl AsRef<str> for Keyword {
            fn as_ref(&self) -> &str {
                match self {
                    $(Self::$name => $str,)+
                }
            }
        }
        impl std::fmt::Display for Keyword {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name => write!(f, "{}", self.as_ref()),)+
                }
            }
        }
    };
}

/// Denotes that a [`Symbol`] or [`CookedSymbol`] is also classified as a potential [`Delimiter`].
pub trait Delimiter: Copy + Span {}

/// A [`Token`] delimited by some [`Symbol`] or [`CookedSymbol`].
//
/// Delimiters are `Option` since we should try to recover if parsing fails.
/// [`DelimitedToken`]s are also considered [`Token`]s since they could potentially be nested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Token)]
pub struct DelimitedToken<O, T, C>
where
    O: Delimiter,
    T: Token,
    C: Delimiter,
{
    open: Option<O>,
    token: Option<T>,
    close: Option<C>,
}
impl<O, T, C> DelimitedToken<O, T, C>
where
    O: Delimiter,
    T: Token,
    C: Delimiter,
{
    pub fn new(open: Option<O>, token: Option<T>, close: Option<C>) -> Self {
        Self { open, token, close }
    }
    pub fn open(&self) -> Option<O> {
        self.open
    }
    pub fn token(&self) -> Option<T> {
        self.token
    }
    pub fn close(&self) -> Option<C> {
        self.close
    }
}
impl<O, T, C> Span for DelimitedToken<O, T, C>
where
    O: Delimiter,
    T: Token,
    C: Delimiter,
{
    fn src(&self) -> &str {
        todo!("src should come from the token")
    }
    fn start(&self) -> usize {
        self.open().unwrap().start()
    }
    fn end(&self) -> usize {
        self.close().unwrap().end()
    }
    fn span(&self) -> &str {
        &self.src()[self.start()..self.end()]
    }
    fn len(&self) -> usize {
        (self.start()..self.end()).count()
    }
}

#[cfg(test)]
mod token_tests {
    //! Tests for asserting that the macros expand as expected.

    use super::{DelimitedToken, Delimiter, Token};
    use crate::span::Span;
    use derive_token::Delimiter;
    use expect_test::{expect, Expect};

    #[derive(Debug, Copy, Clone)]
    struct DummyToken;
    impl Token for DummyToken {}

    symbol!(
        [ExclamationMark, '!', [Bang]],
        [PoundSign, '#', [Hash]],
        [OpenParenthesis, '(', { Delimiter }],
        [ClosedParenthesis, ')', { Delimiter }]
    );

    fn check_spans<S: Span + std::fmt::Debug>(output: S, expect: Expect) {
        expect.assert_eq(&format!("{output:#?}"));
    }

    #[test]
    fn test_symbol() {
        let symbol = Symbol::PoundSign;
        let symbol_str = symbol.as_ref().to_string();
        let src = r#"# Hello, World!"#;
        let hash = Hash::new(src, 0, symbol_str.len());

        assert_eq!(symbol_str.len(), hash.len());
        assert_eq!(symbol_str, format!("{hash}"));
        check_spans(
            hash,
            expect![[r##"
            PoundSign {
                span: SourceSpan {
                    src: "# Hello, World!",
                    start: 0,
                    end: 1,
                },
            }"##]],
        );
    }

    #[test]
    fn test_cooked_symbol() {
        cooked_symbol!(Symbol, [Shebang, [PoundSign, ExclamationMark]]);

        let symbols = [Symbol::PoundSign, Symbol::ExclamationMark];
        let symbol_str = symbols.iter().map(|s| s.as_ref()).collect::<String>();
        let src = r#"#! /bin/bash"#;
        let shebang = Shebang::new(src, 0, symbol_str.len());

        assert_eq!(symbols, Shebang::symbols());
        assert_eq!(symbol_str.len(), shebang.len());
        assert_eq!(symbol_str, format!("{shebang}"));
        assert_eq!(symbol_str, shebang.span());
        check_spans(
            shebang,
            expect![[r##"
            Shebang {
                span: SourceSpan {
                    src: "#! /bin/bash",
                    start: 0,
                    end: 2,
                },
            }"##]],
        );
    }

    #[test]
    fn test_keyword() {
        keyword!([If, "if"]);

        let keyword = Keyword::If;
        let keyword_str = keyword.as_ref();
        let src = r#"if [ ! -e "$1" ]; then"#;
        let if_keyword = If::new(src, 0, keyword_str.len());

        assert_eq!(keyword_str.len(), if_keyword.len());
        assert_eq!(keyword_str, format!("{if_keyword}"));
        assert_eq!(keyword_str, if_keyword.span());
        check_spans(
            if_keyword,
            expect![[r#"
            If {
                span: SourceSpan {
                    src: "if [ ! -e \"$1\" ]; then",
                    start: 0,
                    end: 2,
                },
            }"#]],
        );
    }

    #[test]
    fn test_delimiter() {
        let open_str = Symbol::OpenParenthesis.as_ref().to_string();
        let close_str = Symbol::ClosedParenthesis.as_ref().to_string();
        let src = r#"()"#;
        let open = OpenParenthesis::new(src, 0, open_str.len());
        let close = ClosedParenthesis::new(src, open.end(), open.end() + close_str.len());
        let delimited_token = DelimitedToken::new(Some(open), None::<DummyToken>, Some(close));

        assert_eq!(open_str.len(), delimited_token.open().unwrap().len());
        assert_eq!(close_str.len(), delimited_token.close().unwrap().len());
        assert_eq!(open_str, delimited_token.open().unwrap().span());
        assert_eq!(close_str, delimited_token.close().unwrap().span());
        check_spans(
            delimited_token,
            expect![[r#"
            DelimitedToken {
                open: Some(
                    OpenParenthesis {
                        span: SourceSpan {
                            src: "()",
                            start: 0,
                            end: 1,
                        },
                    },
                ),
                token: None,
                close: Some(
                    ClosedParenthesis {
                        span: SourceSpan {
                            src: "()",
                            start: 1,
                            end: 2,
                        },
                    },
                ),
            }"#]],
        );
    }
}
