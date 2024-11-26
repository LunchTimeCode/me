use maud::{html, Markup, PreEscaped};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::response::content;

pub mod about_me;
mod components;
pub mod contact;
pub mod education;
pub mod exp;
pub mod home;
mod icons;
pub mod nav;
pub mod projects;
pub mod skills;

const JADE: &str = r#"<link rel="stylesheet" href="_assets/pico.min.css/jade">"#;
const FUCHSIA: &str = r#"<link rel="stylesheet" href="_assets/pico.min.css/fuchsia">"#;

#[get("/")]
pub fn index(theme: Option<Theme>) -> content::RawHtml<String> {
    let link = theme
        .map(|t| t.as_link().to_string())
        .unwrap_or_else(|| JADE.to_string());

    let raw = page(
        html! {
            header {
                // load nav always
                div hx-get="/nav" hx-trigger="load" {}
                // load home first
                div hx-get="/home" hx-trigger="load" hx-target="#body" {}
            }

        body {
            div #body {}
        }
        },
        PreEscaped(link),
    )
    .into_string();
    content::RawHtml(raw)
}

const CSS: &str = r#"<link rel="stylesheet" href="_assets/app.css">"#;
const PICO_EXT: &str = r#"<link rel="stylesheet" href="_assets/pico_ext.css">"#;
const HTMX: &str = r#"<script src="/_assets/htmx.min.js"></script>"#;
const ALPINE: &str = r#"<script src="/_assets/alpine.min.js"></script>"#;
const REFRESH: &str = r#"<script src="/_assets/refresh.js"></script>"#;

pub fn page(markup: Markup, pico_css: Markup) -> Markup {
    html! {
       html {

            head {
                ({scripts()})
                ({pico_css})
                ({title("Welcome to me")})
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
       (PreEscaped(PICO_EXT))
       (PreEscaped(REFRESH))
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}

pub enum Theme {
    Jade,
    Fuchsia,
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match value.as_str() {
            "jade" => Theme::Jade,
            "fuchsia" => Theme::Fuchsia,
            _ => Theme::Jade,
        }
    }
}

impl Theme {
    pub fn as_link(&self) -> &str {
        match self {
            Theme::Jade => JADE,
            Theme::Fuchsia => FUCHSIA,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Theme {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("X-Theme") {
            Some(key) => request::Outcome::Success(key.to_string().into()),
            None => request::Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
