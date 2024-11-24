use maud::html;
use rocket::response::content;




#[get("/exp")]
pub fn get() -> content::RawHtml<String> {
    let raw = html! {
               main class="p-8" {
                   section class="bg-gray-5 p-6 rounded mb-4" {
                       h2 class="text-xl font-semibold mb-4" { "Work Experience" }
                       ul class="list-disc list-inside" {
                           li {
                               strong { "Company A:" }
                               " Software Engineer (June 2020 - Present)"
                           }
                           li {
                               strong { "Company B:" }
                               " Frontend Developer (January 2019 - May 2020)"
                           }
                       }
                   }
                   section class="bg-gray-5 p-6 rounded mb-4" {
                       h2 class="text-xl font-semibold mb-4" { "Education" }
                       ul class="list-disc list-inside" {
                           li {
                               strong { "University of X:" }
                               " Bachelor of Science in Computer Science (2015 - 2019)"
                           }
                       }
                   }
                   section class="bg-gray-5 p-6 rounded mb-4" {
                       h2 class="text-xl font-semibold mb-4" { "Skills" }
                       ul class="list-disc list-inside" {
                           li { "HTML, CSS, JavaScript" }
                           li { "React, Angular, Vue" }
                           li { "Web Accessibility" }
                           li { "Responsive Design" }
                       }
                   }
               }
               footer class="p-6 bg-white text-center" {
                   p class="text-sm" {
                       "© 2023 Your Name. All rights reserved."
                   }
               }
                         
    }
    .into_string();
    content::RawHtml(raw)
}