pub mod login;

#[macro_use]
extern crate rocket;

use rocket::form::{Form, Strict};
use sqlx::{PgPool, Result};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user")]
fn new_user() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
