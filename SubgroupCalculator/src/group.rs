use permutation;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Subset {
    // The number of objects the permutations act on.
    size : usize,
    // The permutations in the set.
    elements : HashSet<permutation::Permutation>,
}

#[derive(Debug)]
pub struct Subgroup(Subset);
#[derive(Debug)]
pub struct ConjugacyClass(Subset);

// TODO: check for subgroups

// Generate the trivial group on the given number of elements.
pub fn trivial(size : usize) -> Subgroup {
    let mut group = HashSet::new();
    group.insert(permutation::identity(size));
    Subgroup(Subset {size: size, elements: group})
}
