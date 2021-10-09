use rocket::serde::{json::Json, Deserialize, Serialize};
use std::process::Command;

#[derive(Deserialize)]
pub struct CommandRequest<'r> {
    command: &'r str,
}

#[derive(Serialize)]
pub struct CommandResponse {
    status: u8,
    command_output: String,
}

#[post("/", format = "application/json", data = "<command_request>")]
pub fn execute_cmd(command_request: Json<CommandRequest<'_>>) -> Json<CommandResponse> {
    let command_output = Command::new("../c/shell")
        .current_dir("resources/demo-directory")
        .arg(command_request.command)
        .output()
        .expect("ugh c is sometimes just a b");
    Json(CommandResponse {
        status: 200,
        command_output: String::from_utf8(command_output.stdout)
            .expect("ugh utf8 is sometimes just a b"),
    })
}
