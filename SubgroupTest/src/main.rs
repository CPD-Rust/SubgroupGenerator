//TODO Run check functie bij aanmaken van PermElements en geef error als fout gaat.
// An element of S_n is represented as an n-vector where the i-th position gives the value of i
// after the permutation.

struct PermElement {
    //PermElement is an element of S_n so the numbers in the permutation can be no larger than n.
    max_n: u32,

    // The length of the vector, i.e. is it a transposition or a triple etc.
    size: usize,

    // We represent the actual permutation as a vector.
    permutation: Vec<u32>,
}

trait Ispermelement {
    fn check(&self);
}

impl Ispermelement for PermElement {
    fn check(&self) {
        //Check if all elements are indeed in the interval [1, max_n]
        let mut tempbool = true;
        for x in &self.permutation {
            tempbool = tempbool && (x <= &self.max_n) && (*x >= 1);
        }

        // Check if there are no duplicate elements in our permutation.
        let mut tempvec = self.permutation.clone();
        tempvec.dedup();

        // Check if the permutation is the correct size and combine the results.
        let result = tempvec == self.permutation && self.size == self.permutation.len() && tempbool;

        if !result {
            panic!("This is not a well-defined permutation!");
        }
    }
}

// We gebruiken newtype
struct Perm(PermElement);

fn action(perm: &PermElement, int: &u32) -> u32 {
    if (int < &1 || int > &perm.max_n) {
        panic!("The group action is not defined for this number!");
    }
    else {
        let returnval = perm.permutation[(int-1) as usize].clone();
        returnval
    }
}

fn composition(perm1: &PermElement, perm2: &PermElement) -> PermElement {
    let resultvec: Vec<_> = perm1.permutation.iter().map(|&x| action(&perm2, &x)).collect();
    let returnperm = PermElement { max_n: perm2.max_n, size: perm2.size, permutation: resultvec };
    returnperm
}

fn main() {
    let testperm = PermElement { max_n: 3, size: 3, permutation: vec![2, 1, 3] };
    let Perm(newtypetest) = Perm(PermElement { max_n: 3, size: 3, permutation: vec![3, 1, 2] });
    testperm.check();

    //Doe even S3 voor nu.
    let identity = PermElement { max_n: 3, size: 3, permutation: vec![1, 2, 3] };

    println!("{}", action(&identity, &2));
    println!("{}", action(&testperm, &2));
    println!("{}", action(&newtypetest, &2));
    println!("{:?}", composition(&testperm, &newtypetest).permutation);
    println!("{:?}", composition(&testperm, &identity).permutation);
    println!("{:?}", composition(&identity, &newtypetest).permutation);
}
