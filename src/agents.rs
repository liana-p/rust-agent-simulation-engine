use std::any::Any;

use simulation::*;
#[derive(Clone)]
pub struct TagAgent {
    pub position: Position,
    pub is_it: bool,
    pub last_hitter: u128,
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
    fn as_any(&'static self) -> Box<dyn Any> {
        let res = Box::new(self) as Box<dyn Any>;
        res   
    }
}

// Union type for all the agents
pub type Agents = TagAgent;