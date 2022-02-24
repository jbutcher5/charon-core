# The W Programming Language

A highly abstracted mathematical programming language.

## Core Concept

The W Programming Language (or wlang as I shall refer to it) is programming language unlike any other. The core idea is that each line of code is it's own array that then consumes and evaluates it's self. To achieve this wlang trys to abstract it's self as much as it can. For example functions, procedures and variables all fall under what in wlang is known as a container.

## Basic Syntax

Wlang is a postfix programming which means the parameters are followed by the function. For example `1 2 add` would evaluate to 3 and the value 3 would replace `1 2 add` so all following functions would see 3 not `1 2 add`, for example the code `1 2 add 4 sub` would add 1 and 2 then subtract the result of that, which is 3, by 4 to result with -1.

A clear example of how everything in wlang is an array is the code `1 3 add 8 OUTPUT`. In the code first 1 and 3 would be consumed by add and replaced with 4 which then leaves us with the code `4 8 OUTPUT`. The OUTPUT function is much like a print command in any other programming language and would write to the stdout `4 8` as `OUTPUT` display all the infomation that comes before it and then only consumes it's self such that you could chain the `OUTPUT` followed by a function or container which may look like `1 2 add 8 OUTPUT mul` which would print `4 8` then evaluate to 32 because it multiplys 4 and 8.

## Common Patterns

There are no for/while loops in wlang. All loops can be done through container recursion. This may look like the following:

```
x <- { $0 1 add OUTPUT x } { $0 1 add OUTPUT } ( $0 10 eq ) if-else
0 x
```

This code would output the numbers 1 to 11. It achieves the recursion by calling x from within it's self an increments the parameter by 1 which is then checked if it equals 10 and the if-else provides different paths of code for the language based on the value of the first parameter. 
