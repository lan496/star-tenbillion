# Group structure and state counts

Position numbers follow the [project notation](notation.md).

## The move group is A_23

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

## Counting color arrangements

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

if the names of the five four-ball colors -- and hence their solved columns --
are regarded as interchangeable. The latter is about 451 times the advertised
ten billion.

Run `pixi run analyze` to reproduce the group computation and both counts.
