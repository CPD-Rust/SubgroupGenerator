use std::collections::BTreeSet;

mod permutation;
mod group;

use permutation::CustomDisplay;

fn main() {
    let result = group::all_subgroups(3);
    let mut count = 0;
    let visible_output = true;
    for group in &result {
        count += 1;
    }
    println!("Total: {}", count);
    if visible_output {
        for group in &result {
            println!("\n {}",group);
        }
    }
}
