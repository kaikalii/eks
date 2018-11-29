//! This simple example involves a world of particles that have both position and velocity

use eks::*;

component! {
    Pos: f64,
    Vel: f64,
    Sprite: char
}

/// Function for initializing a particle
fn particle(pos: f64, vel: f64, sprite: char) -> Entity<Component> {
    entity! { Pos: pos, Vel: vel, Sprite: sprite }
}

/// Create some empty space
fn space() -> Vec<char> {
    (0..60).map(|_| ' ').collect()
}

fn main() {
    // Create the world and add some particle entities
    let mut world = World::new();
    world.push(particle(1.0, 3.0, '#'));
    world.push(particle(8.0, 4.0, '@'));
    world.push(particle(30.0, -2.0, '%'));
    world.push(particle(50.0, -8.0, '$'));

    // Loop 20 times
    for _ in 0..20 {
        // Ititialize some empty space for this iteration
        let mut space = space();
        // Put #'s where the particles are
        for (pos, sprite) in world
            .iter()
            .filter_map(map!(Pos::as_ref(), Sprite::as_ref()))
        {
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
        for (pos, vel) in world
            .iter_mut()
            .filter_map(map_mut!(Pos::as_mut(), Vel::as_ref()))
        {
            *pos += *vel;
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
