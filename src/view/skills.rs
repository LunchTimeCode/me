use maud::{html, Markup};
use rocket::response::content;

use crate::{models::Skill, sources::get_skills};

use super::components;

#[get("/skills")]
pub async fn get() -> content::RawHtml<String> {
    let comp = components::grid_of(get_skills().iter().map(|s| skill_view(s.clone())).collect());

    let raw = html! {
        article {
          header {
            h2 { "Skills" }
          }
        body{
            (comp)
         }
        }
    }
    .into_string();
    content::RawHtml(raw)
}

fn skill_view(skill: Skill) -> Markup {
    html! {
        article {
            header {
                h2 { (skill.get_name()) }
            }
            body {
                p { (skill.get_description())
            }
            footer{
                {(components::progress_out_of_hundred(skill.get_progress()))}
            }
        }
     }
    }
}
