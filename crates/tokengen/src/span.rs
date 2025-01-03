pub trait Span {
    fn src(&self) -> &str;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn span(&self) -> &str;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SourceSpan<'a> {
    src: &'a str,
    start: usize,
    end: usize,
}

impl<'a> SourceSpan<'a> {
    pub fn new(src: &'a str, start: usize, end: usize) -> Self {
        Self { src, start, end }
    }
}

impl Span for SourceSpan<'_> {
    fn src(&self) -> &str {
        self.src
    }
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> usize {
        self.end
    }
    fn span(&self) -> &str {
        &self.src[self.start..self.end]
    }
    fn len(&self) -> usize {
        (self.start..self.end).count()
    }
}
