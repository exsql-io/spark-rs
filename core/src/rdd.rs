mod map;
mod vec;
mod filter;
mod foreach;

pub trait RDD {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn close(&mut self);

    fn map<F, U>(self, f: F) -> map::Map<Self, F> where Self: Sized, F: Fn(Self::Item) -> U {
        map::Map::new(self, f)
    }
    
    fn filter<F>(self, f: F) -> filter::Filter<Self, F> where Self: Sized, F: Fn(&Self::Item) -> bool {
        filter::Filter::new(self, f)
    }
    
    fn for_each<F>(self, f: F) where Self: Sized, F: Fn(Self::Item) {
        let mut rdd = foreach::Foreach::new(self, f);
        rdd.execute();
    }
}

pub fn new<T>(vec: Vec<T>) -> vec::Collection<T> {
    vec::Collection::new(vec)
}