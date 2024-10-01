use crate::rdd::RDD;

pub struct MapRDD<R, F> {
    parent: R,
    f: F,
}

impl<R, F> MapRDD<R, F> {
    pub fn new(parent: R, f: F) -> MapRDD<R, F> {
        MapRDD { parent, f }
    }
}

impl <U, R: RDD, F> RDD for MapRDD<R, F>
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
