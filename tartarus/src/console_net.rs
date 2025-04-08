use crate::auth::Auth;
use crate::console_lib;
use crate::SharedState;
use rocket::{serde::json::Json, Route};
use talaria::console::*;

#[post("/monolith", data = "<command_context>")]
pub async fn monolith(
    _auth: Auth,
    state: &rocket::State<SharedState>,
    command_context: Json<CommandContext>,
) -> Json<Result<ConsoleResponse, ConsoleError>> {
    Json(console_lib::evaluate_command(state.inner().clone(), command_context.0).await)
}

pub fn routes() -> Vec<Route> {
    rocket::routes![monolith]
}
