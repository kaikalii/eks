//! This simple example involves a world of particles that have both position
//! and velocity

use eks::{component, entity, map, map_mut, Entity, World};

// Set up components
component! {
    Particle {
        Pos: f64,
        Vel: f64,
        Sprite: char,
    }
}

/// Function for initializing a particle
fn particle(pos: f64, vel: f64, sprite: char) -> Entity<Particle> {
    entity! { Pos: pos, Vel: vel, Sprite: sprite }
}

fn main() {
    // Create the world and add some particle entities
    let mut world = World::new();
    world.insert(particle(1.0, 3.0, '#'));
    world.insert(particle(8.0, 4.0, '@'));
    world.insert(particle(30.0, -2.0, '%'));
    world.insert(particle(50.0, -8.0, '$'));
    world.insert(particle(-30.0, 4.0, '&'));

    // Loop 20 times
    for _ in 0..20 {
        // Ititialize some empty space for this iteration
        let mut space: Vec<char> = (0..60).map(|_| ' ').collect();

        // Put sprites where the particles are
        for (pos, sprite) in map!(Pos, Sprite in world) {
            if *pos >= 0.0 {
                let upos = *pos as usize;
                if upos < space.len() {
                    space[upos] = *sprite;
                }
            }
        }

        // Draw the space
        for c in space {
            print!("{}", c);
        }
        println!();

        // Update the particle positions
        for (pos, vel) in map_mut!(Pos, Vel in world) {
            *pos += *vel;
        }

        // Wait for a quarter second
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}
