use crate::rdd::RDD;

pub struct Filter<R, F> {
    parent: R,
    f: F,
}

impl<R, F> Filter<R, F> {
    pub fn new(parent: R, f: F) -> Filter<R, F> {
        Filter { parent, f }
    }
}

impl <R: RDD, F> RDD for Filter<R, F>
where F: FnMut(&R::Item) -> bool {

    type Item = R::Item;

    fn next(&mut self) -> Option<R::Item> {
        while let Some(item) = self.parent.next() {
            if (self.f)(&item) {
                return Some(item);
            }
        }

        None
    }

    fn close(&mut self) {
        self.parent.close();
    }

}
