use maud::{html, Markup};
use rocket::response::content;

use crate::{
    models::{AppType, Loc, Project},
    sources::get_projects,
    view::components::list_of,
};

#[get("/projects")]
pub async fn get() -> content::RawHtml<String> {
    let comp: Vec<Markup> = get_projects()
        .iter()
        .map(|p| project_view(p.clone()))
        .collect();

    let raw = html! {
        article {
          header {
            h2 { "Projects" }
          }
        body{
               (list_of(comp))
         }
        }
    }
    .into_string();
    content::RawHtml(raw)
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
                p class="primary" { "Files: " (l.get_files()) }
                p { "Lines: "(l.get_lines()) }
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
