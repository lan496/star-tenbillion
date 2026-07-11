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

## Elementary counting bounds

There are

```text
23! / (3! (4!)^5 5!) = 4,509,264,634,875
```

color-equivalence classes. A graph of maximum degree `d` has at most

```text
1 + d ((d - 1)^r - 1) / (d - 2)
```

vertices within radius `r`. Substitution gives initial lower bounds of 15
fifth-turns (`d = 8`) and 11 disc-turns (`d = 16`). Relations among moves make
the actual balls grow more slowly than this idealized tree.

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

| Metric | Exact quotient goal radius | Full-puzzle lower bound |
|---|---:|---:|
| Fifth-turn | 16 | **16** |
| Disc-turn | 15 | **15** |

Both searches reach all 8,580,495 abstract states.

### Fifth-turn distance distribution

| Distance | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| States | 5 | 30 | 146 | 642 | 2,551 | 9,215 | 31,160 | 102,799 | 315,787 |

| Distance | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| States | 830,630 | 1,567,266 | 2,122,897 | 1,962,784 | 1,152,892 | 429,267 | 51,265 | 1,159 |

One distance-16 pattern has the three-ball color at positions `{12,14,20}`
and the tracked four-ball color at `{3,4,6,7}`.

### Disc-turn distance distribution

| Distance | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| States | 5 | 58 | 428 | 2,718 | 14,183 | 62,060 | 223,053 | 649,371 |

| Distance | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| States | 1,303,368 | 1,794,771 | 2,006,085 | 1,553,772 | 785,668 | 176,054 | 8,352 | 549 |

One distance-15 pattern has the three-ball color at positions `{6,21,22}` and
the tracked four-ball color at `{1,2,5,8}`. Position numbers are shown in the
[position and move diagram](positions-and-moves.svg).

## What remains unknown

The quotient search proves lower bounds only. It does not show that every full
colored state is solvable in 16 fifth-turns or 15 disc-turns. Thus neither
number is yet established as God's number.

Closing the gap requires one of:

1. an exhaustive full colored-state computation with symmetry reduction;
2. stronger pattern databases combined with a complete IDA* search; or
3. a constructive solver with a matching proved worst-case length.

The full graph has exactly 525,525 times as many color-equivalence classes as
the quotient used here, so direct breadth-first enumeration is not practical.
