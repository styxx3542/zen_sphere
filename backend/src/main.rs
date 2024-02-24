pub mod signup;

#[macro_use]
extern crate rocket;

use rocket::{form::Form, State };
use sqlx::PgPool;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/signup", data = "<form>")]
async fn new(form: Form<signup::SignupForm>, pool: &State<PgPool>){
    signup::signup(&form.username, &form.password, &form.email, pool).await;

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
