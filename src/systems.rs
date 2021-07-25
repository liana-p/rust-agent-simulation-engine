use std::collections::HashMap;

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
    fn id() -> String {
        return String::from("Tag System");
    }
    fn dyn_id(&self) -> String {
        return TagAgentSystem::id();
    }
    fn simulate<'a>(&self, agent: &'a mut Agent, states: &'a mut HashMap<u128, Box<dyn AgentState>>) {
        let data = agent.get_state_mut::<agents::TagAgent>(states);
        if data.is_it {
            // Find closest enemy to run to
            let others = states.iter_mut()
                .filter(|&(key, state)| agent.id != *key && agent.id != data.last_hitter)
                .map(|&(key, state)| state).collect();
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
            let danger = states.iter_mut().find(|&(key, state)|
                simulation::get_mut::<agents::TagAgent>(state).is_it);
            if let Some((key, enemy)) = danger {
                let enemy_pos: Position = simulation::get_mut::<agents::TagAgent>(enemy).position;
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
