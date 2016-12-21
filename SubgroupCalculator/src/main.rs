use std::collections::BTreeSet;

mod permutation;
mod group;

use permutation::CustomDisplay;

fn main() {
    let result = group::all_subgroups(3);
    for group in &result {
        println!("Subgroup");
    }
}
