use simulation::simulation::*;

mod simulation;

struct TagState {

}



struct TagAgent{
    pub is_it: bool,
}
impl AgentState for TagAgent{}
struct TagAgentSystem {

}

impl System for TagAgentSystem {

    fn simulate(agent: &mut Agent, state: &mut TagAgent) {
        agent.position.x += 1f32;
        state.is_it = true;
    }
}
fn main() {
    println!("Hello, world!");
    let mut world = World {
        agents: vec![],
        systems: vec![],
    };
    world.add_agent(Box::new(TagAgent {
        is_it: true,
    }));
}
