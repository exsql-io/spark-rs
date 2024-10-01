mod map_rdd;
mod vec_rdd;
mod filter_rdd;
mod foreach_rdd;

pub trait RDD {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn close(&mut self);

    fn map<F, U>(self, f: F) -> map_rdd::MapRDD<Self, F> where Self: Sized, F: Fn(Self::Item) -> U {
        map_rdd::MapRDD::new(self, f)
    }
    
    fn filter<F>(self, f: F) -> filter_rdd::FilterRDD<Self, F> where Self: Sized, F: Fn(&Self::Item) -> bool {
        filter_rdd::FilterRDD::new(self, f)
    }
    
    fn for_each<F>(self, f: F) where Self: Sized, F: Fn(Self::Item) {
        let mut rdd = foreach_rdd::ForeachRDD::new(self, f);
        rdd.execute();
    }
}

pub fn new<T>(vec: Vec<T>) -> vec_rdd::VecRDD<T> {
    vec_rdd::VecRDD::new(vec)
}