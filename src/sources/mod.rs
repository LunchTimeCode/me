use crate::models::{AppType, Loc, Project, Skill};

pub fn get_skills() -> Vec<Skill> {
    vec![
        Skill::new(
            "Kotlin",
            95,
            "I am a kotlin developer, created and maintained many applications", 
            vec![
                Project::new(
                "Production Server",
                "Security and features",
                AppType::Server,
                "Server side application",
                None,
                Some(Loc::new(65598, 53073))
            ),
            Project::new(
                "Self service",
                "With in a team of two",
                AppType::Service,
                "Service to configure and manage business rules",
                None,
                Some(Loc::new(185, 12364))
            ),
            Project::new(
                "Desktop App",
                "with some java",
                AppType::RichClient,
                "Transfer Price Calculation Tool",
                None,
                Some(Loc::new(3563, 314638))
            ),
            Project::new(
                "Desktop App Plugin",
                "Client 1",
                AppType::RichClient,
                "Customized Code for a client",
                None,
                Some(Loc::new(128, 9870))
            ),
            Project::new(
                "Desktop App Plugin",
                "Client 2",
                AppType::RichClient,
                "Customized Code for a client",
                None,
                Some(Loc::new(138, 10317))
            ),
            ]
        ),
    Skill::new(
        "Rust",
        80,
        "I am a rustacean in my free time, more than three years of experience with the language and used it in professional settings for more than a year",
        vec![
            Project::new(
                "TypeFast",
                "created with egui",
                AppType::Web,
                "An applicatin to practice typing, without lags",
                Some("https://github.com/SilenLoc/TypeFast".to_string()),
                Some(Loc::new(19, 1505))
            ),
            Project::new(
                "Dreamy CLI",
                "create wit clap",
                AppType::Cli,
                "Tool to manage your dependencies",
                Some("https://github.com/LunchTimeCode/dreamy-cli".to_string()),
                Some(Loc::new(10, 745))
            )
        ]
    ),
    ]
}
