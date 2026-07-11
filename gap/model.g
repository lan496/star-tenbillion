# Star Tenbillion / Ten Billion Barrel permutation model.
#
# The normal core position has 23 ball positions.  Positions 1..3 are the
# three cap balls; 4..23 are the four five-ball rings, from top to bottom.
# Each disc turn rotates two adjacent rings.  Lower-case generators are the
# same physical turns after moving the core to its other position and back.
# All four generators are written as left turns; right turns are inverses.

T := (4,5,6,7,8)(9,10,11,12,13);
B := (14,15,16,17,18)(19,20,21,22,23);
t := (1,5,2,3,8)(4,10,6,7,13);
b := (9,15,11,12,18)(14,20,16,17,23);

StarTenbillionGroup := Group(T, B, t, b);

# Translate Jaap Scherphuis's notation literally: a word is performed from
# left to right, and GAP's permutation product has the same convention for
# points acting on the right.
top5step := t^-1 * T;                # tr Tl
bottom5step := b^-1 * B;             # br Bl
nineStep := t^-1 * b^-1 * T * B;     # tr br Tl Bl

top5 := top5step^7;
bottom5 := bottom5step^7;
nineCycle := nineStep^11;

# Takashima-inspired one-algorithm solution.  One pass has a 3-cycle and two
# transpositions; squaring it cancels the transpositions.
takashimaStep := t * T^-1 * b * B^-1 * T * B * t^-1 * b^-1;
takashima3 := takashimaStep^2;

# A shorter pure 3-cycle found by exhaustive FTM search through depth 6:
#   Tl bll Tl | br Trr br
# The generator families alternate T,b,T | b,T,b; the turn sizes are the
# palindrome 1,2,1 in each half, with left turns followed by right turns.
compact3 := T * b^2 * T * b^-1 * T^-2 * b^-1;
