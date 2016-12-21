use std::collections::HashSet;

mod permutation;
mod group;

use permutation::CustomDisplay;

fn main() {
    let result = all_subgroups(3);
    println!("All subgroups are {:?}", result);
}
