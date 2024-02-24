#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{context, Template};


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .attach(Template::fairing())

}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {title: "lym"})
}

/* abrir apps o archivos lym open [app] */

