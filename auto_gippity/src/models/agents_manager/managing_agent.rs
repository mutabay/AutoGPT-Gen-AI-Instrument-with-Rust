use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use crate::ai_functions::ai_func_managing::convert_user_input_to_goal;
use crate::helpers::general::{ai_task_request, ai_task_request_decoded};
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::general::llm::Message;

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position: String = "Project Manager".to_string();

        let attributes: BasicAgent = BasicAgent {
            objective: "Manage Agents who are building an excelent website for the user"
                .to_string(),
            position: position.clone(),
            memory: vec![],
            state: AgentState::Discovery,
        };

        let project_description: String = ai_task_request(
            usr_req,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal,
        )
        .await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];
        let factsheet: FactSheet = FactSheet {
            project_description: project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self {
            attributes,
            factsheet,
            agents,
        })
    }

    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        // ! TODO ADD BACKEND AGENT
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_res: Result<(), Box<dyn std::error::Error>> =
                agent.execute(&mut self.factsheet).await;

                let agent_info = agent.get_attributes_from_agent();
                dbg!(agent_info);
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::models::agents_manager::managing_agent;

    use super::*;

    #[tokio::test]
    async fn test_managing_agent() {
        let usr_request: &str = 
        "Need a full stack app that fetches and tracks my fitness progress. Needs to include timezone info from the web.";
        let mut managing_agent: ManagingAgent = ManagingAgent::new(usr_request.to_string()).await.expect("Error creating Managing Agent");

        managing_agent.execute_project().await;

        dbg!(managing_agent.factsheet);
    }
}