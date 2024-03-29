# Grammar Ideas

This document is for myself to flesh out the ideas for my grammar. It is very outdated, and the ideas changed a lot in the end. But can be fun to look back on.

## Name

Maybe something related to Lisp, since it is built around those horrible brackets

## Formal Grammar

program := expr                     \
expr := '(' func ')'                \
    | int

func := expr expr*                  \
    | LET expr expr                 \
    | IF expr expr expr             \
    | PRINT expr                    \
    | BLOCK '(' expr+ ')' # Replaced with empty func\
    | SUB expr expr

int := 0|-?[1-9][0-9]*            

### Functions

* eval
  * The normal function call which is when none of the special ones are invoked. It evaluates the id, arguments, then creates a new namespace where each argument is bound to the nagative index of it's position in. So te first arg will be bound to -1, second -2 and so on. This way the declarations of the functions don't have to worry about parameters at all, just assume they will be bound to certain things. Also nice to recommend negative numbers not be identifiers to get around claches (unless you want to use them for default arguments).

* let
  * Binds an identifier to a function which is added to the current namespace. Returns the result of that expression.
* if 
  * A normal if then else functional statement. It should probably not create a nested namespace, but instead continue operating in the current one.
* print
  * Prints the expr as an ascii char. So it can be assumed to be valid ascii. Returns the value of the expression.
* block
  * First opens a nested local namespace. Then evaluates the expressions in order and returns the value of the last one. 

## Namespaces

This is not as easy as I initially thought. Thanks to the book Crafting interpreters for alerting me to the importance of closures if we want to treat functions as first class citicens (which this language does). The main issue is what to do if a function outlives the local namespace, while still using its variables. A simple way to solve this would be if all namespaces where nested hashmaps, and we just never garbage collected them until after they are out of use. This would not be that hard, although not efficient either. But a good solution. I think the nested map structure is the most intuitive way to do it as well, so people might even make it possible without realizing it.

### Description

Each expression should just be a pair of parantheses, where we invoke the function/subroutine that is the first value in the list, and use the rest as arguments. We have a few special functions, which don't really work as normal ones.

At first I thought we should only use global variables, but that seems too annoying, so I will allow local ones as well. Primarily makes functions more reasonable.

We should only allow ints as the only data type. Before I thought we should allow strings, which represented ints, but that feels like unnecessary work for the programmer toimplement. Here is my old idea:
> The only data type should be 64 bit ints (8 chars). These can be represented as a sequence of digits, but they can also be represented as up to 8 ASCII chars (33-126), disallowing parantheses (40,41) and digits (48-57) (just to make parsing simple). Don't specify what to do with overflow.

## Building the question

For me to create input I should first specify a program, either in real plaintext, or in some extended code with for example the special functions as keywords. Then I should convert that to a real AST. From that I could probably insert some extra confusion, for exampe by expanding 10 to (- 15 (- 7 2)), which makes the code harder to read by a human. 

I of course also have to create my own interpreter of the language, which might be a goof first step. That way I can test if the generated input is correct. Maybe I should also have some faulty interpreters, to make sure the input does not work for them...

What exactly the question and answer should be is not trivial.

Here are some obfuscation ideas:
* Expand ints to arithmetic expressions
* Create unnecessary functions
* Declare functions at different levels, and maybe re-declare them


### Parts

There should be several parts to the puzzle
1. Parse the recursive structure, keep track of something.
2. Learn the rules of the language, interpret it and get visual? output
3. Maybe give a more complicated input which does the same thing, but tests if you have implemented namespaces correctly, and nests stuf in a really wierd way. But maybe too annoying. 


## Extra features

I will implement a few extra features to make development easier for myself.

1. Precompiler macros, like in C
  - Lines starting with # are treated as precompiler macros
  - Macros names can only be strings of uppercase chars
  - The first word following # is the id
  - After the following space, everyhing until the newline becomes the macro value, which is substitued for the identifier in all places before the code is interpreted.
2. Built in macros
  - LET
  - BLOCK
  - IF
  - SUB
  - PRINT
3. Comments
  - All lines beginning with // will be removed by the precompiler
  
## Example