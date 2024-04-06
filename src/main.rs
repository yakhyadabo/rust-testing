mod model;  // Import Module model

use crate::model::company::Company;
use crate::model::client::Client;

fn my_print() {
    println!("Hello, world!");
}

fn main() {
    let _comp = Company {};
    let _client = Client {};
    my_print()
}
