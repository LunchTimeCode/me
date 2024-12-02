use crate::models::{AppType, Loc, Project};

pub fn get_projects() -> Vec<Project> {
    vec![
        Project::new(
            "Production Server",
            "Security and features",
            AppType::Server,
            "Server side application",
            None,
            Some(Loc::new(65598, 53073)),
        ),
        Project::new(
            "Desktop App",
            "with some java",
            AppType::RichClient,
            "Transfer Price Calculation Tool",
            None,
            Some(Loc::new(3563, 314638)),
        ),
        Project::new(
            "Desktop App Plugin",
            "Client 1",
            AppType::RichClient,
            "Customized Code for a client",
            None,
            Some(Loc::new(128, 9870)),
        ),
        Project::new(
            "Desktop App Plugin",
            "Client 2",
            AppType::RichClient,
            "Customized Code for a client",
            None,
            Some(Loc::new(138, 10317)),
        ),
        Project::new(
            "TypeFast",
            "created with egui",
            AppType::Web,
            "An application to practice typing, without lags",
            Some("https://github.com/SilenLoc/TypeFast".to_string()),
            Some(Loc::new(19, 1505)),
        ),
        Project::new(
            "Dreamy CLI",
            "created wit clap",
            AppType::Cli,
            "Tool to manage your dependencies",
            Some("https://github.com/LunchTimeCode/dreamy-cli".to_string()),
            Some(Loc::new(10, 745)),
        ),
    ]
}
