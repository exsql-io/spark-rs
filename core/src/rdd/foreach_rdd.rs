use crate::rdd::RDD;

pub struct ForeachRDD<R, F> {
    parent: R,
    f: F,
}

impl<R: RDD, F> ForeachRDD<R, F> where F: FnMut(R::Item) {
    pub fn new(parent: R, f: F) -> ForeachRDD<R, F> {
        ForeachRDD { parent, f }
    }
    
    pub fn execute(&mut self) {
        self.next();
    }
}

impl <R: RDD, F> RDD for ForeachRDD<R, F>
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
