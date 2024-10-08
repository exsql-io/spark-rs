use fake::faker::name::en::{FirstName, LastName};
use fake::{Dummy, Fake, Faker};
use spark_rs_core::{rdd, RDD};
use std::fmt::Display;

#[derive(Debug, Dummy)]
struct Record {
    #[dummy(faker = "1000..")]
    id: u32,
    #[dummy(faker = "FirstName()")]
    first_name: String,
    #[dummy(faker = "LastName()")]
    last_name: String,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.id, self.first_name, self.last_name)
    }
}

fn main() {
    println!("######### records rdd #########");
    rdd::new(vec![1,2,3])
        .map(|int| int + 1)
        .filter(|&i| i % 2 == 0)
        .flat_map(|count| { (0..count).map(|_| Faker.fake::<Record>()) })
        .for_each(|r| println!("{:#}", r))
}
