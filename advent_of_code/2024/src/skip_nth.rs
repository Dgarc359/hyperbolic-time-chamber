pub struct SkipNth<T: Iterator> {
    inner: T,
    n_to_skip: usize,
    idx_counter: usize,
}

impl<T: Iterator> SkipNth<T> {
    pub fn new(thing: T, n_to_skip: usize) -> SkipNth<T> {
        SkipNth {
            inner: thing,
            n_to_skip,
            idx_counter: 0,
        }
    }
}

impl<T: Iterator> Iterator for SkipNth<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.inner.next()?;

        let result = if self.idx_counter == self.n_to_skip {
            self.inner.next()
        } else {
            Some(next_item)
        };

        self.idx_counter += 1;

        result
    }
}
pub trait SkipNthExt: Iterator {
    fn skip_nth(self, i: usize) -> impl Iterator<Item = Self::Item>;
}

impl<T> SkipNthExt for T
where
    T: Iterator,
{
    fn skip_nth(self, i: usize) -> impl Iterator<Item = Self::Item> {
        SkipNth::new(self, i)
    }
}
