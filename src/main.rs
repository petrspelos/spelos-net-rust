use rocket::fs::FileServer;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            foo: 123,
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("static/"))
        .attach(Template::fairing())
}
