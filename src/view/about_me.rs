use maud::html;
use rocket::response::content;

#[get("/about-me")]
pub fn get() -> content::RawHtml<String> {
    let raw = html!{
        article {
                            header {
                                h1 { "About Me" }
                            }
                            // Professional Summary
                            section {
                                h2 { "Professional Summary" }
                                p {
                                    "With over 4 years of experience in software development, I specialize in building 
                                    scalable web applications and integrated services. I'm passionate about good code, 
                                    and creating good user experiences."
                                }
                                p {
                                    "Currently working as a Software Engineer at Optravis LLC, where I work on the 
                                    development of applications serving users daily."
                                }
                            }
                            // Technical Skills
                            section {
                                h2 { "Technical Expertise" }
                                div {
                                    // Programming Languages
                                    h3 { "Programming Languages" }
                                    div {
                                        span class="skill-tag" { "Rust" }
                                        span class="skill-tag" { "Kotlin" }
                                        span class="skill-tag" { "Java" }
                                        span class="skill-tag" { "JavaScript" }
                                        span class="skill-tag" { "TypeScript" }
                                        span class="skill-tag" { "PHP" }
                                    }
                                    // Frameworks & Tools
                                    h3 { "Frameworks & Tools" }
                                    div {
                                        span class="skill-tag" { "Ktor" }
                                        span class="skill-tag" { "Rocket.rs" }
                                        span class="skill-tag" { "http4k" }
                                        span class="skill-tag" { "jooq" }
                                        span class="skill-tag" { "HTMX" }
                                        span class="skill-tag" { "WebComponents" }
                                        span class="skill-tag" { "React" }
                                        span class="skill-tag" { "Docker" }
                                        span class="skill-tag" { "Kubernetes" }
                                    }
                                    // Soft Skills
                                    h3 { "Soft Skills" }
                                    div {
                                        span class="skill-tag" { "Project Management" }
                                        span class="skill-tag" { "Technical Writing" }
                                    }
                                }
                            }
                            // Personal Interests
                            section class="interests" {
                                h2 { "Personal Interests" }
                                p {
                                    "Beyond coding, I'm passionate about:"
                                }
                                ul {
                                    li { "Writing open-source projects" }
                                    li { "Reading books" }
                                    li { "Exploring new technologies and frameworks" }
                                }
                            }                           // Current Focus
                            section {
                                h2 { "Current Focus" }
                                p {
                                    "I'm currently focused on:"
                                }
                                ul {
                                    li { "Deepening my expertise in Rust and systems programming" }
                                    li { "Building performant, accessible web applications" }
                                    li { "Sharing knowledge through speaking" }
                                }
                            }
                            footer {
                                p {
                                    "Interested in collaboration? "
                                    a href="/" { "Let's connect!" }
                                }                            }                        }   }.into_string();
    content::RawHtml(raw)
}
