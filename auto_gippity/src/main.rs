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

fn main() {
    let user_req: String = get_user_response("What webserver are we building today?");
    dbg!(user_req);

}
