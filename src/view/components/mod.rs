use maud::{html, Markup};

#[allow(dead_code)]
pub fn list_of(any_vec: Vec<Markup>) -> Markup {
    html! {
        div {
                @for m in &any_vec {
                            div { (m) }
                }
        }
    }
}

#[allow(dead_code)]
pub fn grid_of(any_vec: Vec<Markup>) -> Markup {
    html! {
        div .container {
            div class="grid"{
                @for m in &any_vec {
                    div { (m) }
                }
            }
        }
    }
}

pub fn progress_out_of_hundred(progress: i8) -> Markup {
    html! {
        progress value=(progress) max="100" {}
    }
}
