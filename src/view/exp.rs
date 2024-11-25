use maud::html;
use rocket::response::content;

#[get("/exp")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {
               main {
                   section {
                       h2 { "Work Experience" }
                       ul  {
                           li {
                               strong { "Optravis:" }
                               "Software Engineer"
                           }
                       }
                   }
               }

               footer {
                   p {
                       "© 2023 Silen Locatelli. All rights reserved."
                   }
               }

    }
    .into_string();
    content::RawHtml(raw)
}
