pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub struct Evens<I> {
    iter: I,
}
impl<I> Evens<I> {
    pub fn new(iter: I) -> Evens<I> {
        Evens { iter }
    }
}

impl<I> Iterator for Evens<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        let result = self.iter.next();
        // skip next item
        self.iter.next();
        result
    }
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    Evens::new(iter)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
