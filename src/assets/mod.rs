use rocket::{response::content, Build, Rocket};

pub const JADE: &str = include_str!("../../assets/pico.jade.min.css");
pub const FUCHSIA: &str = include_str!("../../assets/pico.fuchsia.min.css");

#[get("/pico.min.css/<theme>")]
fn pico_code(theme: &str) -> content::RawCss<&'static str> {
    let pico = match theme {
        "jade" => JADE,
        "fuchsia" => FUCHSIA,
        _ => JADE,
    };
    content::RawCss(pico)
}

#[get("/app.css")]
fn app_css() -> content::RawCss<&'static str> {
    let app = include_str!("../../assets/app.css");
    content::RawCss(app)
}

#[get("/htmx.min.js")]
fn htmx_code() -> content::RawJavaScript<&'static str> {
    let app = include_str!("../../assets/htmx.min.js");
    content::RawJavaScript(app)
}

#[get("/theme_chooser.js")]
fn theme_chooser_code() -> content::RawJavaScript<&'static str> {
    let app = include_str!("../../assets/theme_chooser.js");
    content::RawJavaScript(app)
}

#[get("/alpine.min.js")]
fn alpine_code() -> content::RawJavaScript<&'static str> {
    let app = include_str!("../../assets/alpine.min.js");
    content::RawJavaScript(app)
}

#[get("/pico_ext.css")]
fn pico_ext_css() -> content::RawCss<&'static str> {
    let app = include_str!("../../assets/pico_ext.css");
    content::RawCss(app)
}

pub fn mount_assets(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/_assets",
        routes![
            htmx_code,
            alpine_code,
            app_css,
            pico_code,
            pico_ext_css,
            theme_chooser_code
        ],
    )
}
