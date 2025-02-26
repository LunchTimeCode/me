use maud::{html, Markup, PreEscaped};

pub fn get() -> Markup {
    html! {

        (header())
        (splash())
        (content())
    }
}

pub fn header() -> Markup {
    html! {
        div."header" {
            div."home-menu pure-menu pure-menu-horizontal pure-menu-fixed" {
                a."pure-menu-heading" href="" {
                    "Silen Celeste Locatelli"
                }
                ul."pure-menu-list" {
                    li."pure-menu-item" {
                        a."pure-menu-link" href="https://github.com/SilenLoc" {
                            "GitHub"
                        }
                    }
                    li."pure-menu-item" {
                        a."pure-menu-link" href="https://www.linkedin.com/in/silen-locatelli/" {
                            "LinkedIn"
                        }
                    }
                    li."pure-menu-item" {
                        a."pure-menu-link" href="https://jobs.silenlocatelli.ch" {
                            "A Jobseeking tool"
                        }
                    }
                }
            }
        }
    }
}

fn splash() -> Markup {
    html! {
        div."splash-container" {
            div."splash" {
                h1."splash-head" {
                    "Welcome"
                }
                p."splash-subhead" {
                    "I am Silen Celeste Locatelli"
                }
                p {
                    a."pure-button" href="mailto:silen.locatelli@gmx.ch" {
                        "Write an email"
                    }
                }
            }
        }
    }
}

fn content() -> Markup {
    html! {
        div."content-wrapper" {
            div ."content" {
                h2."content-head is-center" {
                    "What I do"
                }
                div."pure-g" {
                    div."l-box pure-u-1 pure-u-md-1-2 pure-u-lg-1-4" {
                        h3."content-subhead" .flex-row {
                            (icon("code-bracket"))
                            "Software"
                        }
                        p {
                            "I write software since more than half a decade. I have experience in various languages and technologies."
                        }
                    }
                    div."l-box pure-u-1 pure-u-md-1-5 pure-u-lg-1-4" {
                        h3."content-subhead" .flex-row {
                            (icon("rocket-launch"))
                            "Handball"
                        }
                        p {
                            "I played handball for over 10 years. I was a goalkeeper and I am now playing on the field."
                        }
                    }

                    div."l-box pure-u-1 pure-u-md-1-2 pure-u-lg-1-4" {
                        h3."content-subhead" .flex-row {
                            (icon("book-open"))
                            "Books"
                        }
                        p {
                            "I read a lot of books. I am interested in non-fiction and fantasy."
                        }
                    }

                    div."l-box pure-u-1 pure-u-md-1-2 pure-u-lg-1-4" {
                        h3."content-subhead" .flex-row {
                            (icon("cog-8-tooth"))
                            "Kitchen"
                        }
                        p {
                            "I also cook from time to time. From known italian dishes to old indian curries"
                        }
                    }


                }
            }
            (ribbon())
            (end())
            (footer())
        }
    }
}

fn ribbon() -> Markup {
    html! {
        div."ribbon l-box-lrg pure-g" {
            div."l-box is-center pure-u-1 pure-u-md-3-5 pure-u-lg-2-5" {

            }
            div."pure-u-1 pure-u-md-4-5 pure-u-lg-3-5" {
                h2."content-head content-head-ribbon" {
                    "Projects"
                }

                p{
                    "My coding projects are on GitHub, refer to all my organizations and repositories there."
                }
                p {
                    a."pure-button" href="https://github.com/SilenLoc" {
                        "Go to my GitHub profile"
                    }
                }
            }
        }
    }
}

fn icon(name: &str) -> Markup {
    html! {
        div .hero-sm {
             (PreEscaped(static_hero_icons::twnf_solid(name)))
        }
    }
}

fn end() -> Markup {
    html! {
        div."content" {
            h2."content-head is-center" {
                "About this page"
            }


            div."pure-g" {
                div."l-box-lrg pure-u-1 pure-u-md-2-5" {
                    h4 {
                        "Built with"
                    }
                    p {
                        "Rust, Rocket, Maud and Pure CSS"
                    }
                    h4 {
                        "Deployed with"
                    }
                    p {
                        "Fedora Server, Docker Compose and Traefik"
                    }
                }

                div."l-box-lrg pure-u-1 pure-u-md-3-5" {
                    h4 {
                        "Running in"
                    }
                    p {
                        "Switzerland,  in my living room"
                    }
                    h4 {
                        "Running on"
                    }
                    p {
                        "An a Rasberry Pi 5"
                    }
                }
            }


        }
    }
}

fn footer() -> Markup {
    html! {
        div."footer l-box is-center" {
            "Made by me"
        }
    }
}
