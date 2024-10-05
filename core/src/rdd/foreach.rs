use crate::rdd::RDD;

pub struct Foreach<R, F> {
    parent: R,
    f: F,
}

impl<R: RDD, F> Foreach<R, F> where F: FnMut(R::Item) {
    pub fn new(parent: R, f: F) -> Foreach<R, F> {
        Foreach { parent, f }
    }
    
    pub fn execute(&mut self) {
        self.next();
    }
}

impl <R: RDD, F> RDD for Foreach<R, F>
where F: FnMut(R::Item) {

    type Item = R::Item;

    fn next(&mut self) -> Option<R::Item> {
        while let Some(item) = self.parent.next() {
            (self.f)(item);
        }

        self.close();
        None
    }

    fn close(&mut self) {
        self.parent.close();
    }

}
