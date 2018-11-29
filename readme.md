# Description
`eks` is an entity-component system Rust crate with a focus on simplicity and ergonomics.

# Usage
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
    world.push(entity! {
        Position: 0,
        Speed: -1,
    });
    world.push(entity! {
        Position: 2,
        Speed: 3,
        Special: (),
    });

    // Move the entities forward one step
    for (position, speed) in map_mut!(Position, Speed in world) {
        *position += *speed;
    }

    // Check that it worked
    let mut position_iter = map!(Position in world);
    assert_eq!(Some(&-1), position_iter.next());
    assert_eq!(Some(& 5), position_iter.next());
    assert_eq!(1, tags!(Special in world).count())
}
```
