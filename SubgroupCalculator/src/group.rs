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

fn subset_size(elements : &HashSet<permutation::Permutation>) -> Option<usize> {
    // TODO: we wanted to just take "an element" from the elements
    // but that requires iterators which take a mutable reference,
    // and then this happened.
    let mut size : Option<usize>= None;
    for elem in &elements {
        match size {
            None => {
                size = Some(elem.permutation.len());
            }
            Some(expected_size) => {
                if elem.permutation.len() != expected_size {
                    return None;
                }
            }
        }
    };
    size
}

fn check_closed(subset : Subset) -> Option<Subgroup> {
    // We need to check the group is closed under all operations.
    let elements = &(subset.elements);
    for g in elements {
        for h in elements {
            let tempelem = permutation::composition(g,&permutation::invert(&h));
            if !elements.contains(&tempelem) {
                return None;
            }
        }
    }

    Some(Subgroup(subset));
}

fn make_subset(elements : HashSet<permutation::Permutation>) -> Option<Subset> {
    subset_size(&elements)
        .and_then(|size| => Subset { size: size, elements: elements })
}

// When the set is a group, wrap it in the Subgroup type.
pub fn make_subgroup(elements : HashSet<permutation::Permutation>) -> Option<Subgroup> {
    // First we make it into a Subset by checking the sizes.
    // For this, we need at least one element.
    make_subset(elements)
        .and_then(|subset| => check_closed(subset))

}

// Generate the trivial group on the given number of elements.
pub fn trivial(size : usize) -> Subgroup {
    let mut group = HashSet::new();
    group.insert(permutation::identity(size));
    Subgroup(Subset {size: size, elements: group})
}

