mod models;

use std::sync::Mutex;

use crate::models::PoolDTO;
use crate::models::VoteDTO;
use models::{Pool, Pools};
use rocket::form::{Contextual, Form};
use rocket::fs::{relative, FileServer, Options};
use rocket::response::Redirect;
use rocket::{get, launch, post, routes, uri, State};
use rocket_dyn_templates::{context, Template};

#[launch]
fn rocket() -> _ {
    let pools: Pools = Mutex::new(Vec::new());

    rocket::build()
        // add templating system
        .attach(Template::fairing())
        .manage(pools)
        // serve content from disk
        .mount(
            "/public",
            FileServer::new(
                relative!("/public"),
                Options::Missing | Options::NormalizeDirs,
            ),
        )
        // register routes
        .mount("/", routes![root, new, save_new, show_pool, vote])
}
#[post("/pools/<id>/vote", data = "<form>")]
async fn vote(id: usize, form: Form<Contextual<'_, VoteDTO>>, pools: &State<Pools>) -> Redirect {
    let mut pools = pools.lock().unwrap();

    if let Some(pool) = pools.get_mut(id) {
        if let Some(vote_form) = &form.value {
            if vote_form.option < pool.options.len() {
                pool.votes[vote_form.option] += 1;
            }
        }
    }

    Redirect::to(uri!(show_pool(id)))
}

#[get("/new")]
async fn new() -> Template {
    Template::render("new", context! {})
}
#[post("/new", data = "<form>")]
async fn save_new(form: Form<Contextual<'_, PoolDTO>>, pools: &State<Pools>) -> Redirect {
    let mut pools = pools.lock().unwrap();
    match form.value {
        Some(ref pool_dto) => {
            let id = pools.len();
            let new_pool = Pool::new(id, pool_dto.title.clone(), pool_dto.options.clone());
            println!("new pool created: {:?}", new_pool);
            pools.push(new_pool);
            Redirect::to(format!("/pools/{}", id))
        }
        None => {
            println!("Form validation failed");
            Redirect::to(format!("/pools/new"))
        }
    }
}

#[get("/pools/<id>")]
async fn show_pool(id: usize, pools: &State<Pools>) -> Template {
    let pools = pools.lock().unwrap();
    let pool = pools.get(id).unwrap();
    let mut options = Vec::new();
    for (idx, option) in pool.options.iter().enumerate() {
        options.push(context! {
            idx: idx,
            option: option,
            votes: pool.votes[idx]
        });
    }

    Template::render(
        "pool",
        context! { pool_id: pool.id, title: &pool.title, options: options },
    )
}
#[get("/")]
async fn root() -> Template {
    Template::render("root", context! {})
}
