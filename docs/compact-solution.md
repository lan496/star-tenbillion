# A compact Star Tenbillion solution

The Pedro Luis solution uses three reusable algorithms. A method attributed to
Naoaki Takashima showed that one three-cycle is enough. An exhaustive GAP
search finds a shorter three-cycle with a simple mirrored pattern.

## One algorithm

In this project's notation, use

```text
C = Tl bll Tl | br Trr br
```

The bar separates two mirrored halves. Remember it as:

```text
top-bottom-top,    left  1-2-1
bottom-top-bottom, right 1-2-1
```

Only the normal-core top disc (`T`) and lowered-core bottom disc (`b`) occur.
GAP calculates

```text
C = T b^2 T b^-1 T^-2 b^-1 = (11,13,18).
```

The result moves exactly three balls and fixes the other 20. Applying `C`
twice reverses the cycle, so no inverse algorithm needs to be memorized.

Run the exact check with:

```sh
pixi run compact
pixi run search-compact
pixi run test
```

## Compactness result

The search treats any nonzero turn of one disc in one core position as one
face-turn-metric (FTM) move. Thus `l`, `ll`, `rr`, and `r` each cost one move.
It enumerates all reduced words over the 16 possible moves and finds no pure
3-cycle through depth 5; `C` is the first result at depth 6. Consequently six
FTM moves is optimal for a pure 3-cycle in this generator set.

In the fifth-turn metric, `C` costs eight clicks because each double turn costs
two. A second exhaustive pass finds no pure 3-cycle through depth 7, so eight
clicks is optimal too. This local result does not imply that using a shortest
3-cycle minimizes an entire solve.

## Row-by-row procedure

1. Put the three odd-color balls in the cap. This preliminary step is done with
   ordinary disc and core moves.
2. Rotate the lower disc until the bottom row contains one ball of each of the
   five four-ball colors. Their order defines the target column colors.
3. The cycle `(11,13,18)` has two working locations in row 3 and one in row 2,
   while leaving the bottom row and cap untouched. Use disc turns to bring the
   next required ball and its destination to working locations.
4. Apply `C` once or twice to insert that ball, then reverse the setup turns.
   Repeating this operation completes row 2, then rows 3 and 4. Already matched
   balls provide temporary storage as the unsolved region shrinks.

The exact choice of setup turns depends on the scramble; this is a method, not
one fixed move string. Its advantage is that only `C` must be remembered.

## GAP group-theoretic check

GAP also gives

```text
NormalClosure(A23, <C>) = A23.
```

Thus conjugates of this one 3-cycle generate the entire orientation-preserving
move group. This is the algebraic reason a single local cycle is sufficient as
the reusable operation. It does not mean that arbitrary conjugating setup
words are automatically short.

## Historical construction

Takashima's historical one-algorithm idea translates to

```text
(tl Tr bl Br Tl Bl tr br), repeated twice.
```

One pass is `(4,9,18)(7,12)(11,15)`: a 3-cycle and two swaps. Repeating it
cancels the swaps and leaves `(4,18,9)`. This costs 16 fifth-turn clicks, twice
the new word, but it motivates the same row-by-row method. The historical
description appears in the 1981 Cube Lovers archive; its orientation
conventions differ from this project.

The other documented macros cost 14, 14, and 44 fifth-turns and sometimes move
five or nine useful balls at once. Which method produces a shorter full solve
depends on the scramble.

The separate [optimal-number study](optimal-number.md) concerns provably
shortest solutions and currently establishes lower bounds, not a matching
God's number.

## Source

- [Stan Isaacs, “Ten Billion Puzzle (the Barrel),” Cube Lovers archive
  (1981)](https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/Stan_Isaacs__Ten_Billion_Puzzle_%28the_Barrel%29.html)
