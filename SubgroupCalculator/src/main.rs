mod permutation;

use permutation::CustomDisplay;

fn main() {
    let identity = permutation::identity(5);
    println!("identity: {}", identity);
    let nonsense = permutation::make_permutation(vec![1, 2, 3, 2, 1]);
    println!("nonsense: {}", nonsense.display());
    for i in 1..6 {
        println!("identity action on {} is {}", i, permutation::action(&identity, i));
    }
    let perm1 = permutation::make_permutation(vec![5, 4, 3, 2, 1]).unwrap();
    let perm2 = permutation::make_permutation(vec![5, 3, 1, 2, 4]).unwrap();
    for i in 1..6 {
        println!("perm1 action on {} is {}", i, permutation::action(&perm1, i));
    }

    println!("composing {} and {} gives {}", perm1, perm2, permutation::composition(&perm1, &perm2));

    println!("inverse of {} is {}", identity, permutation::inverse(&identity));
    println!("inverse of {} is {}", perm2, permutation::inverse(&perm2));
}
