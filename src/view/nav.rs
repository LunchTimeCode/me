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
                li { button class="outline primary" hx-get="/home" hx-target="#body" { "Home" } }
                li { button class="outline contrast" hx-get="/about-me" hx-target="#body"  { "About me" } }
                li { button class="outline secondary" hx-get="/exp" hx-target="#body"  { "Experience" } }
                li { button class="outline secondary" hx-get="/skills" hx-target="#body" { "Skills" } }
                li { button class="outline secondary" hx-get="/projects" hx-target="#body"  { "Projects" } }
                li { button class="outline secondary" hx-get="/education" hx-target="#body"  { "Education" } }
                li { button class="outline secondary" hx-get="/contact" hx-target="#body" { "Contact" } }
            }
        }
    }
    .into_string();
    content::RawHtml(raw)
}
