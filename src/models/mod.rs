#[derive(Debug, Clone)]
pub struct Skill {
    name: String,
    progress: i8,
    description: String,
    #[allow(dead_code)]
    projects: Vec<Project>,
}

impl Skill {
    pub fn new(name: &str, progress: i8, description: &str, projects: Vec<Project>) -> Skill {
        Skill {
            name: name.to_string(),
            progress,
            description: description.to_string(),
            projects,
        }
    }

    pub fn get_progress(&self) -> i8 {
        self.progress
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    name: String,
    app_type: AppType,
    description: String,
    url: String,
}

#[allow(dead_code)]
impl Project {
    pub fn new(name: &str, app_type: AppType, description: &str, url: &str) -> Self {
        Project {
            name: name.to_string(),
            app_type,
            description: description.to_string(),
            url: url.to_string(),
        }
    }

    pub fn only_with_app_type(app_type: AppType) -> Self {
        Project {
            name: "".to_string(),
            app_type,
            description: "".to_string(),
            url: "".to_string(),
        }
    }

    pub fn get_app_type(&self) -> AppType {
        self.app_type.clone()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AppType {
    Server,
    Service,
    Web,
    Job,
    Cli,
    RichClient,
    Internal,
}
