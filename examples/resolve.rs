extern crate resolv;

use resolv::record::MX;
use resolv::{Class, RecordType, Resolver};

fn main() {
    // You must create a mutable resolver object to hold the context.
    let mut resolver = Resolver::new().unwrap();

    // .query() and .search() are the main interfaces to the resolver.
    let mut response = resolver
        .query(b"gmail.com", Class::IN, RecordType::MX)
        .unwrap();

    // You can iterate through answers as follows.  You must specify the
    // type of record.  A run-time error will occur if the records
    // decoded are of the wrong type.
    for answer in response.answers::<MX>() {
        println!("{:?}", answer);
    }
}
