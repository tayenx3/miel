#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub source_id: usize
}

impl Span {
    #[inline]
    pub const fn empty(source_id: usize) -> Self {
        Self {
            start: 0usize,
            end: 1usize,
            source_id
        }
    }

    #[inline]
    pub fn concat(&self, other: &Span) -> Self {
        debug_assert_eq!(self.source_id, other.source_id);
        debug_assert!(self.start <= other.end);
        Self {
            start: self.start,
            end: other.end,
            source_id: self.source_id
        }
    }

    #[inline]
    pub const fn splat_to_end(&self) -> Self {
        Self {
            start: self.end,
            end: self.end + 1,
            source_id: self.source_id
        }
    }
}

impl AsRef<Span> for Span {
    fn as_ref(&self) -> &Span {
        self
    }
}