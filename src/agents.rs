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

// Union type for all the agents
pub type Agents = TagAgent;