use crate::rdd::RDD;

pub struct Map<R, F> {
    parent: R,
    f: F,
}

impl<R, F> Map<R, F> {
    pub fn new(parent: R, f: F) -> Map<R, F> {
        Map { parent, f }
    }
}

impl <U, R: RDD, F> RDD for Map<R, F>
where F: FnMut(R::Item) -> U {

    type Item = U;

    fn next(&mut self) -> Option<U> {
        match self.parent.next() {
            None => None,
            Some(item) => Some((self.f)(item)),
        }
    }

    fn close(&mut self) {
        self.parent.close();
    }
}
