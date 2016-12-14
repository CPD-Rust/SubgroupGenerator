mod permutation;

use permutation::CustomDisplay;

fn main() {
    let identity = permutation::identity(5);
    println!("identity: {}", identity);
    let nonsense = permutation::make_permutation(vec![1, 2, 3, 2, 1]);
    println!("nonsense: {}", nonsense.display());
}
