#[macro_use]
extern crate rocket;
mod routes;

use routes::execute::execute_cmd;

#[get("/")]
fn index() -> &'static str {
    "nothing to see here.."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/execute", routes![execute_cmd])
}
