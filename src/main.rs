use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;
mod view;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let rocket = rocket.configure(rocket::Config::figment().merge(("port", 12500)));
    let rocket = rocket.configure(rocket::Config::figment().merge(("address", "0.0.0.0")));

    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let with_index = rocket.mount("/", routes![view::index,]);

    let (asset_path, asset_routes) = assets::api();

    with_index.mount(asset_path, asset_routes)
}
