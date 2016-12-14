use std::fmt;

// We would like to add an extra type parameter, making the size part of the type.
// TODO: check if this is possible
// In order to enforce that we get an actual permutation, we want to wrap the vector
// in a newtype and only use the constructor function that checks the invariants.
#[derive(Debug)]
pub struct Permutation {
    // We represent the permutation as a mapping from int to int
    permutation: Vec<usize>,
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.permutation)
    }
}

pub struct PermutationDisplay<'a>(&'a Option<Permutation>);

pub trait CustomDisplay {
    fn display<'a>(&'a self) -> PermutationDisplay<'a>;
}

impl CustomDisplay for Option<Permutation> {
    fn display<'a>(&'a self) -> PermutationDisplay<'a> {
        PermutationDisplay(self)
    }
}

impl<'a> fmt::Display for PermutationDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let PermutationDisplay(contents) = *self;
        match *contents {
            Some(ref perm) => write!(f, "{}", perm),
            None => write!(f, "Nothing"),
        }
    }
}

pub fn make_permutation(mapping : Vec<usize>) -> Option<Permutation> {
    // Each object should occur exactly once in the map.
    for object in 1..mapping.len() + 1 {
        let mut count = 0;
        for mapped in &mapping {
            if *mapped == object {
                count += 1;
            }
        }
        if count != 1 {
            return None;
        }
    }
    Some(Permutation{ permutation: mapping })
}

// Return the identity element of the permutation group with given order.
pub fn identity(order : usize) -> Permutation {
    let mapping = (1..order+1).collect::<Vec<usize>>();
    make_permutation(mapping).expect("could not generate identity element")
}

pub fn action(perm : &Permutation, object: usize) -> usize {
    if object < 1 || object > perm.permutation.len() {
        panic!("The group action is not defined for this number");
    }
    perm.permutation[object-1]
}

pub fn composition(perm1 : &Permutation, perm2 : &Permutation) -> Permutation {
    let mapping = perm1.permutation.map(|x|.action(perm2, x));
    make_permutation(mapping).expect("Composition of permutations is not permutation")
}
