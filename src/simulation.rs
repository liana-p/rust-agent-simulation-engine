
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

    pub trait AgentState {}

    pub struct World {
        pub agents: Vec<Agent>,
        pub systems: Vec<dyn System>,
    }

    impl World {
        pub fn add_agent(&mut self, state: Box<dyn AgentState>) {
            self.agents.push(Agent {
                id: Uuid::new_v4().to_string(),
                position: Position {
                    x: 0f32,
                    y: 0f32,
                },
                state: state,
            });
        }

        pub fn tick() {
            
        }
    }
    pub trait System {
        type StateData;
        fn simulate(&self, agent: &mut Agent, state: &mut impl AgentState);
    }
}