Read("gap/model.g");

G := StarTenbillionGroup;

Assert(0, NrMovedPoints(G) = 23);
Assert(0, IsTransitive(G, [1..23]));
Assert(0, ForAll(GeneratorsOfGroup(G), g -> SignPerm(g) = 1));
Assert(0, Size(G) = Factorial(23) / 2);
Assert(0, IsAlternatingGroup(G));

Assert(0, Order(top5) = 5);
Assert(0, Order(bottom5) = 5);
Assert(0, Order(nineCycle) = 9);
Assert(0, NrMovedPoints(top5) = 5);
Assert(0, NrMovedPoints(bottom5) = 5);
Assert(0, NrMovedPoints(nineCycle) = 9);

Assert(0, Order(compactStep) = 6);
Assert(0, compactStep = (4,9,18)(7,12)(11,15));
Assert(0, Order(compact3) = 3);
Assert(0, NrMovedPoints(compact3) = 3);
Assert(0, compact3 = (4,18,9));
Assert(0,
  NormalClosure(StarTenbillionGroup, Group(compact3))
    = StarTenbillionGroup);

Assert(0,
  Factorial(23) / (Factorial(3) * Factorial(4)^5) = 541111756185000);
Assert(0,
  Factorial(23) / (Factorial(3) * Factorial(4)^5 * Factorial(5))
    = 4509264634875);

Print("All Star Tenbillion checks passed.\n");
QUIT_GAP(0);
