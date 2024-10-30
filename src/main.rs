use std::env;

use assets::mount_assets;
use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;
mod index;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let port = env::var("ROCKET_PORT").unwrap_or("8000".to_string());

    let path = format!("http://127.0.0.1:{}", port);

    #[allow(clippy::if_same_then_else)]
    if webbrowser::open(&path).is_ok() {
        mount(rocket)
    } else {
        mount(rocket)
    }
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let with_index = rocket.mount("/", routes![index::index]);

    mount_assets(with_index)
}
