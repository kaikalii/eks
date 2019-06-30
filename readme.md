[![Latest Version](https://img.shields.io/crates/v/eks.svg)](crates.io/crates/eks)
[![Documentation](https://docs.rs/eks/badge.svg)](docs.rs/eks)

# Description
`eks` is an entity-component-system library with a focus on simplicity and ergonomics.

# Getting Started
To use `eks`, simply add the following to your `Cargo.toml`:

```toml
[dependencies]
eks = "*"
```

# Example
```rust
use eks::*;

// Set up the components
component! {
    Position: isize,
    Speed: isize,
    Special: (),
}

fn main() {
    // Create the world
    let mut world = World::new();

    // Add some entities
    let a = world.insert(entity! {
        Position: 0,
        Speed: -1,
    });
    let b = world.insert(entity! {
        Position: 2,
        Speed: 3,
        Special: (),
    });

    // Move the entities forward one step
    for (position, speed) in map_mut!(Position, Speed in world) {
        *position += *speed;
    }

    // Check that it worked
    assert_eq!(-1, world[a][Position]);
    assert_eq!( 5, world[b][Position]);
    assert_eq!(1, tags!(Special in world).count())
}
```
