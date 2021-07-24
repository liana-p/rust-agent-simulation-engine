use simulation::simulation::*;

mod simulation;

struct TagState {

}



struct TagAgent{
    pub position: Position,
    pub is_it: bool,
    pub systems: Vec<String>,
}
impl AgentState for TagAgent{
    fn systems(&self) -> Vec<String> {
        return self.systems;
    }
}

#[derive(Default)]
struct TagAgentSystem {

}

impl System for TagAgentSystem {
    type StateData = StateStorage<TagAgent>;
    fn simulate(&self, data: &mut Self::StateData) {
        for agent in &mut data.agents {
            agent.position.x += 1f32;   
        }
    }
}

type Systems = TagAgentSystem;
type Agents = TagAgent;

fn main() {
    println!("Hello, world!");
    let world: World<Systems, Agents> = World {
        agents: Vec::new(),
        systems: Vec::new(),
    };
    let agent = TagAgent {
        is_it: true,
        position: Position {
            x: 0f32,
            y: 0f32,
        },
    };
    world.add_agent(agent)
    .add_system(TagAgentSystem::default(), String::from("Tag Agent System"));

}
