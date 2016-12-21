use std::collections::HashSet;

mod permutation;
mod group;

use permutation::CustomDisplay;

fn main() {
    println!("S7 is {:?}", group::elements(7));
}
