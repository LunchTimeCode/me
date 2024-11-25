use maud::html;
use rocket::response::content;

#[get("/projects")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {
               main {
                   section {
                       h2 { "Projects" }

                   }
               }

               footer {
                   p {
                       "Report broken links in the over my email adress."
                   }
               }

    }
    .into_string();
    content::RawHtml(raw)
}
