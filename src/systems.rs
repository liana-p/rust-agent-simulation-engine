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

// Union type for all the systems
pub type Systems = TagAgentSystem;
