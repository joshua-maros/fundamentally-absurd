A guidebook to all the fascinating worlds you will see.

# Glossary
**GOL pattern** - Conway's game of life has many different patterns that have been named by various people. These patterns will sometimes be used to describe the behavior of patterns in different universes. For example, the 'block' pattern is a 2x2 square of cells which remains unchanged generation after generation.
**Still life** - A pattern which never changes. An example is the block pattern described above.
**Oscillator** - A pattern which repeats a particular animation over and over again. The **period** of the oscillator is the number of generations it takes to return to its original state. Oscillators always stay in the same position.
**Ship** - A pattern which moves across the universe over time.
**Breeder** - A pattern which produces ships over time.
**Dead universe** - a universe which fills itself with empty cells in a short period of time and remains completely empty from that point.
**Chaotic universe** - a universe which rapidly forms an extremely chaotic pattern with no discernable patterns.
**Semichaotic universe** - still an uninteresting chaotic universe, but the chaos pattern is slightly more orderly than a regular chaotic universe. Analagous to how brown noise is lower frequency than white noise. This distinction is somewhat subjective and should be considered a subcategory of chaotic universes.
**Static universe** - a universe which fills itself with still lifes within a short period of time, producing no oscillators, ships, or breeders.
**Oscillating universe** - a universe which contains oscillators and (optionally) still lives. These universes contain no ships or breeders.
**Metro universe** - a universe with a chaotic mess of ships and breeders which follow axis-oriented paths, causing the whole universe to resemble a cyberpunk city with glowing, flying cars. The term **crash** is used to describe a chaotic pattern inside this world which is not axis aligned, it appears as if the cars are crashing into each other and generating fiery explosions. The term **order** is used to describe how much the gliders move along the same axis. If there are many intersecting and overlaping lines of gliders going in perpendicular directions, the universe has low order. If all the gliders are travelling on the same direction, the universe has perfect order.
**Living universe** - a universe which contains ships. In contrast to a metro universe, living universes are much more sparse and do not contain a dense net of lines.

# Square Sum Map algorithm
1. Sum everything in a 3x3 neighborhood around the cell, including the cell itself.
2. Modulo it by a certain value.
3. Use the remainder to pick a new value from a list.

For example: 2 % 5 1 is an algorithm where everything is mod 2, any cell with the remainder 0 becomes 5, anything with the remainder 1 becomes 1. 2 is the divisor and 5 1 is the list of coefficients.

Because of the modulo, adding the divisor to a coefficient results in an identical universe, despite the coefficients technically being different. As such, all classifications of universes will imply that these multiples exist without explicitly notating them.

Most universes follow general patterns with some exceptions. Patterns will be listed from more general to more specific. Any more specific pattern that shows up later should be implicitly excluded from patterns shown above it. For example, given two classifications "3 % 0 0 A" and "3 % 0 0 1", it is implied that in the first set, A can be anything except 1.

## Some general observations
### Strobing universes
Having anything other than zero as the first coefficient causes the universe to strobe painfully. As such, these universes will not be explored.

## Mod 2 universes.

### Dead universes
- 2 % 0 0

### Chaotic universes
- 2 % 0 1
It seems to reset itself periodically. Somehow the pattern of chaos encodes the starting state of the universe at it appears to periodically reset to its initial configuration.

## Mod 3 universes.

### Chaotic universes
- 3 % 0 A B
Some exhibit some form of memory.

### Dead universes.
- 3 % 0 0 0

## Mod 4 universes.

### Chaotic universes
- 4 % 0 A B C

### Dead universes
- 4 % 0 0,2 0 0,2

### Oscillating universes
- 4 % 0 0 0 1
- 4 % 0 3 0 0
Comprised mostly of blinkers with some basic GOL still lives such as loaf, tub, and beehive.

## Mod 5 universes.

### Chaotic universes
- 5 % 0 A B C D

### Semichaotic universes:
- 5 % 0 0 1 4 0
- 5 % 0 0 4 1 0
- 5 % 0 2 0 0 3
- 5 % 0 3 0 0 2
- 5 % 0 3 0 2 0
- 5 % 0 4 1 0 0

### Dead universes
- 5 % 0 0 0 0 0
For some reason there is only one dead mod5 universe. I thought there would be more.

### Static universes:
- 5 % 0 0 0 0 1
- 5 % 0 0 0 2 0
- 5 % 0 0 3,4 0 0
- 5 % 0 0 3 0 5
- 5 % 0 2,4 0 0 0
These universes contain GOL block still lifes.

### Oscillating universes:
- 5 % 0 0 0 0 3
- 5 % 0 0 0 1 0 
These universes contain GOL blinker oscillators. The first also contains GOL block still lives, while the second is composed 100% of blinkers.

## Mod 6 universes.
### Chaotic universes
- 6 % 0 0-2 B C D E
- 6 % 0 0 3 0 1 3 (briefly resembles a metro universe before being engulfed in chaos.)

### Semichaotic universes
- 6 % 0 0 0 0 5 1
- 6 % 0 0 0 1 5 0
- 6 % 0 0 1 0 3 0
- 6 % 0 0 1 5 0 0
- 6 % 0 0 3 0 1 0
- 6 % 0 0 3 0 5 0
- 6 % 0 0 5 0 3 0
- 6 % 0 2 3 0 0 3
- 6 % 0 3 0 0 2 3

### Dead universes
- 6 % 0 0 0 0 0 1,3
- 6 % 0 0 0 0 3 0,3
- 6 % 0 0 3 0 0 3,4
- 6 % 0 0 3,4 0 3 0,3
- 6 % 0 0,2 0 0,2,4 0 0,2,4
- 6 % 0 3 0 0 0,3 0,3

### Static universes
- 6 % 0 0 0 0 1 0-2
Comprised of only GOL block still lives. Incrementing the last digit makes the universe take longer to settle to this state.
- 6 % 0 0 5 0 0 0
Again, only GOL blocks.

### Oscillator universes
- 6 % 0 0 0 0 5 3
Quickly decays into a handful of oscillators. There are two types, one small type with a small period and one suprisingly large type with quite a long period as well as ain interesting symmetricality to it. Upon closer inspection, it appears there are multiple variations of the smaller oscillator.
- 6 % 0 0 0 1 0 0
Mostly comprised of GOL blinkers.
- 6 % 0 0 0 1 0 1
Displays a number of different kinds of novel 4 phase oscillators.
- 6 % 0 0 0 1 5 1
Mostly filled with GOL blinkers, also contains a novel 2 and a novel 3 phase oscillator.
- 6 % 0 0 0 5 0 0
Mostly GOL blinkers, contains a novel 4 phase oscillator.
- 6 % 0 0 0 5 0 2
Almost completely empty except for a novel 2 phase oscillator.
- 6 % 0 0 0 5 1 0
Primarily composed of novel 3 and novel 5 phase oscillators.
- 6 % 0 0 3 0 0 1
Mostly empty universe with a novel 4 phase oscillator.
- 6 % 0 0 3 0 2 0
Mostly empty universe with a novel 38 (!!!) phase oscillator.
- 6 % 0 0 3 0 2 3
Filled with oscillators. There are 3 species with periods 3, 4, and 17.
- 6 % 0 0 3 0 5 3
Contains two novel oscillators, one 3 phase one 4 phase. Takes a little while to calm down to that state.
- 6 % 0 0 4 0 3 3
Sparse universe with two novel oscillators, one 2 phase the other 7 phase.
- 6 % 0 0 5 1 0 0
Primarily composed of a novel 3-phase oscillator, also conatins a rarer novel 5-phase oscillator and some GOL blocks.
- 6 % 0 2 0 0 0 1
Almost completely empty except for a novel 2-phase oscillator.
- 6 % 0 2 0 0 3 0
Almost completely empty except for a novel 2-phase oscillator.

### Metro universes
- 6 % 0 0 0 0 1 3
More chaotic, lots of crashes.
- 6 % 0 0 0 0 3 1
Does not exhibit crashes. Slowly becomes more and more ordered over time. Eventually converges to being almost perfectly ordered, although perpendicular gliders are occasionally spontaneously generated.
- 6 % 0 0 0 1 0 4
Displays some crashes. Does not appear to become more oredered over time.
- 6 % 0 0 0 2 0 1
Lots of crashing,
- 6 % 0 0 3 0 3 1
Almost no crashing. Starts off sparse but slowly grows over time. The universe has some order to it because of this, it appears that growth occurs parallel to existing lines. There does not appear to be any mechanism to bring the entire universe to order over time, though.
- 6 % 0 2 0 0 0 3, 6 % 0 3 0 0 0 4
No crashing. Quickly gros and becomes completely ordered.
- 6 % 0 2 0 0 3 3, 6 % 0 2 3 0 3 3, 6 % 0 3 0 0 0 2 
Almost no crashing. Eventually orders itself.
- 6 % 0 2 0 5 0 0
Lots of crashing, does not order itself.
- 6 % 0 3 0 0 3 2
Moderate amount of crashing. Trends towards order but never seems to make it to perfect order, there is always a fair amount of crashing still going on. No noticably long perpendicular lines though.

### Living universes
- 6 % 0 0 0 4 0 1
Contains many axis-aigned ships, many different species varying in length and complexity. It is not uncommon for two ships to interact just right such that a new kind of ship is generated. Occasionally a breeder will emerge which produces ships behind it to its left and to its right, symmetrically.
- 6 % 0 0 3 0 3 4
Very sparse. Has a novel 2 phase oscillator and a 4 phase axis-aligned ship. The ship takes all 4 phases to move a single cell.
- 6 % 0 2 3 0 0 0
Very sparse. Has two novel 4 phase diagonal ship which appear to rotate to move, which I find humorous.
- 6 % 0 2 3 0 3 0
Appears identical to 6 % 0 0 3 0 3 4
- 6 % 0 3 0 0 0 1
Very sparse with a novel 4 phase diagonal ship which appears to rotate to move.

### 6 % 0 0 0 0 3 4
Like the previous universe, quickly decays into many diagonal ships. These ships have many more stages however, and move much more slowly.

### 6 % 0 0 0 1 0 2, 6 % 0 0 1 0 3 3, 6 % 0 3 0 0 3 1,4
These are sort of chaotic metro universes, there are definite axis-aligned patterns showing up, but they are too chaotic in nature to form any kind of consistent order.