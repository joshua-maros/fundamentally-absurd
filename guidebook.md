A guidebook to all the fascinating worlds you will see.

## Square Sum Map algorithm
1. Sum everything in a 3x3 neighborhood around the cell, including the cell itself.
2. Modulo it by a certain value.
3. Use the remainder to pick a new value from a list.

For example: 2 % 5 1 is an algorithm where everything is mod 2, any cell with the remainder 0 becomes 5, anything with the remainder 1 becomes 1. 2 is the divisor and 5 1 is the list of coefficients.

Because of the modulo, adding the divisor to a coefficient results in an identical universe, despite the coefficients technically being different. As such, all classifications of universes will imply that these multiples exist without explicitly notating them.

Most universes follow general patterns with some exceptions. Patterns will be listed from more general to more specific. Any more specific pattern that shows up later should be implicitly excluded from patterns shown above it. For example, given two classifications "3 % 0 0 A" and "3 % 0 0 1", it is implied that in the first set, A can be anything except 1.

### Glossary
**Dead universe** - a universe which fills itself with empty cells in a short period of time and remains completely empty from that point.
**Chaotic universe** - a universe which rapidly forms an extremely chaotic pattern with no discernable patterns.
**Semichaotic universe** - still an uninteresting chaotic universe, but the chaos pattern is slightly more orderly than a regular chaotic universe. Analagous to how brown noise is lower frequency than white noise.
**Static universe** - a universe which quickly fills itself with patterns that do not change after they are created. The universe remains completely static after a short amount of time.
**Blinker pattern** - an arrangement of 3 cells that starts vertical then rotates 90 degrees every generation.

### Some general observations
#### Strobing universes
Having anything other than zero as the first coefficient causes the universe to strobe painfully. As such, these universes will not be explored.

### Mod 2 universes.
Start off with the simplest universes to see if any patterns emerge.

#### 2 % 0 0
Dead universe.

#### 2 % 0 1
A chaotic universe that seems to reset itself periodically. Somehow the pattern of chaos encodes the starting state of the universe at it appears to periodically reset to its initial configuration.

### Mod 3 universes.

#### 3 % 0 A (1, 2)
Chaotic universes, some exhibit some form of memory.

#### 3 % 0 (1, 2) A
Chaotic universes, some exhibit some form of memory.

#### 3 % 0 0 0
Dead universes.

### Mod 4 universes.

#### 4 % 0 A B C
Chaotic universes.

#### 4 % 0 0,2 0 0,2
Empty universes.

#### 4 % 0 0 0 1, 4 % 0 3 0 0
Mostly static universes with blinkers.

### Mod 5 universes.
Chaotic:
- 5 % 0 0 0 0 2
- 5 % 0 0 0 0 4
- 5 % 0 0 0 1 1 
- 5 % 0 0 0 1 2,3,4
- 5 % 0 0 0 2 1,2,3,4
- 5 % 0 0 0 3,4 A
- 5 % 0 0 1,2,3,4 A B
- 5 % 0 0 3 0 3 
- 5 % 0 1 A B C
Semichaotic:
- 5 % 0 0 1 4 0
- 5 % 0 0 4 1 0
Empty:
- 5 % 0 0 0 0 0
Completely Static: (all with bricks so far.)
- 5 % 0 0 0 0 1
- 5 % 0 0 0 2 0
- 5 % 0 0 3 0 0
- 5 % 0 0 3 0 5
- 5 % 0 0 4 0 0
- 5 % 0 4 0 0 0
Static w/ blinkers:
- 5 % 0 0 0 0 3
- 5 % 0 0 0 1 0 (100% composed of blinkers)

