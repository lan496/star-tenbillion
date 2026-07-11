Read("gap/model.g");

families := [T, B, t, b];
searchMoves := Concatenation(List(families, g -> [g, g^2, g^-2, g^-1]));
searchLabels := [
  "Tl", "Tll", "Trr", "Tr",
  "Bl", "Bll", "Brr", "Br",
  "tl", "tll", "trr", "tr",
  "bl", "bll", "brr", "br"
];

CompactSearch3Cycle := function(remaining, permutation, word, lastFamily)
  local i, family, result;

  if remaining = 0 then
    if NrMovedPoints(permutation) = 3 then
      return [word, permutation];
    fi;
    return fail;
  fi;

  for i in [1..Length(searchMoves)] do
    family := QuoInt(i - 1, 4) + 1;
    # Consecutive turns of one disc/core family collapse to one FTM move.
    if family <> lastFamily then
      result := CompactSearch3Cycle(
        remaining - 1,
        permutation * searchMoves[i],
        Concatenation(word, [i]),
        family
      );
      if result <> fail then
        return result;
      fi;
    fi;
  od;
  return fail;
end;

for depth in [1..7] do
  result := CompactSearch3Cycle(depth, (), [], 0);
  if result = fail then
    Print("No 3-cycle at FTM depth ", depth, ".\n");
  else
    Print("3-cycle at FTM depth ", depth, ": ",
      List(result[1], i -> searchLabels[i]), " = ", result[2], "\n");
    break;
  fi;
od;

# Repeat the search in the fifth-turn metric.  A reduced run on one family is
# either one or two clicks in one direction; opposite clicks cancel.
clickMoves := [T, T^-1, B, B^-1, t, t^-1, b, b^-1];
clickLabels := ["Tl", "Tr", "Bl", "Br", "tl", "tr", "bl", "br"];

CompactClickSearch3Cycle := function(
    remaining, permutation, word, lastMove, runLength)
  local i, sameFamily, nextRun, result;

  if remaining = 0 then
    if NrMovedPoints(permutation) = 3 then
      return [word, permutation];
    fi;
    return fail;
  fi;

  for i in [1..Length(clickMoves)] do
    sameFamily := lastMove <> 0
      and QuoInt(i - 1, 2) = QuoInt(lastMove - 1, 2);
    if sameFamily and i <> lastMove then
      # Opposite clicks on the same disc cancel.
      continue;
    fi;
    if sameFamily and runLength = 2 then
      # Three same-direction clicks equal two clicks the other way.
      continue;
    fi;
    if sameFamily then
      nextRun := runLength + 1;
    else
      nextRun := 1;
    fi;
    result := CompactClickSearch3Cycle(
      remaining - 1,
      permutation * clickMoves[i],
      Concatenation(word, [i]),
      i,
      nextRun
    );
    if result <> fail then
      return result;
    fi;
  od;
  return fail;
end;

for depth in [1..8] do
  result := CompactClickSearch3Cycle(depth, (), [], 0, 0);
  if result = fail then
    Print("No 3-cycle at fifth-turn depth ", depth, ".\n");
  else
    Print("3-cycle at fifth-turn depth ", depth, ": ",
      List(result[1], i -> clickLabels[i]), " = ", result[2], "\n");
    break;
  fi;
od;

QUIT_GAP(0);
