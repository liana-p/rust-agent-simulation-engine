use rand::Rng;
use std::time::Duration;
use futures_timer::Delay;

use simulation::*;
mod systems;
mod agents;
mod helpers;
mod constants;



fn main() {
    let mut world = World::new();
    create_player(&mut world, true);
    for i in 0..10 {
        create_player(&mut world, false);
    }
    world.add_system(systems::TagAgentSystem::default());
    start_simulation(world);
}

async fn start_simulation(mut world: World) {
    loop {
        simulation_tick(&mut world).await;
    }
}

async fn simulation_tick(world: &mut World) {
    println!("Simulation tick");
    world.run_systems();
    Delay::new(Duration::from_millis(constants::TICK_TIME)).await;
}

fn create_player(world: &mut World, is_it: bool) {
    let agent = agents::TagAgent {
        is_it: is_it,
        position: Position {
            x: random_coordinate(),
            y: random_coordinate(),
        },
        systems: vec![systems::TagAgentSystem::id()],
        speed: constants::CHASER_SPEED,
        last_hitter: 0,
    };
    world.add_agent(agent);
}

fn random_coordinate() -> f32 {
    return rand::thread_rng().gen_range(0f32..constants::SCREEN_SIZE);
}