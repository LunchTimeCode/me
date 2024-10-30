use maud::{html, Markup, PreEscaped};
use rocket::response::content;

#[get("/")]
pub fn index() -> content::RawHtml<String> {
    let markup = html! {
       html {

            head {
                ({scripts()})
                ({title("", "Welcome to me")})
            }

            body {
                h1 { "Welcome to me" }
            }
        }
    };

    let raw = markup.into_string();
    println!("{raw}");
    content::RawHtml(raw)
}

const PICO: &str = r#"<link rel="stylesheet" href="_assets/pico.min.css">"#;
const CSS: &str = r#"<link rel="stylesheet" href="_assets/app.css">"#;
const HTMX: &str = r#"<script src="/_assets/htmx.min.js"></script>"#;

fn scripts() -> Markup {
    html! {
       (PreEscaped(PICO))
       (PreEscaped(CSS))
       (PreEscaped(HTMX))
    }
}

fn title(typ: impl Into<String>, title: impl Into<String>) -> Markup {
    html! {

    title type=({typ.into()}) { ({title.into()}) }
    }
}
