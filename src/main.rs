mod models;

use std::sync::{Arc, Mutex};

use models::{Pool, Pools};
use rocket::{get, launch, post, routes, uri, State};
use rocket::form::{Contextual, Form};
use rocket::fs::{FileServer, Options, relative};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{context, Template};
use crate::models::PoolDTO;

#[launch]
fn rocket() -> _ {
    let pools: Pools = Arc::new(Mutex::new(Vec::new()));

    rocket::build()
        // add templating system
        .attach(Template::fairing())
        .manage(pools)
        // serve content from disk
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .mount("/", routes![root, new, save_new])
}

#[get("/new")]
async fn new() -> Template {
    Template::render("new", context! {})
}
#[post("/new", data = "<form>")]
async fn save_new(form: Form<Contextual<'_, PoolDTO>>, pools: &State<Pools>) -> Template {
    let mut pools = pools.lock().unwrap();
    match form.value {
        Some(ref pool_dto) => {
            let id = pools.len();
            let new_pool = Pool::new(id, pool_dto.title.clone(), pool_dto.options.clone());
            println!("new pool created: {:?}", new_pool);
            pools.push(new_pool);
            Template::render("new", context! { message: "Form submitted successfully!" })
        },
        None => {
            println!("Form validation failed");
            Template::render("new", context! { message: "Form submission failed. Please try again." })
        }
    }
}

#[get("/")]
async fn root() -> Template {
    Template::render("root", context! {})
}