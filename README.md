# AI cell evolution simulation

## Technical

- Using nannou engine for 2D graphics and audio.
- Using rust for the simulation engine.
- Using a custom made genetic algorithm for the evolution of the cells.
- Using a custom made neural network for the brain of the cells.
- Using a custom made DNA for the genetic code of the cells.
- Using a custom made world for the environment of the cells.
- Using a simple matrix based collision system for the cells.
- Using a custom save system for the simulation.
- Using nannou for the UI.

## How to run

Refer to nannou's installation for any OS specific dependencies.

```bash
cargo run
```

Create a build cadidate

```bash
cargo run --release
```

## Concepts

- DNA defines brain's weights and connexions
- DNA hash
- genetic mutation while living (reproduce the concept of aging AKA flaky DNA)
- Cells have organ system which gives bonus or malus to their possibilities, those system are DNA defined so that mutation can cause desease or super power
- world chages
- reproduction (with DNA merge)

## TODO

- [ ] create a world
- [ ] create a cell
- [ ] create a brain
- [ ] create a DNA
- [ ] create a DNA hash
- [ ] create a DNA mutation concept while living
- [ ] create a DNA merge function
- [ ] create a reproduction

## Environment concepts (world)

- [ ] world is a 2D grid
- [ ] each grid has attributes
- [ ] multiple attributes can be found on the same grid position
    - Different types of energy (stable, radioactive)
    - Different types of food (plants, animals)
    - Different types of cells (animals, bacterias)
- [ ] everything transform, no delition (death doesn't mean just a removal of the object, consumption is a transformation)

