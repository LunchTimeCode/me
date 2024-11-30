use maud::{html, Markup};
use rocket::response::content;

use crate::{
    models::{AppType, Loc, Project, Skill},
    sources::get_skills,
    view::components::list_of,
};

use super::components;

#[get("/skills")]
pub async fn get() -> content::RawHtml<String> {
    let comp = components::list_of(get_skills().iter().map(|s| skill_view(s.clone())).collect());

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
                {(projects(skill.get_projects()))}
            }
        }
     }
    }
}

fn projects(projects: Vec<Project>) -> Markup {
    let mut app_types: Vec<AppType> = projects.iter().map(|p| p.get_app_type()).collect();
    app_types.dedup_by(|a, b| a == b);
    let comp: Vec<Markup> = projects.iter().map(|p| project_view(p.clone())).collect();

    html! {

                details {
                    summary {
                       h6 {"Projects" }
                    }
                    {(list_of(comp))}
                }


    }
}

fn project_view(project: Project) -> Markup {
    let url = project.get_url();
    let loc = project.get_loc();
    html! {
        article {
            header {
                hgroup {
                     h4 { (project.get_name()) }
                     p {(project.get_small_description()) }
                }
            }
            body {
                p { (project.get_description()) }
            @if let Some(u) = url { a href=(u){ (u) } }
            }
            footer {
               {(app_type_view(project.get_app_type()))}
               {(loc_view(loc))}
            }
        }
    }
}

fn loc_view(loc: Option<Loc>) -> Markup {
    match loc {
        Some(l) => html! {
            div .grid{
                ins { "Files: " (l.get_files()) }
                ins { "LInes: "(l.get_lines()) }
            }
        },
        None => html! {},
    }
}

fn app_type_view(app_type: AppType) -> Markup {
    let text: String = match app_type {
        AppType::Server => "Server".to_string(),
        AppType::Service => "Service".to_string(),
        AppType::Web => "Web".to_string(),
        AppType::Job => "Job".to_string(),
        AppType::Cli => "Cli".to_string(),
        AppType::RichClient => "RichClient".to_string(),
        AppType::Internal => "Internal".to_string(),
    };
    html! {
            div {
                {(text)}
            }
    }
}
