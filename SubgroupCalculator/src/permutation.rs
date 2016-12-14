use std::fmt;

// We would like to add an extra type parameter, making the size part of the type.
// TODO: check if this is possible
// In order to enforce that we get an actual permutation, we want to wrap the vector
// in a newtype and only use the constructor function that checks the invariants.
pub struct Permutation {
    // We represent the permutation as a mapping from int to int
    permutation: Vec<usize>,
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.permutation)
    }
}

pub fn make_permutation(mapping : Vec<usize>) -> Option<Permutation> {
    // Each object should occur exactly once in the map.
    for object in (1..mapping.len() + 1) {
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
