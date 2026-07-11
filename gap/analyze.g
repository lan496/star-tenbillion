Read("gap/model.g");

G := StarTenbillionGroup;

Print("Star Tenbillion move group\n");
Print("  degree:            ", NrMovedPoints(G), "\n");
Print("  order:             ", Size(G), "\n");
Print("  structure:         ", StructureDescription(G), "\n");
Print("  is A_23:           ", IsAlternatingGroup(G), "\n");
Print("  generator signs:   ", List(GeneratorsOfGroup(G), SignPerm), "\n");
Print("  point orbits:      ", Orbits(G), "\n\n");

Print("Published solution macros\n");
Print("  tr Tl, repeated 7:       ", top5, "\n");
Print("  br Bl, repeated 7:       ", bottom5, "\n");
Print("  tr br Tl Bl, repeated 11: ", nineCycle, "\n\n");

Print("Compact one-algorithm solution\n");
Print("  Tl bll Tl | br Trr br:       ", compact3, "\n");
Print("  face turns / fifth-turns:    6 / 8\n");
Print("  normal closure is A_23:      ",
  NormalClosure(G, Group(compact3)) = G, "\n\n");

allColoredStates := Factorial(23) / (Factorial(3) * Factorial(4)^5);
statesModuloColorNames := allColoredStates / Factorial(5);
Print("Colored positions (fixed color names): ", allColoredStates, "\n");
Print("Modulo the five column-color names:    ", statesModuloColorNames, "\n");

QUIT_GAP(0);
