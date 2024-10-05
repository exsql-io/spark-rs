use crate::rdd::RDD;

pub struct FlatMap<I, R, F> {
    parent: R,
    f: F,
    iter: Option<I>
}

impl<I, R, F> FlatMap<I, R, F>
where Self: Sized {
    pub fn new(parent: R, f: F) -> FlatMap<I, R, F> {
        FlatMap { parent, f, iter: None }
    }
}

impl <I, U, R: RDD, F> RDD for FlatMap<I, R, F>
where
    I: Iterator<Item=U>,
    F: FnMut(R::Item) -> I {

    type Item = U;

    fn next(&mut self) -> Option<U> {
        match &mut self.iter {
            None => {
                match self.parent.next() {
                    None => None,
                    Some(item) => {
                        let mut iter = (self.f)(item);
                        let value = iter.next();
                        self.iter = Some(iter);
                        value
                    }
                }
            }
            Some(iter) => {
                match iter.next() {
                    None => {
                        self.iter = None;
                        self.next()
                    }
                    s@Some(_) => s,
                }
            }
        }
    }

    fn close(&mut self) {
        self.parent.close();
    }
}
