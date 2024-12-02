use maud::html;
use rocket::response::content;

use crate::view::components::nav_button_with_class;

#[get("/nav")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {
        nav {
            ul {
                li {
            a href="/" { 
                h1 { "Silen Locatelli" }
            }
                     }
            }
            ul {
                li { (nav_button_with_class("Home", "/home", "outline contrast")) }
                li { (nav_button_with_class("About me", "/about-me", "outline contrast")) }
                li { (nav_button_with_class("Experience", "/exp", "outline contrast")) }
                li { (nav_button_with_class("Projects", "/projects", "outline contrast")) }
                li { (nav_button_with_class("Education", "/education", "outline contrast")) }
                li { (super::to_contact_with_class("Contact", Some("outline contrast".to_string()))) }
            }
        }
    }
    .into_string();
    content::RawHtml(raw)
}
