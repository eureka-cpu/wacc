use tokengen::{span::SourceSpan, token::Token, Token};

pub mod c_keyword;
pub mod c_symbol;

#[derive(Debug, Copy, Clone, Token, PartialEq, Eq)]
pub enum CToken {
    Keyword(c_keyword::Keyword),
    Operator(c_symbol::Operator),
    Punctuator(c_symbol::Punctuator),
    Identifier(tokengen::token::Ident),
    Constant(Constant),
    Whitespace,
}
impl CToken {
    pub fn is_whitespace(&self) -> bool {
        self == &Self::Whitespace
    }
}

#[derive(Debug, Copy, Clone, Token, PartialEq, Eq)]
pub struct Constant {
    #[allow(dead_code)]
    span: SourceSpan,
}
impl Constant {
    pub fn new(src: &str, start: usize, end: usize) -> Self {
        Self {
            span: SourceSpan::new(src, start, end),
        }
    }
}

#[macro_export]
macro_rules! match_regex {
    ($src:expr, $pos:expr, $token_stream:expr, $errors:expr, {
        $($pattern:expr => {$closure:expr}),* $(,)?
    }) => {{
        use once_cell::sync::Lazy;
        use regex::Regex;

        static REGEX_PATTERNS_AND_CLOSURES: Lazy<Vec<(Regex, fn(&str, usize, usize) -> Result<CToken, LexError>)>> = Lazy::new(|| vec![
            $(
                (
                    Regex::new($pattern).expect(&format!("failed to create regex from pattern: {}", $pattern)),
                    $closure,
                ),
            )*
        ]);

        for (regex, closure) in REGEX_PATTERNS_AND_CLOSURES.iter() {
            if let Some(mat) = regex.find_at($src, $pos) {
                if $pos == mat.start() {
                    match closure($src, mat.start(), mat.end()) {
                        Ok(ctoken) if !ctoken.is_whitespace() => $token_stream.push(ctoken),
                        Err(err) => $errors.push(err),
                        _ => {}
                    }
                    $pos = mat.end();
                    break;
                }
            }
        }
    }};
}
