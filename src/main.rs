
use assets::mount_assets;
use rocket::{Build, Rocket};
use tec::mount_tec;

#[macro_use]
extern crate rocket;

mod assets;
mod models;
mod sources;
mod tec;
mod view;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let rocket = rocket.configure(rocket::Config::figment().merge(("port", 12500)));
    let rocket = rocket.configure(rocket::Config::figment().merge(("address", "0.0.0.0")));

    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let with_index = rocket.mount(
        "/",
        routes![
            view::index,
            view::nav::get,
            view::about_me::get,
            view::home::get,
            view::exp::get,
            view::skills::get,
            view::education::get,
            view::contact::get,
            view::projects::get,
        ],
    );

    let with_assets = mount_assets(with_index);

    mount_tec(with_assets)
}
