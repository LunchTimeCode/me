use components::nav_button_with_class;
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

const ORANGE: &str = r#"<link rel="stylesheet" href="_assets/pico.min.css">"#;

#[get("/")]
pub fn index(theme: Option<Theme>) -> content::RawHtml<String> {
    let link = theme
        .map(|t| t.as_link().to_string())
        .unwrap_or_else(|| ORANGE.to_string());

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
const ECHARTS: &str = r#"<script src="/_assets/echarts.js"></script>"#;

pub fn page(markup: Markup, pico_css: Markup) -> Markup {
    html! {
       html {

            head {
                ({scripts()})
                ({pico_css})
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
       (PreEscaped(PICO_EXT))
       (PreEscaped(ECHARTS))
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

impl Theme {
    pub fn as_link(&self) -> &str {
        match self {
            Theme::Orange => ORANGE,
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

fn to_contact(title: &str) -> Markup {
    to_contact_with_class(title, None)
}

fn to_contact_with_class(title: &str, classes: Option<String>) -> Markup {
    let classes = classes.unwrap_or_default();
    html! {
        (nav_button_with_class(title, "/contact", classes.as_str()))
    }
}
