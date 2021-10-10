#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};
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

#[get("/")]
fn index() -> &'static str {
    "nothing to see here.."
}

#[post("/", format = "application/json", data = "<command_request>")]
pub fn execute_cmd(command_request: Json<CommandRequest>) -> Json<CommandResponse> {
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

fn main() {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/execute", routes![execute_cmd])
        .attach(cors.to_cors().unwrap())
        .launch();
}
