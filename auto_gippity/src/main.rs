#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {
        stringify!($func)
    };
}

#[macro_use]

mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::cli::get_user_response;
use models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() {
    let user_req: String = get_user_response("What website are we building today?");

    let manager_agent: ManagingAgent = ManagingAgent::new(user_req).await.expect("Error creating agent");

    manager_agent.execute_project().await;
    dbg!(user_req);
}
