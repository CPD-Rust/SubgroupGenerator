use std::collections::HashSet;

mod permutation;
mod group;

use permutation::CustomDisplay;

fn main() {
    println!("S4 is {:?}", group::elements(4));
}
