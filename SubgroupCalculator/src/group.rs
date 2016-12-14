use permutation;
use std::collections::HashSet;

#[derive(Debug)]
struct Subset {
    size : usize,
    elements : HashSet<permutation::Permutation>,
}

#[derive(Debug)]
struct Subgroup(Subset);
#[derive(Debug)]
struct ConjugacyClass(Subset);

// TODO: check for subgroups

pub fn trivial(order : usize) -> Subgroup {
    let mut group = HashSet::new();
    group.insert(permutation::identity(order));
    Subgroup(Subset {size: order, elements: group})
}
