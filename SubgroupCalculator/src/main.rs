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
    let elements : HashSet<_> = [perm2.clone(), perm2_2, perm2_3, perm2_4, perm2_5].iter().cloned().collect();
    let testgroup = group::make_subgroup(elements).unwrap();
    println!("another group is {:?}", &testgroup);
    println!("Conjugate test: {:?}", group::conjugate(&testgroup,&perm1));

    let gen1 = permutation::make_permutation(vec![2,3,1,4,5]).unwrap();
    let gen2 = permutation::make_permutation(vec![2,1,3,4,5]).unwrap();
    let gen3 = permutation::make_permutation(vec![1,2,3,5,4]).unwrap();
    let generators : HashSet<_> = [gen1.clone(), gen2.clone(), gen3.clone()]
        .iter().cloned().collect();
    let generated = group::generate(&group::make_subset(generators).unwrap());
    println!("Group generated is {:?}", generated);

    println!("S3 is {:?}", group::elements(3));
}
