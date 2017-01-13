\documentclass[a4paper]{article}
%include polycode.fmt
\usepackage{xltxtra}

% allow us to ignore Haskell code
\long\def\ignore#1{}

\begin{document}

\batchmode % silence warnings

We can also verify this by directly calculating all subgroups of $S_5$ and showing all Gaßmann triples are in fact trivial. We made a Haskell program for performing this calculation, and let it run for about 24 hours to accomplish this.

\begin{code}
{-# LANGUAGE TemplateHaskell #-}

module Main where

import Control.Monad
import Debug.Trace
import Data.Foldable hiding (product)
import Data.List hiding (product, findIndex)
import qualified Data.Set as Set
import Prelude hiding (product)
import System.IO
import Test.QuickCheck hiding (elements, generate)
import qualified Test.QuickCheck as QC
\end{code}

Define the set of five elements that $S_5$ acts on. We use a |newtype| to make the |Bounded| instance, and to indicate not all |Int|s are |Five|.
\begin{code}
newtype Five = Five Int
    deriving (Eq, Show, Ord)
instance Enum Five where
    toEnum = Five
    fromEnum (Five x) = x
instance Bounded Five where
    minBound = Five 1
    maxBound = Five 5
\end{code}

Represent an element of $S_5$ as a permutation using a five-tuple, where the value at index $i$ is equal to the value of $i$ after permutation. Of course, to be a permutation, we need to make sure every element of |Five| appears exactly once in this tuple. This restriction is indicated by the |newtype| declaration.
%format S5 = "{S_5}"
\begin{code}
newtype S5 = S5 (Five, Five, Five, Five, Five)
    deriving (Eq, Show, Ord)
\end{code}

The most important element of any group: the identity.
\begin{code}
identity :: S5
identity = S5 (Five 1, Five 2, Five 3, Five 4, Five 5)
\end{code}

Use an element of |S5| as a permutation on |Five|. This makes for easier definition of |S5| composition, as we can just let the first permutation act on the image of the second permutation.
\begin{code}
action :: S5 -> Five -> Five
action (S5 (a, b, c, d, e)) five = case five of
    Five 1 -> a
    Five 2 -> b
    Five 3 -> c
    Five 4 -> d
    Five 5 -> e
\end{code}

However, to make the action work on each value of the tuple, we first need to be able to lift a function to a five-tuple.
\begin{code}
lift5 :: (a -> b) -> (a, a, a, a, a) -> (b, b, b, b, b)
lift5 f (a, b, c, d, e) = (f a, f b, f c, f d, f e)

liftS5 :: (Five -> Five) -> S5 -> S5
liftS5 f (S5 xs) = S5 $ lift5 f xs
\end{code}

Now we can define the product of two group elements, simply by letting the results of the first permutation be acted upon by the second permutation.
\begin{code}
product :: S5 -> S5 -> S5
product elem1 elem2 = liftS5 (action elem1) elem2
\end{code}

To complete the definition of the group |S5|, we want to make an inverse operation. The inverse is calculated by mapping each |Five| in the tuple of |S5| to the index it occupied. This is a slightly tricky process in Haskell since we can't index tuples directly. Hence, this function.
\begin{code}
findIndex :: S5 -> Five -> Int
findIndex (S5 (a, b, c, d, e)) x
    | x == a = 1
    | x == b = 2
    | x == c = 3
    | x == d = 4
    | x == e = 5
    | otherwise = error "findIndex: tuple doesn't contain element"
\end{code}

Make the inverse by mapping each element to the place it originated. Again, tuple indexing is impossible, so we need to manually write out all the cases.
\begin{code}
invert :: S5 -> S5
invert elem@(S5 (a, b, c, d, e)) = S5 (a', b', c', d', e') where
    a' = Five $ findIndex elem $ Five 1
    b' = Five $ findIndex elem $ Five 2
    c' = Five $ findIndex elem $ Five 3
    d' = Five $ findIndex elem $ Five 4
    e' = Five $ findIndex elem $ Five 5
\end{code}

Since the |invert| function isn't by definition total, we introduce a |QuickCheck| property to verify this. If an error occurs or inverting doesn't actually produce an inverse, |QuickCheck| will pick up on this fact.
\begin{code}
prop_invertGivesInverse :: S5 -> Property
prop_invertGivesInverse elem
    = property $ identity == (invert elem `product` elem)
\end{code}

To define operations on more than single elements of |S5|, we make a distinction on the different meanings of sets of |S5|. These are simply distinctions for naming, no extra protection against using the wrong type is implied.

%format Set.Set = "{\mathrm{Set}}"
\begin{code}
type Subset = Set.Set S5
type Subgroup = Set.Set S5
type ConjugacyClass = Set.Set S5
\end{code}

The easiest subgroup to define is of course the trivial one.
\begin{code}
trivial :: Subgroup
trivial = Set.fromList [identity]
\end{code}

Fundamental to our program is the ability to conjugate subgroups with some element. This is done by conjugating each element of the set individually and collecting the result in a |Set.Set|. Using |conjugate| and |generate| (defined below), we will make all the subgroups required.
\begin{code}
conjugate :: Subgroup -> S5 -> Subgroup
conjugate subgroup g
    = Set.map (\h -> invert g `product` h `product` g) subgroup
\end{code}

Generating a subgroup is done by adding all possible products between the set until no more are required. To find the point where all products have been added, we iterate the adding until we reach a fixed point. The |fixpoint| function does this.
\begin{code}
fixpoint :: Eq a => (a -> a) -> a -> a
fixpoint f x
    | f x == x = x
    | otherwise = fixpoint f $ f x
\end{code}

Repeatedly take the product of elements in the generating set. We have to add the identity element so the process non-strictly increases the number of elements. There are $120$ distinct elements of |S5| so this process terminates. Since |S5| is a finite group, all elements have finite order, so we will also generate inverses.
\begin{code}
generate :: Subset -> Subgroup
generate xs = fixpoint generate' (identity `Set.insert` xs) where
    generate' xs
        = foldMap (
            \y -> Set.map (
                \x -> product x y
            ) xs
        ) xs
\end{code}

All the elements of |S5| are now easily computed. We don't want to manually write out all permutations of |S5| so generate them from a smaller set.
\begin{code}
elements :: Subgroup
elements = generate $ Set.fromList
    [ S5 (Five 2, Five 1, Five 3, Five 4, Five 5)
    , S5 (Five 2, Five 3, Five 4, Five 1, Five 5)
    ]
\end{code}

And now we have a set of all elements of |S5|, it's easy to produce an arbitrary example. This allows |QuickCheck| to check the specifications.
\begin{code}
instance Arbitrary S5 where
    arbitrary = QC.elements $ Set.toList $ elements
\end{code}

The following function calculates all subgroups of |S5|. To keep it somewhat speedy, we use the fact that three elements suffice to generate any subgroup of |S5|. However, we still need some billions of computational steps, to find them all. There is a potential definition of the |main| function that fully calculates the set and outputs it, so you can do that computation up front and use its result for significantly quicker results from the other functions.
\begin{code}
allSubgroups :: Set.Set Subgroup
allSubgroups
    = foldMap (\z ->
        foldMap (\y ->
            Set.map (\x ->
                generate $ Set.fromList [x, y, z]
            ) elements
        ) elements
    ) elements
\end{code}

Verify the result of |allSubgroups| using a check for actual subgroupness and some values we can find in the literature. If the set of |allSubgroups| contains $156$ distinct subgroups, we've found them all.
\begin{code}
isSubgroup :: Subgroup -> Property
isSubgroup subgr
    = counterexample "has no identity" hasIdentity
    .&&. (counterexample "is not closed" $
        Set.foldr (\h p ->
            p && Set.foldr (\g p' ->
                p' && isClosed g h
            ) True subgr
        ) True subgr
    ) where
        hasIdentity = Set.member identity subgr
        isClosed g h = Set.member (g `product` invert h) subgr

prop_allSubgroupsCount :: Property
prop_allSubgroupsCount = once $ Set.size allSubgroups == 156
prop_allSubgroupsAreSo :: Property
prop_allSubgroupsAreSo
    = once $ Set.foldr (\subgr p ->
        isSubgroup subgr .&&. p
    ) (property True) allSubgroups
\end{code}

The first half of the code that checks for a non-trivial Gaßmann triple: check the triviality.
\begin{code}
isConjugate :: Subgroup -> Subgroup -> Bool
isConjugate h1 h2 = Set.foldr (||) False $
    Set.map (\g -> conjugate h1 g == h2) elements
\end{code}

To actually determine whether a triple is Gaßmann, we need to take the intersection of each conjugacy class. These classes can be relatively quickly generated by taking all the distinct conjugacy classes of each element of |S5|.
\begin{code}
conjugacyClass :: S5 -> ConjugacyClass
conjugacyClass g = Set.map
    (\h -> invert h `product` g `product` h) elements

conjugacyClasses :: Set.Set ConjugacyClass
conjugacyClasses = Set.map conjugacyClass elements
\end{code}

Verify that the identity element has a trivial conjugacy class, which went wrong during testing of the program.
\begin{code}
prop_conjugacyClassIdentity :: Property
prop_conjugacyClassIdentity = once $
    conjugacyClass identity == trivial
\end{code}

The second half of the code that checks for a non-trivial Gaßmann triple: check the Gaßmannness. As a good optimization, we first check the order of the groups.
\begin{code}
isGassmann :: Subgroup -> Subgroup -> Bool
isGassmann h1 h2 = length h1 == length h2 &&
    all sameIntersectionCount conjugacyClasses where
        sameIntersectionCount conjClass
            = Set.size (conjClass `Set.intersection` h1)
                == Set.size (conjClass `Set.intersection` h2)
\end{code}

A sanity check for |isGassmann|, by taking a known non-Gaßmann pair that can easily get mistaken when there is a small error in the code.
\begin{code}
prop_notGassmannExample :: Property
prop_notGassmannExample = once $ not $
    isGassmann twoCycles doubleCycles where
        twoCycles = generate $ Set.fromList
            [ S5 (Five 4, Five 2, Five 3, Five 1, Five 5)
            , S5 (Five 1, Five 5, Five 3, Five 4, Five 2)
            ]
        doubleCycles = generate $ Set.fromList
            [ S5 (Five 3, Five 2, Five 1, Five 5, Five 4)
            , S5 (Five 4, Five 2, Five 5, Five 1, Five 3)
            ]
\end{code}

Combine |isGassmann| and |isConjugate| into a function that checks for counterexamples.
\begin{code}
isCounterexample :: Subgroup -> Subgroup -> Bool
isCounterexample h1 h2
    = isGassmann h1 h2 && not (isConjugate h1 h2)
\end{code}

The |main| function which is called when the program runs. We have two implementations, either computing all subgroups and outputting them as quickly as possible, or finding a counterexample somewhere in all the subgroups which have been calculated.
\begin{code}
main :: IO ()
main = do
    hSetBuffering stdout NoBuffering
    putStrLn $ show allSubgroups
\end{code}
\begin{code}%this code is hidden to the Haskell compiler
main :: IO ()
main = mapM_ (putCounterexample) [(h1, h2) |
    h1 <- Set.toList allSubgroups,
    h2 <- Set.toList allSubgroups] where
        putCounterexample (h1, h2) = if isGassmann h1 h2
            then if isCounterexample h1 h2
                then putStrLn $
                    "Counterexample!" ++
                    (show h1) ++ " and " ++ (show h2)
                else putStr "G"
            else putStr "-"

\end{code}%this code is hidden to the Haskell compiler

\ignore{
% we want LaTeX to ignore this since evil QuickCheck TemplateHaskell hackery
\begin{code}
return []
runTests = $quickCheckAll
\end{code}
}

\end{document}
