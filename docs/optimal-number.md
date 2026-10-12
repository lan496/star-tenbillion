# Star Tenbillion's optimal number

For a fixed move metric, God's number is the greatest, over all scrambled
states, of the length of a shortest solution. Star Tenbillion has multiple
solved states because the five four-ball colors may occupy the five columns in
any order.

## Metrics

This study uses the standard notation in which changing the core position is
free and implicit.

- **Fifth-turn metric (QTM-like):** one 72-degree click left or right counts as
  one move. The eight moves are `T`, `B`, `t`, `b`, and their inverses.
- **Disc-turn metric (FTM-like):** any nonzero rotation of one disc counts as
  one move. These are the four nonidentity powers of each of the four
  generators, giving 16 moves.

The answer would be different if moving the core counted as a move.

## State count

There are

```text
N = 23! / (3! (4!)^5 5!) = 4,509,264,634,875
```

color-equivalence classes (derived in the
[group-structure note](group-structure.md)). Identifying the five four-ball
colors makes the solved state unique, so every distance below is measured to a
single goal. Any proof that fewer than `N` states lie within distance `r` of the
goal shows that God's number exceeds `r`.

## Counting bound from least words

Every state has a shortest move sequence that is least in shortlex order,
comparing moves by their index. That sequence is also the least word for its
element of the move group, and so is each of its factors, because replacing a
factor by a shorter or lexicographically smaller word for the same group
element would give a smaller sequence for the state. The number of states
within distance `r` is therefore at most the number of words of length at most
`r` whose factors of length `k` are all least words.

[`src/full_ball.rs`](../src/full_ball.rs) lists the least words up to length
`k = 10` (fifth-turn) or `k = 7` (disc-turn) by breadth-first search on the
move group, which is `A_23` acting on labelled balls, and counts the admissible
longer words with a finite automaton whose state is the last `k - 1` moves.
Short relations among the generators are what the count exploits. The pairs
`(T, B)`, `(t, B)` and `(t, b)` act on disjoint positions and commute, and
`T` and `b` share three positions and satisfy equal-length relations of length
six, for example `(T b^-1)^3 = (b^-1 T)^3`.

The bound is sharpened by an exact breadth-first search of the full state space
to a depth `d`. A least shortest path to a state beyond `d` passes through a
state `x` at distance exactly `d`; its next move forms a least two-move word
with some move that ends a shortest path to `x`, and its remaining moves are a
word counted by the automaton. The search records, for each state at distance
`d`, the set of moves that end its shortest paths, so the bound uses the exact
size of the distance-`d` sphere in place of a word count.

The search ranks each state by its black positions and a canonical set
partition of the other twenty positions into blocks of four, so a state costs
one 43-bit integer. Levels are kept as sorted arrays, and the last level is only
counted, never stored. Run:

```sh
pixi run test-full-ball
pixi run full-ball
```

The fifth-turn search to depth 12 takes about three minutes on four cores.

## Exact quotient search

[`src/optimal_number.rs`](../src/optimal_number.rs) performs multi-source
breadth-first search on a projection retaining:

- all three balls of the odd-sized color class; and
- all four balls of one chosen four-ball color.

The number of abstract states is

```text
C(23, 3) C(20, 4) = 8,580,495.
```

There are five abstract goals: the three-ball color occupies the cap and the
tracked four-ball color occupies any one column. Every abstract state can be
completed to a full colored state. Projection cannot increase shortest-path
distance, so the maximum abstract distance to these goals is a rigorous lower
bound for the full puzzle's God's number.

The program uses a perfect combinatorial ranking, one byte of distance per
state, and a multi-source queue. Its self-test exhaustively round-trips all
8,580,495 ranks and checks the move inverses. Run:

```sh
pixi run test-optimal
pixi run optimal
```

## Results

Table 1 collects the lower bounds. In the fifth-turn metric the counting bound
exceeds the quotient radius by three moves; in the disc-turn metric the
quotient radius remains the stronger bound.

**Table 1.** Lower bounds on God's number. "Least words" is the counting bound
without any search; "with exact ball" adds the breadth-first search to the
depth shown; "quotient radius" is the exact radius of the projection described
above. The last column is the best bound.

| Metric | Least words | With exact ball (depth) | Quotient radius | Lower bound |
|---|---:|---:|---:|---:|
| Fifth-turn | 18 | 19 (12) | 16 | **19** |
| Disc-turn | 14 | 14 (8) | 15 | **15** |

### Distances in the full puzzle

Table 2 gives the exact number of full states at each small distance. The
fifth-turn sphere grows by a factor of 4.67 from distance 11 to 12, against
about 4.8 for the least-word count, which is why the exact ball gains one move over
the least-word bound. With the depth-12 ball, at most 4,209,909,874,053 states
lie within 18 fifth-turns, fewer than `N`.

**Table 2.** Number of full color-equivalence classes at each exact distance
from the solved state.

| Distance | Fifth-turn | Disc-turn |
|---:|---:|---:|
| 0 | 1 | 1 |
| 1 | 6 | 12 |
| 2 | 30 | 96 |
| 3 | 144 | 768 |
| 4 | 692 | 6,128 |
| 5 | 3,322 | 48,536 |
| 6 | 15,919 | 380,170 |
| 7 | 76,118 | 2,933,051 |
| 8 | 363,028 | 22,255,491 |
| 9 | 1,726,019 | |
| 10 | 8,174,666 | |
| 11 | 38,512,387 | |
| 12 | 179,681,779 | |
| 13 | 822,494,643 | |

One fifth-turn and one disc-turn give fewer distinct states than moves because
`T B` rotates all four rings together, which fixes the solved state up to
color names; hence `B` acts on the solved state as `T^-1` does. The distance-13
count comes from a separate run of the same search and is not used in Table 1.

### Quotient distance distributions

Both quotient searches reach all 8,580,495 abstract states. Tables 3 and 4
give their distance distributions.

**Table 3.** Fifth-turn distances in the quotient.

| Distance | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| States | 5 | 30 | 146 | 642 | 2,551 | 9,215 | 31,160 | 102,799 | 315,787 |

| Distance | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| States | 830,630 | 1,567,266 | 2,122,897 | 1,962,784 | 1,152,892 | 429,267 | 51,265 | 1,159 |

One distance-16 pattern has the three-ball color at positions `{12,14,20}`
and the tracked four-ball color at `{3,4,6,7}`.

**Table 4.** Disc-turn distances in the quotient.

| Distance | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| States | 5 | 58 | 428 | 2,718 | 14,183 | 62,060 | 223,053 | 649,371 |

| Distance | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| States | 1,303,368 | 1,794,771 | 2,006,085 | 1,553,772 | 785,668 | 176,054 | 8,352 | 549 |

One distance-15 pattern has the three-ball color at positions `{6,21,22}` and
the tracked four-ball color at `{1,2,5,8}`. Position numbers follow the
[project notation](notation.md).

## What remains unknown

No upper bound is established, so God's number is known only to be at least 19
fifth-turns and at least 15 disc-turns. The counting bound also shows that a
large share of states lies beyond 18 fifth-turns: at most 93% of the `N` states
are within 18.

An exhaustive breadth-first search of the full graph needs two bits per state
to separate the frontier from visited states, about 1.1 TB, or 560 GB after the
mirror symmetry. GAP finds that the left-right mirror is the only nontrivial
symmetry of the move set that fixes the solved state; a second symmetry, which
exchanges the top and bottom discs, moves the cap and does not reduce the
search. Subsets defined by projections, such as the quotient above, are not
closed under any large set of moves, so the search cannot be run one subset at
a time.

Closing the gap requires one of:

1. a coset computation in the style of the Rubik's Cube proof, for example on
   the orbits of the subgroup `<T, t, B>`, which fixes the colors of rows 1 and
   2 up to rotation and has 96,023 orbits of at most 162,162,000 states; or
2. stronger pattern databases combined with a complete IDA* search over a
   proved covering set; or
3. a constructive solver with a matching proved worst-case length.
