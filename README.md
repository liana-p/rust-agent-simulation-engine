# Rust Agent Simulation

A library for doing agent and systems based simulations in Rust, with an example simulation.

**(Work in progress, not functional yet)**

## Installation

Clone the repo and use Cargo

```bash
git@github.com:nialna/rust-agent-simulation-engine.git
```

The package isn't functional yet so builds will fail

```bash
cargo build
```

## Usage

```bash
cargo run
```

## Architecture

The architecture is somewhat similar to an [ECS](https://en.wikipedia.org/wiki/Entity_component_system) but with only agents (equivalent to being an entity and its only component at the same time) and systems (which run on agents)

A simulation is composed of the following:

- A world in which agents are added
- Agents which have a state containing data
- Systems which are run on agents individually to perform whatever they're supposed to do
- A global state attached to the world (not implemented yet)

Agents have a dynamic list of systems in their data, this allows the user to change which systems a particular agent uses at runtime).

## Agents

Agents are structs with user-provided properties to define their state. They all implement the `AgentState` trait. You can store whatever data you want in their state.

Agents also need to return a `systems` string vector, which is the list of system ids that should be run for this entity (not functional yet).

Example:

```rust
use std::any::Any;

use simulation::*;
#[derive(Clone)]
pub struct TagAgent{
    pub position: Position,
    pub is_it: bool,
    pub last_hitter: String,
    pub systems: Vec<String>,
    pub speed: f32,
}
// TODO: Find a way to not need all the boilerplate... macro?
impl AgentState for TagAgent{
    fn id() -> String {
        return String::from("Tag Agent State");
    }
    fn dyn_id(&self) -> String {
        return TagAgent::id();
    }
    fn systems(&self) -> Vec<String> {
        return self.systems;
    }
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}
```

## Systems

Systems are empty structs which implement the `System` trait. They take care of running the simulation and updating the state of agents.

Systems have a `simulate` function that takes for parameters:

- `id`: The `id` of the current entity the system is running on (useful to find it again in the world)
- `data`: The data type they're processing. This is an `AgentState` struct and must be specified as the `StateData` type in the system definition
- `world`: A mutable reference to the world, to make it possible to effect other agents

Example:

```rust
use cgmath::InnerSpace;
use cgmath::MetricSpace;
use simulation::*;
use crate::agents;
use crate::helpers;
use crate::constants;

#[derive(Default)]
pub struct TagAgentSystem;
impl TagAgentSystem {
    fn move_agent(data: &mut agents::TagAgent, direction: &Position) {
        data.position += direction * data.speed;
    }
}
impl System for TagAgentSystem {
    type StateData = agents::TagAgent;
    fn id() -> String {
        return String::from("Tag System");
    }
    fn dyn_id(&self) -> String {
        return TagAgentSystem::id();
    }
    fn simulate(&self, id: String, data: &mut agents::TagAgent, world: &mut World) {
        if (data.is_it) {
            // Find closest enemy to run to
            let others: Vec<&agents::TagAgent> = world.agents.into_iter()
                .filter(|agent| agent.id != id && agent.id != data.last_hitter)
                .map(|agent| {
                    let mut state:<agents::TagAgent> = simulation::get_state_mut(agent);
                    return state;
                }).collect();
            let closest = helpers::closest(data.position, others);
            // Move towards them
            TagAgentSystem::move_agent(&mut data, &(closest.position - data.position).normalize());
            // If we're close enough, hit them
            if data.position.distance(closest.position) <= constants::HIT_DISTANCE {
                closest.is_it = true;
                closest.last_hitter = id;
                closest.speed = constants::CHASER_SPEED;
                data.is_it = false;
                data.speed = constants::NORMAL_SPEED;
            }
        } else {
            // If we're not it, just run away
            let danger = world.agents.iter().find(|agent| agent.state.as_ref().is_it);
            if let Some(enemy) = danger {
                let enemy_pos: Position = enemy.state.position;
                let flee_direction = (enemy_pos - data.position).normalize();
                TagAgentSystem::move_agent(&mut data, &flee_direction);
            }
        }
        // Make sure no one leaves the screen
        helpers::clamp_position(&mut data.position, 0.0, constants::SCREEN_SIZE);
    }
}
```

## World and running a simulation

To run a simulation, we need to instantiate a World and give it systems and agents

```rust
fn main() {
    let mut world = World::new();
    create_player(&mut world, true);
    for i in 0..10 {
        create_player(&mut world, false);
    }
    world.add_system(systems::TagAgentSystem::default());
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
        last_hitter: String::new(),
    };
    world.add_agent(agent);
}

fn random_coordinate() -> f32 {
    return rand::thread_rng().gen_range(0f32..constants::SCREEN_SIZE);
}
```

## Current Issues

The simulation engine doesn't currently work due to issues with figuring out how to store dynamic user-provided systems and entities properly without making the compiler sad.

## License

[MIT](https://choosealicense.com/licenses/mit/)
