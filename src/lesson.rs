use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    name: String,
    #[serde(rename = "type")]
    lesson_type: String,
    link: String,
    password: Option<String>,
    group: Option<String>,
    algorithms: Option<String>,
    combinatorics: Option<String>,
    start_m: u32,
    end_m: u32,
}

impl Lesson {
    pub fn print(&self) -> String {
        let pass = match &self.password {
            Some(pwd) => format!(" Пароль: {}", pwd),
            None => String::new(),
        };
        format!(
            "{:02}:{:02} \\- {:02}:{:02}\t[{}]({}) \\({}\\){}",
            self.start_m / 60,
            self.start_m % 60,
            self.end_m / 60,
            self.end_m % 60,
            self.name.replace("+", "\\+"),
            self.link,
            self.lesson_type,
            pass
        )
    }

    pub fn is_next<T: chrono::Timelike>(&self, time: &T) -> bool {
        let current_minutes = time.hour() * 60 + time.minute();
        current_minutes - 3 < self.start_m
    }
}

pub async fn get_day_timetable<'a, 'b>(
    day: &'a str,
    groups: crate::handlers::UserGroups<'b>,
) -> Result<Vec<Lesson>, reqwest::Error> {
    let lessons = reqwest::get(&format!("http://localhost:8000/timetable/{}", day))
        .await?
        .json::<Vec<Lesson>>()
        .await?
        .into_iter()
        .filter(|les| {
            les.group.is_none() || groups.group.is_none() || les.group.as_deref() == groups.group
        })
        .filter(|les| {
            les.algorithms.is_none()
                || groups.algorithms.is_none()
                || les.algorithms.as_deref() == groups.algorithms
        })
        .filter(|les| {
            les.combinatorics.is_none()
                || groups.combinatorics.is_none()
                || les.combinatorics.as_deref() == groups.combinatorics
        })
        .collect();
    Ok(lessons)
}

pub fn print_day(lessons: &[Lesson]) -> String {
    if lessons.is_empty() {
        return String::from("В этот день нет уроков");
    }
    lessons
        .iter()
        .map(|lesson| lesson.print())
        .collect::<Vec<String>>()
        .join("\n")
}
