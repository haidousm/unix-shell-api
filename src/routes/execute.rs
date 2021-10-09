use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CommandRequest<'r> {
    command: &'r str,
}

#[derive(Serialize)]
pub struct CommandResponse<'r> {
    status: u8,
    command_output: &'r str,
}

#[post("/", format = "application/json", data = "<command_request>")]
pub fn execute_cmd(command_request: Json<CommandRequest<'_>>) -> Json<CommandResponse> {
    Json(CommandResponse {
        status: 200,
        command_output: command_request.command,
    })
}
