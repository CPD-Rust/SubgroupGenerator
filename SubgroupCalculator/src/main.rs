use std::collections::HashSet;

mod permutation;
mod group;

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

    println!("inverse of {} is {}", identity, permutation::invert(&identity));
    println!("inverse of {} is {}", perm2, permutation::invert(&perm2));

    println!("trivial group is {:?}", group::trivial(5));

    let perm2_2 = permutation::composition(&perm2, &perm2);
    let perm2_3 = permutation::composition(&perm2, &perm2_2);
    let perm2_4 = permutation::composition(&perm2, &perm2_3);
    let perm2_5 = permutation::composition(&perm2, &perm2_4);
    let elements : HashSet<_> = [perm2, perm2_2, perm2_3, perm2_4, perm2_5].iter().cloned().collect();
    let testgroup = group::make_subgroup(elements).unwrap();
    println!("another group is {:?}", &testgroup);
    println!("Conjugate test: {:?}", group::conjugate(&testgroup,&perm2));
}
