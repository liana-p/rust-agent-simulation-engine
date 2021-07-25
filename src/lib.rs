/// Rust Agent-based Simulation
///
/// A library for doing agent and systems based simulations in Rust, with an example simulation.
///
/// **(Work in progress, not functional yet)**
///
/// Look at `main.rs` for a usage example.
/// You need to create systems and agents, then instantiate a world and populate it with those
///

use uuid::Uuid;
use std::{any::Any, collections::HashMap};
use cgmath::Vector2;

pub type Position = Vector2<f32>;

/// The main trait that all agent state structs need to implement
/// 
/// Ideally should have a specific non-static lifetime, but I couldn't get that to work with trait objects
pub trait AgentState {
    fn id() -> String where Self: Sized;
    fn dyn_id(&self) -> String;
    fn systems(&self) -> Vec<String>;
    fn as_any(&'static self) -> Box<dyn Any>;
}

pub struct Agent {
    pub id: u128,
    pub systems: Vec<String>,
}
impl Agent {
    pub fn new(systems: Vec<String>) -> Agent {
        return Agent {
            id: Uuid::new_v4().to_u128_le(),
            systems: systems,
        };
    }
    pub fn get_state_mut<S: AgentState + 'static>(&mut self, states: &'static mut HashMap<u128, Box<dyn AgentState + 'static>>) -> S {
        return get::<S>(states.get_mut(&self.id).unwrap().as_any());
    }
}

/// The World struct is the main struct that runs the simulation
pub struct World {
    pub agents: Vec<Agent>,
    pub agent_states: HashMap<u128, Box<dyn AgentState>>,
    pub systems: HashMap<String, Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        return World {
            agents: Vec::new(),
            agent_states: HashMap::new(),
            systems: HashMap::new(),
        };
    }
    /// Adds an agent to the world
    pub fn add_agent<A: AgentState + 'static>(&mut self, state: A) -> &Self{
        let agent = Agent::new(Vec::new());
        let id = agent.id;
        self.agents.push(agent);
        self.agent_states.insert(id, Box::new(state));
        self
    }

    /// Adds a system to the available systems
    pub fn add_system<S: System + 'static>(&mut self, system: S) -> &Self {
      self.systems.insert(S::id(), Box::new(system));
      self
    }

    pub fn run_systems(&mut self) -> &Self {
        for agent in self.agents.iter_mut() {
            let systems = agent.systems.clone();
            for system_id in systems {
                let system = self.systems.get(&system_id).unwrap();
                // TODO: Can't figure out how to invoke the system
                system.simulate(agent,  &mut self.agent_states);
            }
        }
        self
    }

    pub fn get_states<A: AgentState>(&self) -> Option<A> {
        // for state in self.agents.iter() {
        // }
        None
    }
}

/// The main trait that all system structs need to implement
pub trait System {
    fn id() -> String where Self: Sized;
    fn dyn_id(&self) -> String;
    /// This function is called for every actor that uses the system and gives user code the opportunity to change the state
    fn simulate<'a>(&self, agent: &'a mut Agent, states: &'a mut HashMap<u128, Box<dyn AgentState>>);
}

/// I really wanted to make this work with a specific lifetimme, but can't find a way to downcast non-static trait objects
pub fn get<T: Any>(value: Box<dyn Any>) -> T {
    let pv = value.downcast().expect("The pointed-to value must be of type T");
    *pv
}

pub fn get_mut<T: Any>(value: &mut Box<dyn Any>) -> &mut T {
    let pv = value.downcast_mut().expect("The pointed-to value must be of type T");
    pv
}

