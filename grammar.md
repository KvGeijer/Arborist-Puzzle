# Grammar Ideas

This document is for myself to flesh out the ideas for my grammar. 

## Name

maybe something related to Lisp, since it is built around those horrible brackets

## Formal Grammar

program := expr                     \
expr := '(' func ')'

func := expr expr*                 \
    | LET '(' binding* ')' expr    \
    | IF expr expr expr            \
    | PRINT expr                   \
    | BLOCK block

block := '(' expr* ')'              \
binding := '(' expr block expr ')'  

int := chars | digits               \
chars := char+                      \
char := ([!-']|[\*-/]|[:-}])        \
digits := 0|[1-9]+            

### Functions

* eval
  * The normal function call which is when none of the special ones are invoked. It evaluates the arguments, then creates a new namespace where the arguments are bound to the parameters of the function defined in the local scope as the first expression. TODO: Should the namespace have access to any variables outside if the function definition?

* let
  * Binds several ints to functions which are added to the nested namespace. Returns the result of that expression.
* if 
  * A normal if then else functional statement.
* print
  * Prints the int as an ascii char. So it can be assumed to be valid ascii. Does not return anything.
* block
  * Evaluates the expressions in order and returns the value of the last one. 

### Description

Each expression should just be a pair of parantheses, where we invoke the function/subroutine that is the first value in the list, and use the rest as arguments. We have a few special functions, and the 

At first I thought we should only use global variables, but that seems too annoying, so I will allow local ones as well. Primarily makes functions more reasonable.

The only data type should be 64 bit ints (8 chars). These can be represented as a sequence of digits, but they can also be represented as up to 8 ASCII chars (33-126), disallowing parantheses (40,41) and digits (48-57) (just to make parsing simple). Don't specify what to do with overflow.

## Example