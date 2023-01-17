## Part Two

After evaluating the root you hear someone sniggering in the far corner. Apparently it is an IT elf who decided to hide in the factory while all others went to the tropical island. A way to skip work without going to an icky sticky island, smart!

The elf continues laughing and then tells you that your interpretation of the syntax tree is severely outdated. Nowdays the syntax tree is only used to grow the more complex _binding tree_ where each limb binds a value to a branch expression. Limbs are grown from the _active limb_  by [interpreting](https://craftinginterpreters.com/) expressions in the syntax tree similarly to last time. The active limb can change during evaluation of an expression, but always returns when the evaluation is done.

To evaluate a branch expression you still check the value of the first expression:
* If the value is 0: The binding tree grows a new limb which binds the value of the second expression to the third expression. That limb becomes the active limb while the fourth expression is evaluated and returned.
* If the value is 1: Conditionally _evaluates_ and returns the third or fourth expression like last time.
* If the value is 2: Grows and drops a _flower_ containing the value of the second expsession.
* If the value is 3: Subtracts the values of the second and third expressions, like last time.
* Oterwise, for value _x_: The remaining expressions are evalued and their values become arguments, numbered -1, -2... Searching towards the root for the first limb binding x to some _e_, from that limb grows a long limb binding all arguments to their numbers. The long limb becomes the active one while e is evaluated and returned. 

Each binding tree spells out a message when its dropped flowers are interpreted as ASCII values. 

The large example from the last part gives the message `XXXZAxX` and is represented by a [psuedo root](https://en.wikipedia.org/wiki/Pseudocode) below. For improved readability some recurring values are replaced by strings (```BIND = 0```, ```IF = 1```, ```FLOWER = 2```, ```SUB = 3```, ```FLOWERX = 8```, ```ADD = 9```, ```X = 10```, ```Y = 11```).
```
(BIND 4 0 
	(BIND X 88
		(BIND FLOWERX (FLOWER (X))
			(4 
				(FLOWER 88)
				(FLOWER (X))
				(FLOWERX)
			
				(BIND ADD (SUB (-1) (SUB 0 (-2)))
					(4 
						(FLOWER (ADD (X) 2))
						(FLOWER (SUB (X) 23))
					)
				)
				(BIND Y 89 
					(IF (SUB (X) (Y)) 
						-1
						(BIND X 120
							(4
								(FLOWER (X))
								(FLOWERX)
							)
						)
					)
				)
			)
		)
	)
)
```
Or still in its normal form, same as last time:
```
(0 4 0(0 16 88(0 33(2(16))(4(2 88)(2(16))(33)(0 32(3(-1)(3 0(-2)))(4(2(32(16)2))(2(3(16)23))))(0 17 89(1(3(16)(17))(999)(0 16 120(4(2(16))(33)))))))))
```

What message does the flowers from the real root spell?
```
(0 32(3(-1)(3 0(-2)))(0 4 0(0 33(1(-2)(32(-1)(33(-1)((3 -3 -6)((3 14 16))(3 -6 -7))))0)(0 34(1(32(-1)1)(32(3 -19 -20)(34(((3(3 34((3 17 14)22 11))20)22 19)(-1)((3 -22 -20)))(-2)))-1)(0 37(1(-1)((3 11 11)35(34(-1)10)((3 -13 -13)36(3(-1)((3(3 44(3(3(3 15 10)-6)-3))-3)(35)10))((3 -13 -17)(37((3 53 18)))((3 -2 -4)(32 48((3 54 18)))))))0)(0 38(4((3(3 19 -18)0)(((3 -4(3(3 -19 7)-19))((3 -4 -7)-15 5)-17)))((3 -6 -7)(-1)(((3 16 13)23(3 31 12))(2 44)(38(3(-1)1)(-3)(32(-2)(-3))))0))((3(3 50 16)-4)12 0 1)))))))
```
