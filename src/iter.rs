pub struct Split<T, I, F>
where
    F: Fn(&T) -> bool,
    I: Iterator<Item = T>,
{
    predicate: F,
    iterator: I,
    _data: std::marker::PhantomData<T>,
}

pub type SplitIterator<T> = <Vec<T> as IntoIterator>::IntoIter;

impl<T, I, F> Iterator for Split<T, I, F>
where
    F: Fn(&T) -> bool,
    I: Iterator<Item = T>,
{
    type Item = SplitIterator<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut elements = Vec::<T>::default();

        while let Some(el) = self.iterator.next() {
            if (self.predicate)(&el) {
                break;
            }

            elements.push(el);
        }

        if elements.is_empty() {
            return None;
        }

        return Some(elements.into_iter());
    }
}

pub trait Splittable {
    type Iterator: Iterator;

    fn split_at<F>(
        self,
        predicate: F,
    ) -> Split<<Self::Iterator as Iterator>::Item, Self::Iterator, F>
    where
        F: Fn(&<Self::Iterator as Iterator>::Item) -> bool;
}

pub fn split_at<T, I, F>(iterator: I, predicate: F) -> Split<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    Split {
        predicate,
        iterator,
        _data: Default::default(),
    }
}

impl<T, I> Splittable for T
where
    T: Iterator<Item = I>,
{
    type Iterator = Self;

    fn split_at<F>(
        self,
        predicate: F,
    ) -> Split<<Self::Iterator as Iterator>::Item, Self::Iterator, F>
    where
        F: Fn(&<Self::Iterator as Iterator>::Item) -> bool,
    {
        split_at(self, predicate)
    }
}
