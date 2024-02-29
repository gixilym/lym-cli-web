#[macro_use]
extern crate rocket;
use rocket::{build, Build, Rocket};
use rocket_dyn_templates::{context, Template};
use shuttle_rocket::ShuttleRocket;

#[shuttle_runtime::main]
async fn main() -> ShuttleRocket {
    let rocket: Rocket<Build> = build()
        .mount("/", routes![index])
        .attach(Template::fairing());

    Ok(rocket.into())
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {title: "lym"})
}
