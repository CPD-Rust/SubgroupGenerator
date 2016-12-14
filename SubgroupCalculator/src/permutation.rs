// We would like to add an extra type parameter, making the size part of the type.
// TODO: check if this is possible
// In order to enforce that we get an actual permutation, we want to wrap the vector
// in a newtype and only use the constructor function that checks the invariants.
struct Permutation {
    // We represent the permutation as a mapping from int to int
    permutation: Vec<u32>,
}

fn makePermutation(mapping : Vec<u32>) -> Option<Permutation> {
    None
}
