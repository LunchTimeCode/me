use maud::html;
use rocket::response::content;

#[get("/education")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {
        article{
            header {
                section {
                    h2 { "Education" }
                }
            }
            body{
                        p {
                            "FHNW, Basel, Switzerland"
                        }
             }
             }


               footer {
                   p {
                       "May not be complete."
                   }
               }

    }
    .into_string();
    content::RawHtml(raw)
}
