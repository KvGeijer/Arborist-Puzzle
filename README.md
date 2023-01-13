# North Pole Arborist Puzzle

This repo contains the code for a puzzle I created for my flatmate [Samuel Sellec](https://www.linkedin.com/in/samuel-selleck-027a41150/) who in return made me one. They were both inspired by the fantastic [Advent of Code](https://adventofcode.com/2022/about) which [Eric Wastl](http://was.tl/) sets up each year. Finalizing this puzzle took more time than I had imagined, especially ironing out the ideas and writing the problem descriptions took longer than I had anticipated. It made me even more impressed by what Eric does every year, although I hope he has a better workflow.

## Finding the problem

To try the problem for yourself you can find the problem description in [part1.md](part1.md). I have not set up a good way to validate answers or generate personalized inputs, but that's how it is. Contact me if you want to validate your answer. If you are confident you can proceed directly to [part2.md](part2.md), which is where it in my opinion gets interesting ;)

## My solution

Of course I have made my own solution, which is in Rust and included in the [interpreter](interpreter) folder. I tried to make it nice, and included some extra features such as a precompiler and a randomized expression formatter. Have a look if you want.

## Puzzle idea (Spoilers)
As a sidenote the main idea of the puzzle is to make a _super_ simple interpreter which at its core has some relatively high level functionalities. In the end it was mainly functions and how the scoping worked which became a little advanced. It would be nice to implement some more things, but I am quite happy with how simple it became this way.