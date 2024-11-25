use crate::models::Skill;

pub fn get_skills() -> Vec<Skill> {
    vec![Skill::new("Kotlin", 95, "I am a kotlin developer, created and maintained many applications", vec![]),
    Skill::new("Rust", 80, "I am a rustacean in my free time, more than three years of experience with the language and used it in professional settings for more than a year", vec![]),
    ]
}
