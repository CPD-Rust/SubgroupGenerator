mod permutation;

struct Subset {
    order : usize,
    elements : HashSet<Permutation>,
}

struct Subgroup(Subset);
struct ConjugacyClass(Subset);

// TODO: check for subgroups

pub fn trivial() : Subgroup {
    let mut group = HashSet.new();
    group.insert(permutation::identity());
    group
}
