use rocket::{response::content, Build, Rocket};

#[get("/app.css")]
fn app_css() -> content::RawCss<&'static str> {
    let app = include_str!("../../assets/app.css");
    content::RawCss(app)
}

#[get("/pure.css")]
fn pure_css() -> content::RawCss<&'static str> {
    let app = include_str!("../../assets/pure.css");
    content::RawCss(app)
}

#[get("/project.svg")]
fn project_svg() -> content::RawHtml<&'static str> {
    let app = include_str!("../../assets/project.svg");
    content::RawHtml(app)
}

#[get("/pure-grid.css")]
fn pure_grid_css() -> content::RawCss<&'static str> {
    let app = include_str!("../../assets/pure-grid.css");
    content::RawCss(app)
}

#[get("/htmx.min.js")]
fn htmx_code() -> content::RawJavaScript<&'static str> {
    let app = include_str!("../../assets/htmx.min.js");
    content::RawJavaScript(app)
}

#[get("/alpine.min.js")]
fn alpine_code() -> content::RawJavaScript<&'static str> {
    let app = include_str!("../../assets/alpine.min.js");
    content::RawJavaScript(app)
}

pub fn mount_assets(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/_assets",
        routes![
            htmx_code,
            alpine_code,
            app_css,
            pure_css,
            pure_grid_css,
            project_svg
        ],
    )
}
