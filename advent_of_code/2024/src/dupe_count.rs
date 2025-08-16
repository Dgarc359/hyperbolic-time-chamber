
use std::iter::Peekable;
pub struct DupeCounter<T>
where
    T: Iterator,
    T::Item: PartialEq,
{
    inner: Peekable<T>,
}

impl<T> DupeCounter<T>
where
    T: Iterator,
    T::Item: PartialEq,
{
    pub fn new(thingy: T) -> DupeCounter<T> {
        return DupeCounter {
            inner: thingy.peekable(),
        };
    }
}

impl<T> Iterator for DupeCounter<T>
where
    T: Iterator,
    T::Item: PartialEq,
{
    type Item = (T::Item, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let next_item: T::Item = self.inner.next()?;
        let mut dupe_count = 1;
        while let Some(nexter_item) = self.inner.peek() {
            if nexter_item == &next_item {
                self.inner.next();
                dupe_count += 1;
            } else {
                break;
            }
        }
        Some((next_item, dupe_count))
    }
}

// DupeCounterExt: applies to anything that can be dupe-counted
pub trait DupeCounterExt: Iterator {
    fn count_dupes(self) -> impl Iterator<Item=(Self::Item, i32)>;
}

impl<T> DupeCounterExt for T
where
    T: Iterator,
    T::Item: PartialEq,
{
    fn count_dupes(self) -> impl Iterator<Item=(Self::Item, i32)> {
        DupeCounter::new(self)
    }
}