#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
        }
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Debug)]
pub struct Spanned<T> {
    span: Span,
    node: T
}

impl<T> Spanned<T> {
    pub fn new(start: usize, end: usize, node: T) -> Self {
        Self {
            span: Span {
                start,
                end,
            },
            node,
        }
    }

    pub fn new_from_span(span: Span, node: T) -> Self {
        Self {
            span,
            node,
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn node(&self) -> &T {
        &self.node
    }
}

impl<T: Clone> Clone for Spanned<T> {
    fn clone(&self) -> Self {
        Self {
            span: self.span,
            node: self.node.clone()
        }
    }
}

impl<T: Copy> Copy for Spanned<T> {}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.span == other.span
    }
}

impl<T: PartialEq> Eq for Spanned<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}