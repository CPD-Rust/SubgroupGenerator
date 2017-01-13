use std::collections::BTreeSet;

mod permutation;
mod group;

use permutation::CustomDisplay;

fn main() {
    let result = group::all_subgroups(5);
    let mut count = 0;
    for group in &result {
        println!("Subgroup");
        count += 1;
    }
    println!("Total: {}", count);
}
