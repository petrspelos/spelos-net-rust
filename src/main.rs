use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::fs;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize, Debug)]
struct RedirectMap {
    route: String,
    target: String,
}

#[get("/<keyword>")]
fn index(keyword: &str) -> Redirect {
    let contents =
        fs::read_to_string("redirectUrls.json").expect("Failed to read the redirect file");

    let redirects: Vec<RedirectMap> =
        serde_json::from_str(&contents).expect("Failed to parse the redirect JSON");

    let result = redirects
        .into_iter()
        .find(|r| r.route == keyword.to_lowercase());
    match result {
        Some(x) => Redirect::to(x.target),
        None => Redirect::to("/"),
    }
}

#[get("/")]
fn redirect() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, redirect])
        .mount("/public", FileServer::from("static/"))
        .attach(Template::fairing())
}
