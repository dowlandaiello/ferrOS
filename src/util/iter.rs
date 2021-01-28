use core::iter::Iterator;

/// An iterator that has been joined to another iterator.
pub struct Chained<A: Iterator, B: Iterator> {
    a: A,
    b: Option<B>,

    // If the chained iterator hasn't completed the first iterator in the chain
    curr_a: bool,
}

impl<A: Iterator, B: Iterator> Chained<A, B> {
    /// Creates a new chained iterator from the starting iterator and the
    /// optional fallback iterator.
    pub fn new(a: A, b: Option<B>) -> Self {
        Self { a, b, curr_a: true }
    }
}

impl<I, A: Iterator<Item = I>, B: Iterator<Item = I>> Iterator for Chained<A, B> {
    // Both A and B iterators iterate over the same type
    type Item = I;

    fn next(&mut self) -> Option<I> {
        if self.curr_a {
            self.a.next()
        } else if let Some(ref b) = self.b {
            b.next()
        } else {
            None
        }
    }
}
