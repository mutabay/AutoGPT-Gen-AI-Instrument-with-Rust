use crate::models::general::llm::Message;
use crate::models::agent_basic::basic_traits::BasicTraits;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    working,
    UnitTesting,
    Finished
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub memory: Vec<Message>, 
    pub state: AgentState
}

impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        BasicAgent {
            objective,
            position,
            memory: Vec::new(),
            state: AgentState::Discovery,
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}