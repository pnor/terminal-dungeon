# What is this?
Terminal dungeon

# What does it do differently?
Battle system (use 2x2 matrix and try to get it down to all 0)

Build your own spells to attack enemies by collecting pieces across dungeon

Explore a randomly generated dungeon, gets more difficult the farther you go

## Spell in Details
Spells have 3 parts
- matrix
    - 2x2 matrix
- target
    - relative to user, target in set range based on matrix
- operation
    - add, subtract, multiply, average, reset, **dot product, etc.**

Cast using player spell gauge, where most things are cheap but some things have higher cost based on a function of the
operation, target, and matrix

Ability to add some logic to spells?
Chain them, if/then/else logic

# Mechanics In Depth

# Dungeon
Randomly generated

Enemies have different types, but should essentially be random

Several different appearances:
- have to determine based on the boxes unicode characters
- one with the 3D characters

Ideal: everything is random
- make is a suprise what you get

## Dungeon Generation
Should be a unique experience each time (random layout, types of layout)
Try to have a cohesive theme, that changes gradually as you go deeper

# User Display
**Main**
```
+----++--------------+ +----+
|4   ||1             | |3   |
+----+|              | |----|
      |              | |    |
      |              | |----|
      |              | |    |
      |              | |    |
      +--------------+ +----+
      +---------------------+
      |2                    |
      +---------------------+
```
- 1 Main Screen
- 2 Player Stats
- 3 Nearby Enemies
- 4 Info/How to play

Menus
**Map**
```
aaaa
aaaa
aaaa
```

- Spell Menus

# Tech Goals

- data drive
- purely random everything
- abstract the game mechanics from the front end so the appearance could be in terminal, in game engine, whatever

# End Goal
Get as far as you can in randomly generated dungeons

Have a high score table naturally

