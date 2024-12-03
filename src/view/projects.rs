use maud::{html, Markup, PreEscaped};
use rocket::response::content;

use crate::{
    charts::{self, ChartSize},
    models::{AppType, Language, Loc, Project},
    sources::get_projects,
    view::components::list_of,
};

#[get("/chart/<id>")]
pub async fn get_charts(id: &str) -> content::RawHtml<String> {
    let projects = get_projects();

    let size = ChartSize::new(400, 400);

    let c = match id {
        "lang" => lang_chart(projects.clone(), &size),
        "files" => files_chart(projects.clone(), &size),
        "lines" => line_chart(projects.clone(), &size),
        _ => lang_chart(projects.clone(), &size),
    };

    let raw = html! {
        div .main_chart {
            (PreEscaped(c))
        }
    }
    .into_string();
    content::RawHtml(raw)
}

#[get("/projects")]
pub async fn get() -> content::RawHtml<String> {
    let projects = get_projects();

    let comp: Vec<Markup> = projects.iter().map(|p| project_view(p.clone())).collect();

    let raw = html! {
        article {
          header {
            h2 { "Projects" }

            article {
                header {
                    nav{
                        ul{
                        li {  button class="outline"  hx-get="/chart/lang" hx-target="#chart-t" hx-trigger="click" { "Projects" } }
                        li {  button class="outline" hx-get="/chart/files" hx-target="#chart-t" hx-trigger="click" { "Files" } }
                        li {  button class="outline" hx-get="/chart/lines" hx-target="#chart-t" hx-trigger="click" { "Lines" } }
                        }
                    }
                }
                body{
                    div {
                        div id="chart-t" hx-get="/chart/lang" hx-trigger="load" {}
                }
                }}}


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

pub fn lang_chart(projects: Vec<Project>, size: &ChartSize) -> String {
    let rust = projects
        .iter()
        .filter(|p| p.get_language() == Language::Rust)
        .count() as f64;
    let kotlin = projects
        .iter()
        .filter(|p| p.get_language() == Language::Kotlin)
        .count() as f64;

    let data = vec![(rust, "Rust".to_string()), (kotlin, "Kotlin".to_string())];

    charts::create("Projects by Language", data, size)
}

pub fn line_chart(projects: Vec<Project>, size: &ChartSize) -> String {
    let rust: i32 = projects
        .iter()
        .filter(|p| p.get_language() == Language::Rust)
        .filter_map(|p| p.get_loc().map(|l| l.get_lines()))
        .sum();

    let kotlin: i32 = projects
        .iter()
        .filter(|p| p.get_language() == Language::Kotlin)
        .filter_map(|p| p.get_loc().map(|l| l.get_lines()))
        .sum();

    let data = vec![
        (rust as f64, "Rust".to_string()),
        (kotlin as f64, "Kotlin".to_string()),
    ];

    charts::create("Lines of code by Language", data, size)
}

pub fn files_chart(projects: Vec<Project>, size: &ChartSize) -> String {
    let rust: i32 = projects
        .iter()
        .filter(|p| p.get_language() == Language::Rust)
        .filter_map(|p| p.get_loc().map(|l| l.get_files()))
        .sum();

    let kotlin: i32 = projects
        .iter()
        .filter(|p| p.get_language() == Language::Kotlin)
        .filter_map(|p| p.get_loc().map(|l| l.get_files()))
        .sum();

    let data = vec![
        (rust as f64, "Rust".to_string()),
        (kotlin as f64, "Kotlin".to_string()),
    ];

    charts::create("Files by Language", data, size)
}
