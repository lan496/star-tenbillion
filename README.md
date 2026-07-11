# Star Tenbillion

This is a reproducible GAP analysis and a practical solution for Nintendo's
Star Tenbillion, the 2007 remake of Gunpei Yokoi's Ten Billion Barrel.

## Run the analysis

Install [pixi](https://pixi.sh), then run:

```sh
pixi install
pixi run analyze
pixi run test
```

The environment follows the same project-local GAP setup as
[`lan496/gap-sandbox`](https://github.com/lan496/gap-sandbox). GAP and its
version are locked in `pixi.lock`; nothing needs to be installed globally.

## Move notation

Hold the puzzle upright with its core in the normal (up) position. `T` and `B`
turn the upper and lower two-row discs one fifth-turn to the left. Lower-case
`t` and `b` are the corresponding turns with the core pushed down. A lower-case
`l` or `r` in a move word says left or right, so `tr` means one right turn of
the top disc with the core down. Two letters, such as `Tll`, mean two turns.

Call the five columns `A` through `E`, with `C` at the front. Number rows from
the bottom upward, so row 1 is the bottom and the three-ball cap is row 5.

Moving the core between upper- and lower-case moves is implicit. Return it to
the normal position after a lower-case move sequence.

## A solution

Three short algorithms do almost all of the work:

| Name | Repeat this sequence | Effect |
|---|---|---|
| 9-cycle | `tr br Tl Bl` 11 times | Cycles the nine balls in two working columns by two places |
| top 5-cycle | `tr Tl` 7 times | Cycles the five balls in the upper part of the working columns by two places |
| bottom 5-cycle | `br Bl` 7 times | Cycles the five balls in the lower part of the working columns by two places |

Their inverses run the same cycles backwards. These give the following
systematic solve.

1. Put two odd-colored balls in one column, but not `A` or `E`. If necessary,
   use the 9-cycle to position one ball at `C1`, shift the pair with disc turns,
   and use `br Bl` to stack them. Turn both discs to move the pair to column
   `D`; repeat the 9-cycle until the two balls reach `A5` and `E5`.
2. Choose a four-ball color for column `A`. Repeat the 9-cycle until a ball of
   that color is at `B1`. Do `Tr Br`, run the 9-cycle five times, then do
   `Tl Bl`; this inserts the ball below any already solved balls in `A`.
   Repeat until all four balls fill column `A`.
3. Do `Tl Bl`, moving the solved column to `E`. Solve `A` again with a second
   color. Do `Tl Bl` once more, putting the solved columns at `D` and `E`, then
   solve `A` with a third color. Only `B` and `C` remain unsolved.
4. Choose the color for `B`. Use the top 5-cycle until a ball of that color is
   at `C3`, then use the bottom 5-cycle three times (or its inverse four times)
   to move it to `C2`. Repeat until all four balls of the color are in the
   bottom two rows. Use the top 5-cycle to put the remaining odd-colored ball
   at `B3`, then use the 9-cycle to finish both columns.

The algorithms and this column-by-column procedure are the compact solution
published by Pedro Luis and documented by Jaap Scherphuis. The GAP model checks
the stated 5- and 9-cycle supports directly.

## Group structure

Temporarily label all 23 balls distinctly. In the normal core position, a disc
turn is a product of two disjoint 5-cycles. Conjugating the same turns by the
core motion gives four generators in `gap/model.g`:

```text
T = (4,5,6,7,8)(9,10,11,12,13)
B = (14,15,16,17,18)(19,20,21,22,23)
t = (1,5,2,3,8)(4,10,6,7,13)
b = (9,15,11,12,18)(14,20,16,17,23)
```

GAP proves

```text
G = <T,B,t,b> = A_23,
|G| = 23!/2 = 12,926,008,369,442,488,320,000.
```

The parity restriction is immediate: every generator is even. The substantive
part is that there is no further invariant; GAP's stabilizer-chain computation
identifies the generated transitive group as the full alternating group.
Consequently every even permutation of distinctly labelled balls is reachable
without turning the whole toy end-for-end. Turning it over supplies an odd
permutation, enlarging the group to `S_23`.

With the actual indistinguishable balls (five colors occurring four times and
one color occurring three times), odd swaps within a same-color class erase
the parity distinction. Thus every color arrangement is reachable. There are

```text
23! / (3! (4!)^5) = 541,111,756,185,000
```

color arrangements, or

```text
23! / (3! (4!)^5 5!) = 4,509,264,634,875
```

if the names of the five four-ball colors—and hence their solved columns—are
regarded as interchangeable. The latter is about 451 times the advertised
ten billion.

## Sources

- [Star Tenbillion overview](https://nintendo.fandom.com/wiki/Star_Tenbillion)
- [Puzzle mechanics and solution](https://www.jaapsch.net/puzzles/nintendo.htm)
- [David Singmaster's group/counting discussion](https://www.jaapsch.net/puzzles/cubic2.htm)
- [Nintendo Tumbler Puzzle overview](https://en.wikipedia.org/wiki/Nintendo_Tumbler_Puzzle)
