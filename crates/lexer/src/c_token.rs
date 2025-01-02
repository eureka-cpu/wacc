use tokengen::{span::SourceSpan, token::Token, Token};

pub mod c_keyword;
pub mod c_symbol;

#[derive(Debug, Copy, Clone, Token)]
pub enum CToken<'a> {
    Keyword(c_keyword::Keyword<'a>),
    Operator(c_symbol::Operator<'a>),
    Punctuator(c_symbol::Punctuator<'a>),
    Identifier(tokengen::token::Ident<'a>),
    Constant(Constant<'a>),
    Whitespace,
}

#[derive(Debug, Copy, Clone, Token)]
pub struct Constant<'a> {
    #[allow(dead_code)]
    span: SourceSpan<'a>,
}
impl<'a> Constant<'a> {
    pub fn new(src: &'a str, start: usize, end: usize) -> Self {
        Self {
            span: SourceSpan::new(src, start, end),
        }
    }
}

#[macro_export]
macro_rules! lex_ctokens {
    ($src:expr, $pos:expr, $token_stream:expr, {
        $($token_kind:ident => $pattern:expr => {
            $closure:expr
        }),*
    }) => {{
        $(
            if let Some(mat) = regex::Regex::new($pattern)
                .expect(&format!("failed to create regex from pattern: {}", $pattern))
                .find_at($src, $pos)
            {
                if $pos == mat.start() {
                    if $pattern != r"\s" {
                        $token_stream.push($closure($src, mat.start(), mat.end()));
                    }
                    $pos = mat.end();
                }
            }
        )*
    }};
}
