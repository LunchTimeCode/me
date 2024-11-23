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
                                    "With over 8 years of experience in software development, I specialize in building 
                                    scalable web applications and distributed systems. I'm passionate about clean code, 
                                    performance optimization, and creating exceptional user experiences."
                                }
                                p {
                                    "Currently working as a Senior Software Engineer at Tech Corp, where I lead the 
                                    development of mission-critical applications serving millions of users daily."
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
                                        span class="skill-tag" { "Python" }
                                        span class="skill-tag" { "JavaScript" }
                                        span class="skill-tag" { "TypeScript" }
                                    }
                                    // Frameworks & Tools
                                    h3 { "Frameworks & Tools" }
                                    div {
                                        span class="skill-tag" { "React" }
                                        span class="skill-tag" { "Node.js" }
                                        span class="skill-tag" { "Docker" }
                                        span class="skill-tag" { "Kubernetes" }
                                    }
                                    // Soft Skills
                                    h3 { "Soft Skills" }
                                    div {
                                        span class="skill-tag" { "Team Leadership" }
                                        span class="skill-tag" { "Project Management" }
                                        span class="skill-tag" { "Technical Writing" }
                                        span class="skill-tag" { "Mentoring" }
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
                                    li { "Contributing to open-source projects" }
                                    li { "Writing technical blog posts and documentation" }
                                    li { "Mentoring junior developers" }
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
                                    li { "Exploring WebAssembly and its applications" }
                                    li { "Building performant, accessible web applications" }
                                    li { "Sharing knowledge through technical writing and speaking" }
                                }
                            }
                            footer {
                                p {
                                    "Interested in collaboration? "
                                    a href="/" { "Let's connect!" }
                                }                            }                        }   }.into_string();
    content::RawHtml(raw)
}
