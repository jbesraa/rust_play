pub fn flatten<I>(iter: I) -> Flatten<I> 
where I:Iterator, 
{
    Flatten::new(iter)
};

pub struct Flatten<O> 
where 
    O:Iterator,
{
    outer: O,
    inner: O::Item,
}

impl<O> Flatten<O> 
where
    O: Iterator,
{
    fn new(iter: O) -> Self {
        Flatten { outer: iter }
    }
}


impl<O> Iterator for Flatten<O> 
where 
    O: Iterator,
    O::Item:IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Options<Self::Item> {
        let inner_item = self.outer.next()?;
        let mut inner_iter = inner_item.into_iter();
        inner_iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(Flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }
}
