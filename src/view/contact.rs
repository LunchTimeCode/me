use maud::html;
use rocket::response::content;

#[get("/contact")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {
        article{
            header {
                section {
                    h2 { "Contact" }
                }
            }
            body{
                        p {
                            "📧 " a href="mailto:silen.locatelli@gmx.ch" { "silen.locatelli@gmx.ch" } br;
                            "📍 " "Oberwil BL, Switzlerland"
                        }
             }
             }
               footer {
                   p {
                       "Only serious inquiries please."
                   }
               }
    }
    .into_string();
    content::RawHtml(raw)
}
