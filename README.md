# Star Tenbillion

A reproducible GAP analysis and practical solutions for Nintendo's
Star Tenbillion, the 2007 remake of Gunpei Yokoi's Ten Billion Barrel.

The main machine-checked results:

- The move group on distinctly labelled balls is the full alternating group
  `A_23`, so every color arrangement is reachable.
- A single mirrored six-move algorithm, `C = Tl bll Tl | br Trr br`, is a pure
  3-cycle, and an exhaustive search proves that no shorter one exists.
- God's number is at least 16 fifth-turn clicks and at least 15 disc turns.

## Run the analysis

Install [pixi](https://pixi.sh), then run:

```sh
pixi install
pixi run analyze          # group structure, solution macros, state counts
pixi run test             # GAP model self-tests
pixi run compact          # verify the compact 3-cycle
pixi run search-compact   # prove its minimality
pixi run test-optimal     # self-test the quotient search
pixi run optimal          # lower bounds for God's number
```

GAP and Rust versions are locked in `pixi.lock`; nothing needs to be
installed globally.

## Documentation

| Document | Contents |
|---|---|
| [Notation](docs/notation.md) | Move notation, the 23 position names, and the solved-state convention |
| [Group structure](docs/group-structure.md) | GAP proof that the move group is `A_23`, and the state counts |
| [Compact solution](docs/compact-solution.md) | One six-move 3-cycle, its proved minimality, and a row-by-row solve built on it |
| [Three-algorithm solution](docs/three-algorithm-solution.md) | The Pedro Luis column-by-column method with three reusable cycles |
| [Optimal number](docs/optimal-number.md) | Metrics, the exact quotient search, and the God's-number lower bounds |

## Sources

- [Star Tenbillion overview](https://nintendo.fandom.com/wiki/Star_Tenbillion)
- [Puzzle mechanics and solution](https://www.jaapsch.net/puzzles/nintendo.htm)
- [Takashima one-algorithm solution](https://www.math.rwth-aachen.de/~Martin.Schoenert/Cube-Lovers/Stan_Isaacs__Ten_Billion_Puzzle_%28the_Barrel%29.html)
- [David Singmaster's group/counting discussion](https://www.jaapsch.net/puzzles/cubic2.htm)
- [Nintendo Tumbler Puzzle overview](https://en.wikipedia.org/wiki/Nintendo_Tumbler_Puzzle)
