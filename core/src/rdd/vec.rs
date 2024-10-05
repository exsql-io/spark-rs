use crate::rdd::RDD;

pub struct Collection<T> {
    iter: std::vec::IntoIter<T>,
}

impl<T> Collection<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { iter: vec.into_iter() }
    }
}

impl<T> RDD for Collection<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn close(&mut self) {}
}
