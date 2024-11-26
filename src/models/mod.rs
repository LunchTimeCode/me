#[derive(Debug, Clone)]
pub struct Skill {
    name: String,
    progress: i8,
    description: String,
    #[allow(dead_code)]
    projects: Vec<Project>,
}

impl Skill {
    pub fn new(name: &str, progress: i8, description: &str, projects: Vec<Project>) -> Self {
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

    pub fn get_projects(&self) -> Vec<Project> {
        self.projects.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    name: String,
    small_description: String,
    app_type: AppType,
    description: String,
    url: Option<String>,
    loc: Option<Loc>,
}

#[allow(dead_code)]
impl Project {
    pub fn new(
        name: &str,
        small_description: &str,
        app_type: AppType,
        description: &str,
        url: Option<String>,
        loc: Option<Loc>,
    ) -> Self {
        Project {
            name: name.to_string(),
            small_description: small_description.to_string(),
            app_type,
            description: description.to_string(),
            url,
            loc,
        }
    }

    pub fn get_app_type(&self) -> AppType {
        self.app_type.clone()
    }

    pub fn get_small_description(&self) -> &str {
        &self.small_description
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn get_loc(&self) -> Option<Loc> {
        self.loc.clone()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppType {
    Server,
    Service,
    Web,
    Job,
    Cli,
    RichClient,
    Internal,
}

#[derive(Debug, Clone)]
pub struct Loc {
    files: i32,
    lines: i32,
}

impl Loc {
    pub fn new(files: i32, lines: i32) -> Self {
        Loc { files, lines }
    }

    pub fn get_files(&self) -> i32 {
        self.files
    }

    pub fn get_lines(&self) -> i32 {
        self.lines
    }
}
