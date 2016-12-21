use std::collections::HashSet;

mod permutation;
mod group;

use permutation::CustomDisplay;

fn main() {
    println!("S3 is {:?}", group::elements(3));
}
