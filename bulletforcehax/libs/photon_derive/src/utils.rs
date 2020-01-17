// probably a bit overkill, but it's a nice exercise :)
/// An iterator that only returns 1 item, and panics if there are more.
pub struct SingleIter<I> {
    iter: I,
    returned_value: bool,
}

impl<I> SingleIter<I> {
    pub fn new(iter: I) -> SingleIter<I> {
        SingleIter { iter, returned_value: false }
    }
}

impl<I> SingleIter<I>
where
    I: Iterator,
{
    /// Returns None on no items, Some(T) on 1 item, or panics on multiple items
    pub fn get(mut self) -> Option<<I as Iterator>::Item> {
        let next = self.next();
        self.next();
        next
    }
}

impl<I> Iterator for SingleIter<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.returned_value {
            self.returned_value = true;
            self.iter.next()
        } else {
            if let Some(_) = self.iter.next() {
                panic!("Iter contained multiple elements");
            } else {
                None
            }
        }
    }
}

/// Adds the extention method `single` to iterators
pub trait IterSingleExt: Iterator {
    fn single(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        SingleIter::new(self).get()
    }
}
impl<I: Iterator> IterSingleExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Vec::<()>::new().into_iter().single(), None);
    }

    #[test]
    fn test_1() {
        assert_eq!(vec![()].into_iter().single(), Some(()));
    }

    #[test]
    #[should_panic]
    fn test_2() {
        let _ = vec![(), ()].into_iter().single();
    }
}
