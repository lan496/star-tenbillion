# A compact Star Tenbillion solution

The Pedro Luis solution uses three reusable algorithms. A method attributed to
Naoaki Takashima is more compact in the human sense: it is based on one
three-cycle. The historical description appears in the 1981 Cube Lovers
archive; its orientation conventions differ from this project.

## One algorithm

In this project's notation, use

```text
C = (tl Tr bl Br Tl Bl tr br), repeated twice.
```

The inner eight-turn word is `compactStep` in [`gap/model.g`](../gap/model.g).
GAP calculates

```text
compactStep = (4,9,18)(7,12)(11,15),  order 6
compactStep^2 = (4,18,9),              order 3.
```

The first pass contains a 3-cycle and two disjoint swaps. On the second pass,
the swaps cancel while the 3-cycle advances again. The result moves exactly
three balls and fixes the other 20. Applying `C` twice reverses the cycle, so
no second algorithm needs to be memorized.

Run the exact check with:

```sh
pixi run compact
pixi run test
```

## Row-by-row procedure

1. Put the three odd-color balls in the cap. This preliminary step is done with
   ordinary disc and core moves.
2. Rotate the lower disc until the bottom row contains one ball of each of the
   five four-ball colors. Their order defines the target column colors.
3. The cycle `(4,18,9)` has one working location in each of rows 4, 2, and 3,
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

## What “compact” means

One `C` costs 16 fifth-turns: the eight-turn word is performed twice. The
method is compact in memorized algorithms, not proved optimal in move count.
The previously documented macros cost 14, 14, and 44 fifth-turns respectively,
but sometimes move five or nine useful balls at once. Which method produces a
shorter solve depends on the scramble.

The separate [optimal-number study](optimal-number.md) concerns provably
shortest solutions and currently establishes lower bounds, not a matching
God's number.

## Source

- [Stan Isaacs, “Ten Billion Puzzle (the Barrel),” Cube Lovers archive
  (1981)](https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/Stan_Isaacs__Ten_Billion_Puzzle_%28the_Barrel%29.html)
