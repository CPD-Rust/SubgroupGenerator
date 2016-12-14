use permutation;
use std::collections::HashSet;

struct Subset {
    order : usize,
    elements : HashSet<permutation::Permutation>,
}

struct Subgroup(Subset);
struct ConjugacyClass(Subset);

// TODO: check for subgroups

pub fn trivial(order : usize) -> Subgroup {
    let mut group = HashSet.new();
    group.insert(permutation::identity(order));
    group
}
