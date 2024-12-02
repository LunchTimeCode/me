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
                            footer {                                   (super::to_contact("Let's connect!"))                           
                            }}}.into_string();
    content::RawHtml(raw)
}
