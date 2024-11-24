use maud::{html, Markup, PreEscaped};
use rocket::response::content;

pub mod about_me;
pub mod home;
pub mod exp;
mod icons;
pub mod nav;

#[get("/")]
pub fn index() -> content::RawHtml<String> {
    let raw = page(html! {
        header {
            div hx-get="/refresh" hx-trigger="every 300ms" {}
            // load nav always
            div hx-get="/nav" hx-trigger="load" {}
            // load home first
            div hx-get="/home" hx-trigger="load" hx-target="#body" {}
        }

    body {
        div #body {}
    }
    })
    .into_string();
    content::RawHtml(raw)
}

const PICO: &str = r#"<link rel="stylesheet" href="_assets/pico.min.css">"#;
const CSS: &str = r#"<link rel="stylesheet" href="_assets/app.css">"#;
const HTMX: &str = r#"<script src="/_assets/htmx.min.js"></script>"#;
const REFRESH: &str = r#"<script src="/_assets/refresh.js"></script>"#;

pub fn page(markup: Markup) -> Markup {
    html! {
       html {

            head {
                ({scripts()})
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
       (PreEscaped(PICO))
       (PreEscaped(CSS))
       (PreEscaped(HTMX))
       (PreEscaped(REFRESH))
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}
