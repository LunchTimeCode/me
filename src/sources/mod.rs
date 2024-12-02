use crate::models::{AppType, Language, Loc, Project};

pub fn get_projects() -> Vec<Project> {
    vec![
        Project::new(
            "Production Server",
            "Security and features",
            AppType::Server,
            Language::Kotlin,
            "Server side application",
            None,
            Some(Loc::new(65598, 53073)),
        ),
        Project::new(
            "Desktop App",
            "with some java",
            AppType::RichClient,
            Language::Kotlin,
            "Transfer Price Calculation Tool",
            None,
            Some(Loc::new(3563, 314638)),
        ),
        Project::new(
            "Desktop App Plugin",
            "Client 1",
            AppType::RichClient,
            Language::Kotlin,
            "Customized Code for a client",
            None,
            Some(Loc::new(128, 9870)),
        ),
        Project::new(
            "Desktop App Plugin",
            "Client 2",
            AppType::RichClient,
            Language::Kotlin,
            "Customized Code for a client",
            None,
            Some(Loc::new(138, 10317)),
        ),
        Project::new(
            "TypeFast",
            "created with egui",
            AppType::Web,
            Language::Rust,
            "An application to practice typing, without lags",
            Some("https://github.com/SilenLoc/TypeFast".to_string()),
            Some(Loc::new(19, 1505)),
        ),
        Project::new(
            "Dreamy CLI",
            "created wit clap",
            AppType::Cli,
            Language::Rust,
            "Tool to manage your dependencies",
            Some("https://github.com/LunchTimeCode/dreamy-cli".to_string()),
            Some(Loc::new(10, 745)),
        ),
    ]
}
