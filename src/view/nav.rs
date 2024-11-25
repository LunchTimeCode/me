use maud::html;
use rocket::response::content;

#[get("/nav")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {
        nav {
            ul {
                li { h1 { "Silen Locatelli" } }
            }
            ul {
                li { button class="primary" hx-get="/home" hx-push-url="true" hx-target="#body" { "Home" } }
                li { button class="primary" hx-get="/about-me" hx-target="#body" hx-push-url="true" { "About me" } }
                li { button class="outline" hx-get="/exp" hx-target="#body" hx-push-url="true" { "Experience" } }
                li { button class="outline" hx-get="/skills" hx-target="#body" hx-push-url="true" { "Skills" } }
                li { button class="outline" hx-get="/projects" hx-target="#body" hx-push-url="true" { "Projects" } }
                li { button class="outline" hx-get="/education" hx-target="#body" hx-push-url="true" { "Education" } }
                li { button class="outline" hx-get="/contact" hx-target="#body" hx-push-url="true" { "Contact" } }
            }
        }
    }
    .into_string();
    content::RawHtml(raw)
}
