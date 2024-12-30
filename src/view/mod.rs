use maud::{html, Markup, PreEscaped};
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::content;

mod html;

#[get("/")]
pub fn index(is_htmx: IsHtmx) -> content::RawHtml<String> {
    content::RawHtml(page(is_htmx.is_htmx, html::get()).into_string())
}

const CSS: &str = r#"<link rel="stylesheet" href="_assets/app.css">"#;
const PURE_CSS: &str = r#"<link rel="stylesheet" href="_assets/pure.css">"#;
const PURE_GRID_CSS: &str = r#"<link rel="stylesheet" href="_assets/pure-grid.css">"#;
const HTMX: &str = r#"<script src="/_assets/htmx.min.js"></script>"#;
const ALPINE: &str = r#"<script src="/_assets/alpine.min.js"></script>"#;

pub fn page(is_htmx: bool, markup: Markup) -> Markup {
    if is_htmx {
        return markup;
    }

    html! {
       html {

            head {
                ({scripts()})
                ({title("Silen Locatelli")})
            }

            body class="container" {
                (markup)
        }
       }
    }
}

fn scripts() -> Markup {
    html! {
       (PreEscaped(CSS))
       (PreEscaped(HTMX))
       (PreEscaped(ALPINE))
       (PreEscaped(PURE_CSS))
       (PreEscaped(PURE_GRID_CSS))
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}

pub enum Theme {
    Orange,
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match value.as_str() {
            "orange" => Theme::Orange,
            _ => Theme::Orange,
        }
    }
}

#[derive(Debug)]
pub struct IsHtmx {
    is_htmx: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsHtmx {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.headers()
            .get_one("HX-Request")
            .map_or(Outcome::Success(IsHtmx { is_htmx: false }), |_| {
                Outcome::Success(IsHtmx { is_htmx: true })
            })
    }
}
