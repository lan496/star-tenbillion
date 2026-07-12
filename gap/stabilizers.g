Read("gap/model.g");

G := StarTenbillionGroup;

# Position numbers: cap A5,C5,E5 = 1,2,3 ; row 1 A1..E1 = 19..23.
row1 := [19, 20, 21, 22, 23];
cap := [1, 2, 3];
capRow1 := Concatenation(cap, row1);
pinned := [19, 21, 22];

Print("Pointwise stabilizers are alternating groups on the complement\n");
for s in Combinations(row1, 4) do
  Print("  4-subset ", s, " of row 1:  |Stab| = 19!/2: ",
    Size(Stabilizer(G, s, OnTuples)) = Factorial(19) / 2, "\n");
od;
Print("  row 1:               |Stab| = 18!/2: ",
  Size(Stabilizer(G, row1, OnTuples)) = Factorial(18) / 2, "\n");
Print("  A1,C1,D1:            |Stab| = 20!/2: ",
  Size(Stabilizer(G, pinned, OnTuples)) = Factorial(20) / 2, "\n");
Print("  cap and row 1:       |Stab| = 15!/2: ",
  Size(Stabilizer(G, capRow1, OnTuples)) = Factorial(15) / 2, "\n");
Print("  cap and A1,C1,D1:    |Stab| = 17!/2: ",
  Size(Stabilizer(G, Concatenation(cap, pinned), OnTuples))
    = Factorial(17) / 2, "\n\n");

Print("Single clicks inside each stabilizer\n");
gens := [T, B, t, b];
gnames := ["T", "B", "t", "b"];
for r in [rec(n := "row 1 pointwise:  ", s := row1),
          rec(n := "cap pointwise:    ", s := cap),
          rec(n := "cap and row 1:    ", s := capRow1),
          rec(n := "A1,C1,D1:         ", s := pinned)] do
  Print("  ", r.n,
    gnames{Filtered([1..4], i -> OnTuples(r.s, gens[i]) = r.s)}, "\n");
od;
Print("\n");

Print("Set-preserving clicks generate only a fraction of the stabilizer\n");
Print("  <T,t> is the full stabilizer of rows 1-2 (= A13): ",
  Size(Group(T, t)) = Factorial(13) / 2, "\n");
Print("  |<T,b>|   = ", Size(Group(T, b)),
  "  vs |Stab(cap,A1,C1,D1)| = 17!/2 = ", Factorial(17) / 2, "\n");
Print("  |<T,t,b>| = ", Size(Group(T, t, b)),
  "  vs |Stab(A1,C1,D1)|     = 20!/2 = ", Factorial(20) / 2, "\n\n");

Print("Conjugated insertions recover the full stabilizer\n");
setupConjugates := List(Elements(Group(T, B)), s -> columnInsert3 ^ s);;
Print("  25 conjugates S K S^-1 with S in <T,B> generate Stab(cap,row1): ",
  Group(setupConjugates) = Stabilizer(G, capRow1, OnTuples), "\n");

QUIT_GAP(0);
