#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{context, Template};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket: rocket::Rocket<rocket::Build> = rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing());

    Ok(rocket.into())
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {title: "lym"})
}
