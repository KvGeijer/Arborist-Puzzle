# The North pole arborist

Arriving at the North pole you are greeted by an eerily empty factory. Aimlessly wandering the corridors you finally spot a deflated Santa muttering incoherently to himself about the elves running off to some [tropical island](https://adventofcode.com/2022/day/1). Without them the factory will descend into disarray, and it might be impossible to even find all presents on Christmas. 

To stave off this impending disaster you decide to help Santa with the housekeeping until the elves return. After all, you have always dreamt of beeing a magical maid. Preferably you would have a morningstar and a horn as well, but there is always room for improvement.

You start by going to the garden only to find that their rumored [_syntax tree_](https://en.wikipedia.org/wiki/Abstract_syntax_tree) has wilted and died. Regrowing and evaluating the root of the tree will make the perfect beginning of your new career.

From your academic days you remember that syntax trees grow from a very specific set of recursively nested _growth expressions_. Each such expression can be represented by either a signed integer (a _leaf_), or a sequence of expressions enclosed by a pair of parantheses (a _branch_). For example:

```
(0 4 0(0 16 88(0 33(2(16))(4(2 88)(2(16))(33)(0 32(3(-1)(3 0(-2)))(4(2(32(16)2))(2(3(16)23))))(0 17 89(1(3(16)(17))(999)(0 16 120(4(2(16))(33)))))))))
```

After nurturing the tree back to a lively state you can then evaluate it recursively from the root. The value of an leaf is its integer. The value of a branch depends on the value of its first expression:
* If it is 1 it first evaluates the second expression:
  * If that is larger than 0, it returns the value of the third expression
  * Otherwise, it returns the value of the fourth expression
* If it is 3 it instead returns the value of the second expression subtracted by the value of the third expresion
* Otherwise it returns the sum of the values of all the contained expressions.

Some simple example roots: 
```
(0 1 2 3) -> 6
( 3  5 4) -> 1
((3) 5 4) -> 1
(1 0 1 2) -> 2
```
and the value of the larger example above is 681. 

Looking aroud you find a root expression admist the remains of the old tree. What is the value of the root if you grow the tree following the expression?
```
(0 32(3(-1)(3 0(-2)))(0 4 0(0 33(1(-2)(32(-1)(33(-1)((3 -3 -6)((3 14 16))(3 -6 -7))))0)(0 34(1(32(-1)1)(32(3 -19 -20)(34(((3(3 34((3 17 14)22 11))20)22 19)(-1)((3 -22 -20)))(-2)))-1)(0 37(1(-1)((3 11 11)35(34(-1)10)((3 -13 -13)36(3(-1)((3(3 44(3(3(3 15 10)-6)-3))-3)(35)10))((3 -13 -17)(37((3 53 18)))((3 -2 -4)(32 48((3 54 18)))))))0)(0 38(4((3(3 19 -18)0)(((3 -4(3(3 -19 7)-19))((3 -4 -7)-15 5)-17)))((3 -6 -7)(-1)(((3 16 13)23(3 31 12))(2 44)(38(3(-1)1)(-3)(32(-2)(-3))))0))((3(3 50 16)-4)12 0 1)))))))
```

