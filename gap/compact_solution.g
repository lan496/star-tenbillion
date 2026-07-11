Read("gap/model.g");

G := StarTenbillionGroup;
closure := NormalClosure(G, Group(compact3));

Print("Compact Star Tenbillion solution macro\n");
Print("  word:           Tl bll Tl | br Trr br\n");
Print("  mnemonic:       T-b-T left 1-2-1; b-T-b right 1-2-1\n");
Print("  permutation:    ", compact3, "\n");
Print("  moved balls:    ", NrMovedPoints(compact3), "\n");
Print("  order:          ", Order(compact3), "\n");
Print("  tr C tl:        ", columnInsert3, "\n");
Print("  normal closure: ", StructureDescription(closure), "\n");
Print("  closure = A_23: ", closure = G, "\n");
Print("\nHistorical eight-click word, repeated twice\n");
Print("  word:           tl Tr bl Br Tl Bl tr br\n");
Print("  one pass:       ", takashimaStep, "\n");
Print("  repeat twice:   ", takashima3, "\n");

QUIT_GAP(0);
