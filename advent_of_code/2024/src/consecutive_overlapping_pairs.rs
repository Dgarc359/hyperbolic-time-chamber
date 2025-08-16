
pub struct ConsecutiveOverlappingPairs<T: Iterator> {
    inner: T,
    holding_area: Option<T::Item>,
}

impl<T: Iterator> ConsecutiveOverlappingPairs<T> {
    pub fn new(thing: T) -> ConsecutiveOverlappingPairs<T> {
        ConsecutiveOverlappingPairs {
            inner: thing,
            holding_area: None,
        }
    }
}

impl<T: Iterator> Iterator for ConsecutiveOverlappingPairs<T>
    where T::Item : Clone {
    type Item = (T::Item, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.holding_area.take().or_else(|| self.inner.next())?;
        let right = self.inner.next()?;
        self.holding_area = Some(right.clone());

        Some((left, right))
    }
}
pub trait ConsecutiveOverlappingPairsExt: Iterator {
    fn consecutive_pairs(self) -> impl Iterator<Item=(Self::Item, Self::Item)>;
}

impl<T> ConsecutiveOverlappingPairsExt for T
where
    T: Iterator,
    T::Item: Clone,
{
    fn consecutive_pairs(self) -> impl Iterator<Item=(Self::Item, Self::Item)> {
        ConsecutiveOverlappingPairs::new(self)
    }
}