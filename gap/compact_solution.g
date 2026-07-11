Read("gap/model.g");

G := StarTenbillionGroup;
closure := NormalClosure(G, Group(compact3));

Print("Compact Star Tenbillion solution macro\n");
Print("  one pass:       tl Tr bl Br Tl Bl tr br\n");
Print("  permutation:    ", compactStep, "\n");
Print("  order:          ", Order(compactStep), "\n");
Print("  repeat twice:   ", compact3, "\n");
Print("  moved balls:    ", NrMovedPoints(compact3), "\n");
Print("  order:          ", Order(compact3), "\n");
Print("  normal closure: ", StructureDescription(closure), "\n");
Print("  closure = A_23: ", closure = G, "\n");

QUIT_GAP(0);
