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

// When the set is a group, wrap it in the Subgroup type.
pub fn make_subgroup(elements : HashSet<permutation::Permutation>) -> Option<Subgroup> {
    // First we make it into a Subset by checking the sizes.
    // For this, we need at least one element.
    let mut iter = elements.iter();
    match iter.next() {
        None => {
            return None
        }
        Some(referenceElem) => {
            let size = referenceElem.permutation.len();
            for elem in iter {
                if elem.permutation.len() != size {
                    return None;
                }
            }
        }
    };

    // Then we need to check the group is closed under all operations.
    for g in &elements {
        for h in &elements {
            let tempelem = permutation::composition(g,&permutation::invert(&h));
            if !elements.contains(&tempelem) {
                return None;
            }
        }
    }

    Some(Subgroup(Subset { size: size, elements: elements }))
}

// Generate the trivial group on the given number of elements.
pub fn trivial(size : usize) -> Subgroup {
    let mut group = HashSet::new();
    group.insert(permutation::identity(size));
    Subgroup(Subset {size: size, elements: group})
}

