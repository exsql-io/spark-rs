use crate::rdd::RDD;

pub struct VecRDD<T> {
    iter: std::vec::IntoIter<T>,
}

impl<T> VecRDD<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { iter: vec.into_iter() }
    }
}

impl<T> RDD for VecRDD<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    fn close(&mut self) {}
}
