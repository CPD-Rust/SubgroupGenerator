use permutation;
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn subset_size(elements : &HashSet<permutation::Permutation>) -> Option<usize> {
    // TODO: we wanted to just take "an element" from the elements
    // but that requires iterators which take a mutable reference,
    // and then this happened.
    let mut size : Option<usize>= None;
    for elem in elements {
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
    {
        let elements = &(subset.elements);
        for g in elements {
            for h in elements {
                let tempelem = permutation::composition(g,&permutation::invert(&h));
                if !elements.contains(&tempelem) {
                    return None;
                }
            }
        }
    };

    Some(Subgroup(subset))
}

pub fn make_subset(elements : HashSet<permutation::Permutation>) -> Option<Subset> {
    subset_size(&elements)
        .map(|size| Subset { size: size, elements: elements })
}

// When the set is a group, wrap it in the Subgroup type.
pub fn make_subgroup(elements : HashSet<permutation::Permutation>) -> Option<Subgroup> {
    // First we make it into a Subset by checking the sizes.
    // For this, we need at least one element.
    make_subset(elements)
        .and_then(|subset| check_closed(subset))

}

// Generate the trivial group on the given number of elements.
pub fn trivial(size : usize) -> Subgroup {
    let mut group = HashSet::new();
    group.insert(permutation::identity(size));
    make_subgroup(group).unwrap()
}

pub fn conjugate( subgroup : &Subgroup, g : &permutation::Permutation) -> Subgroup {
    let mut newgroup = HashSet::new();
    let Subgroup(ref elem_set) = *subgroup;
    for elem in &elem_set.elements {
        let conjugate_elem = permutation::composition(g,&permutation::composition(elem,&permutation::invert(g)));
        newgroup.insert(conjugate_elem);
    }
    make_subgroup(newgroup).unwrap()
}

pub fn generate(generators : &Subset) -> Subgroup {
    let mut result = HashSet::new();
    {
        let mut to_visit = VecDeque::new();
        result.insert(permutation::identity(generators.size));
        for elem1 in &generators.elements {
            for elem2 in &generators.elements {
                to_visit.push_back((elem1.clone(), elem2.clone()));
            }
            result.insert(elem1.clone());
        }
        while let Some((elem1, elem2)) = to_visit.pop_front() {
            let product = permutation::composition(&elem1, &elem2);
            if result.insert(product.clone()) {
                // since we move product to the set, we have to know where it
                // ends up
                for elem1 in &result {
                    to_visit.push_back((elem1.clone(), product.clone()));
                    to_visit.push_back((product.clone(), elem1.clone()));
                }
            }
        }
    }
    make_subgroup(result).unwrap()
}

pub fn elements(size : usize) -> Subgroup {
    let mut cycle = (2..size+1).collect::<Vec<usize>>();
    cycle.push(1);
    let mut transposition = vec![2, 1];
    transposition.extend(3..size+1);
    let gen1 = permutation::make_permutation(cycle).unwrap();
    let gen2 = permutation::make_permutation(transposition).unwrap();
    let generators = make_subset([gen1, gen2].iter().collect()).unwrap();
    generate(&generators)
}
