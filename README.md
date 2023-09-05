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

- [x] create a world
- [x] create a cell
- [ ] create a brain
- [ ] create a DNA
- [ ] create a DNA hash
- [ ] create a DNA mutation concept while living
- [ ] create a DNA merge function
- [ ] create a reproduction

## Environment concepts (world)

- [x] world is a 2D grid
- [x] each grid has attributes
- [x] multiple attributes can be found on the same grid position
- [ ] everything transform, no delition (death doesn't mean just a removal of the object, consumption is a transformation)

## Neural inputs
- own energy
- tile energy
- left tile energy
- right tile energy
- top tile energy
- bottom tile energy


## Technical notes

### Get colors from brain weights config

```toml
[dependencies]
sha2 = "0.9"
hex = "0.4"
palette = "0.6"
```

```rust
extern crate sha2;
extern crate hex;
extern crate palette;

use sha2::{Sha256, Digest};
use palette::{Srgb, Shade};

fn main() {
    let complex_object = vec![1, 2, 3, 4, 5];
    let mut hasher = Sha256::new();
    hasher.update(&complex_object);
    let result = hasher.finalize();
    let hex_string = hex::encode(result);

    // Take first 6 characters to form a color code
    let color_code = &hex_string[0..6];
    println!("Color Code: #{}", color_code);

    // Convert to RGB color using `palette`
    let color: Srgb<u8> = Srgb::new(
        u8::from_str_radix(&color_code[0..2], 16).unwrap(),
        u8::from_str_radix(&color_code[2..4], 16).unwrap(),
        u8::from_str_radix(&color_code[4..6], 16).unwrap(),
    );

    println!("RGB Color: {:?}", color);
}
```
