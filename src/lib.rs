
use uuid::Uuid;
use std::{any::Any, collections::HashMap};
use cgmath::Vector2;

pub type Position = Vector2<f32>;

pub trait AgentState {
    fn id() -> String where Self: Sized;
    fn dyn_id(&self) -> String;
    fn systems(&self) -> Vec<String>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct Agent {
    pub id: String,
    pub state: Box<dyn AgentState>,
}

pub struct SystemStorage<S: System> {
    pub system: S,
    pub id: String,
}

pub struct World {
    pub agents: Vec<Box<Agent>>,
    pub systems: HashMap<String, Box<dyn StoredSystem<dyn AgentState>>>,
}

impl World {
    pub fn new() -> Self {
        return World {
            agents: Vec::new(),
            systems: HashMap::new(),
        };
    }
    pub fn add_agent<A: AgentState + 'static>(&mut self, state: A) -> &Self{
        self.agents.push(Box::new(Agent {
            id: Uuid::new_v4().to_string(),
            state: Box::new(state)
        }));
        return self;
    }

    pub fn add_system<S: System, A: AgentState + 'static>(&mut self, system: S) -> &Self {
        // TODO: Can't figure out how to insert the system because of object trait size issues
      let stored_system = Box::new(StoredSystemStruct {
        system: system,
      }) as Box<dyn StoredSystem<dyn AgentState>>;
      self.systems.insert(S::id(), stored_system);
      return self;
    }

    pub fn run_systems(&mut self) -> &Self {
        for agent in self.agents.iter() {
            let state = agent.state.as_ref();
            for system_id in state.systems() {
                let system = self.systems.get(&system_id).unwrap();
                // TODO: Can't figure out how to invoke the system
                system.simulate(agent.id, state, self);
            }
        }
        return self;
    }

    pub fn get_states<A: AgentState + 'static>(&self) -> Option<A> {
        for state in self.agents.iter() {
        }
        None
    }
}

pub trait System {
    type StateData: AgentState;
    fn id() -> String where Self: Sized;
    fn dyn_id(&self) -> String;
    fn simulate(&self, id: String, state: &mut Self::StateData, world: &mut World);
}
pub trait StoredSystem<A: AgentState> {
  fn run(&self, id: String, state: &mut A, world: &mut World);
  fn simulate(&self, id: String, state: &mut A, world: &mut World);
}

pub struct StoredSystemStruct<S: System> {
  system: S
}
impl<A, S> StoredSystem<A> for StoredSystemStruct<S>
where A: AgentState,
S: System {
  fn run(&self, id: String, state: &mut A, world: &mut World) {
    self.simulate(id, state, world);
  }
  fn simulate(&self, _id: String, _state: &mut A, _world: &mut World) {}
}

pub fn get_state_mut<'a, T: AgentState>(agent: Box<Agent>) -> &'a mut T {
    return agent.state.as_any().downcast_mut::<T>().unwrap();
}
