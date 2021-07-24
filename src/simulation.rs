
pub mod simulation {
    use uuid::Uuid;

    pub struct Position {
        pub x: f32,
        pub y: f32,
    }

    pub struct Agent {
        pub position: Position,
        pub id: String,
        pub state: Box<dyn AgentState>,
    }

    pub trait AgentState {
        fn systems(&self) -> Vec<String>;
    }

    pub struct StateStorage<T> {
        pub agents: Vec<T>,
    }
    pub struct World<S: System, A: AgentState> {
        pub agents: Vec<Box<A>>,
        pub systems: Vec<Box<SystemStorage<S>>>,
    }

    pub struct SystemStorage<S: System> {
        pub system: S,
        pub name: String,
    }

    impl<S, A> World<S, A>
    where 
        S: System,
        A: AgentState,
    {
        pub fn add_agent(mut self, state: A) -> Self{
            self.agents.push(Box::new(state));
            return self;
        }

        pub fn add_system(mut self, system: S, name: String) -> Self {
            let storage = Box::new(SystemStorage {
                system,
                name,
            });
            self.systems.push(storage);
            return self;
        }

        pub fn run_systems(mut self) -> Self {
            for system in &self.systems {
                // TODO: Figure out how to query agents to create a SystemStorage based on system data type
            }
            return self;
        }
    }

    pub trait System {
        type StateData;
        fn simulate(&self, state: &mut Self::StateData);
    }
}