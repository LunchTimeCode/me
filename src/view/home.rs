use maud::{html, PreEscaped};
use rocket::response::content;

use crate::view::icons;

#[get("/home")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {

    article class="hero" {

        header {
            h2 { "Senior Software Developer" }
            p { em { "Building elegant solutions to complex problems" } }
    div {
        img src="https://i.ibb.co/5BQY0v8/1959be0f-b7f2-4a2d-b719-f5243ad84152.jpg"
        alt="1959be0f-b7f2-4a2d-b719-f5243ad84152"
        border="0"
        width="400"
        height="400"
        {}
    }
        }

        body class="contact-info"{
            p {
                "📧 " a href="mailto:silen.locatelli@gmx.ch" { "silen.locatelli@gmx.ch" } br;
                "📍 " "Oberwil BL, Switzlerland"
            }
        }



        footer {
            // GitHub
            a class="social-icon" href="https://github.com/silenloc" {
                 "GitHub"
            }

            // LinkedIn
            a class="social-icon" href="https://www.linkedin.com/in/silen-locatelli/"
             {
                  (PreEscaped(icons::LINKEDIN))
            }
        }                       }    }
    .into_string();
    content::RawHtml(raw)
}
